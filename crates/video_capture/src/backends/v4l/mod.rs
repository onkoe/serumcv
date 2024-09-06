//! A Video4Linux 2 capture device backend.

extern crate alloc;

use alloc::borrow::Cow;
use device_info::MediaDeviceInfo;
use fraction::{Fraction, One};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Duration;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Output;
use v4l::{buffer::Type, io::traits::Stream as _};

// TODO: move to internal prelude when back on linux
use crate::config::{Format, SpecificResolution};
use crate::{
    config::{VideoCaptureConfiguration, VideoCaptureImageConfiguration as ImageConfiguration},
    error::VideoCaptureConfigError as ConfigError,
    error::VideoCaptureUsageError as UsageError,
    ConnectionError, VideoCapture, VideoCaptureConnection, VideoCaptureDescriptor,
    VideoCaptureStream,
};

pub use source::V4LSource;

use super::Backend;

mod device_info;
mod framerate;
mod source;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct V4LBackend;

impl<'path, 'conn> Backend<'path, 'conn> for V4LBackend
where
    'path: 'conn,
{
    type Descriptor = V4LVideoCaptureDescriptor;
    type Device = V4LVideoCaptureDevice<'path, 'conn>;
    type Source = source::V4LSource;
    type SourceInput = PathBuf;

    #[inline]
    fn list_connected_devices() -> Vec<Self::SourceInput> {
        // ask v4l for the connected devices, then grab all their paths.
        let mut devices: Vec<_> = v4l::context::enum_devices()
            .iter()
            .map(|node| node.path().to_owned())
            .collect();

        // let's also remove any duplicates.
        //
        // FIXME: this could be a set type. but i kinda want to avoid hashing
        // or even sorting all the paths each check if we can. maybe try some
        // library? alternatively, some stateful object that always maintains
        // a set and mutates it if necessary (some kind of static?)
        //
        // (optimization could be important - i can imagine folks calling this
        // each frame.)
        devices.sort();
        devices.dedup();

        devices
    }

    #[inline]
    fn backend_type() -> super::BackendType {
        super::BackendType::V4L2
    }
}

/// A descriptor for a Video4Linux video capture device.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4LVideoCaptureDescriptor {
    /// This device's unique identifier
    pub device_identifier: String,
    /// Model number/etc.
    pub device_model: String,
}

impl VideoCaptureDescriptor for V4LVideoCaptureDescriptor {
    type IdentiferTy = String;
    type ModelTy = String;

    #[inline]
    fn device_identifier(&self) -> Self::IdentiferTy {
        self.device_identifier.clone()
    }

    #[inline]
    fn device_model(&self) -> Self::ModelTy {
        self.device_model.clone()
    }
}

/// The source for a Video4Linux device. These are always just paths
/// (at least within this library).
pub type V4LBuffer = UserptrStream;

/// A capture device using the Video4Linux backend.
pub type V4LVideoCaptureDevice<'path, 'conn> =
    VideoCapture<V4LVideoCaptureDescriptor, v4l::Device, V4LSource, V4LBuffer>;

impl V4LVideoCaptureDevice<'_, '_> {
    fn source_as_string(&self) -> String {
        self.source.user_source_string()
    }
}

impl<'path> VideoCaptureConnection<'path, V4LSource> for V4LVideoCaptureDevice<'path, '_> {
    type Source = PathBuf;

    #[inline]
    fn new(source: Self::Source) -> Result<Self, ConnectionError> {
        let path_string = Cow::from(source.to_string_lossy().to_string());
        tracing::debug!("creating a new Video4Linux capture device at path `{path_string}`...",);

        // compute the necessary paths
        tracing::trace!("getting media + video source...");
        let checked_source = V4LSource::new(&source)?;
        tracing::trace!("made the sources for V4L device! see: `{checked_source:?}`");

        // grab device info
        tracing::trace!("getting media device info...");
        let device_info =
            MediaDeviceInfo::get(&source).map_err(|e| ConnectionError::CouldntGetDeviceInfo {
                source: path_string.to_string(),
                err_msg: e.to_string(),
            })?;

        let (device_identifier, device_model) = (device_info.serial(), device_info.model());
        tracing::trace!("media device info obtained!");

        // make info into a descriptor
        let descriptor = V4LVideoCaptureDescriptor {
            device_identifier,
            device_model,
        };

        // attempt to access the device by path
        // TODO: hey, check the fs error if it doesn't exist or the camera
        // just failed to connect.

        tracing::trace!("creating device...");
        let device = Device::with_path(&source).map_err(|e| {
            // check if the file exists
            match e.kind() {
                ErrorKind::NotFound => ConnectionError::SourceDoesntExist {
                    source: path_string.to_string(),
                },
                err_kind => ConnectionError::OddIOError {
                    source: source.display().to_string(),
                    err_kind,
                    err_msg: e.to_string(),
                },
            }
        })?;
        tracing::trace!("device created!");

        tracing::trace!("starting stream...");
        let mut stream = V4LBuffer::new(&device, Type::VideoCapture).map_err(|e| {
            // check if the file exists
            match e.kind() {
                ErrorKind::NotFound => ConnectionError::SourceDoesntExist {
                    source: path_string.to_string(),
                },
                err => ConnectionError::OddIOError {
                    source: source.display().to_string(),
                    err_kind: err,
                    err_msg: e.to_string(),
                },
            }
        })?;
        tracing::trace!("stream started!");

        // unused dummy frame to make buggy drivers fill info about the
        // device's capabilities
        stream.next().map_err(|e| ConnectionError::WarmUpFailed {
            source: path_string.to_string(),
            err_msg: e.to_string(),
        })?;

        Ok(Self {
            descriptor,
            device,
            source: checked_source,
            stream,
        })
    }

    #[inline]
    fn new_first() -> Result<Self, ConnectionError> {
        // try to find any device on the system
        let Some(device_path) = V4LBackend::list_connected_devices().first().cloned() else {
            return Err(ConnectionError::NoCaptureDevices);
        };

        Self::new(device_path)
    }

    #[inline]
    fn disconnect(&mut self) -> Result<(), ConnectionError> {
        self.stream.stop().map_err(|e| ConnectionError::StopError {
            source: self.source_as_string(),
            err_msg: e.to_string(),
        })
    }

    #[inline]
    fn reconnect(&mut self) -> Result<(), ConnectionError> {
        // check if we already have a valid stream
        if self.source.video.exists() && self.source.media.exists() {
            return Err(ConnectionError::AlreadyConnected {
                source: self.source_as_string(),
            });
        }

        // grab device info
        let device_info = MediaDeviceInfo::get(&self.source.media).map_err(|e| {
            ConnectionError::CouldntGetDeviceInfo {
                source: self.source_as_string(),
                err_msg: e.to_string(),
            }
        })?;
        let (device_identifier, device_model) = (device_info.serial(), device_info.model());

        // see if the device model changed
        if device_model != self.descriptor.device_model {
            return Err(ConnectionError::ReconnectionModelMismatch {
                source: self.source_as_string(),
                original: self.descriptor.device_model(),
                now: device_model,
            });
        }

        // see if the device serial changed
        if device_identifier != self.descriptor.device_identifier {
            return Err(ConnectionError::ReconnectionSerialMismatch {
                source: self.source_as_string(),
                original: self.descriptor.device_identifier(),
                now: device_identifier,
            });
        }

        // attempt to access the device by path
        #[allow(clippy::map_err_ignore)]
        // TODO: hey, check the fs error if it doesn't exist or the camera
        // just failed to connect.
        let device = Device::with_path(&self.source.video).map_err(|_| {
            ConnectionError::SourceDoesntExist {
                source: self.source_as_string(),
            }
        })?;

        // make a stream connected to the device
        // FIXME: this creates a buffer that the user didn't ask for!
        let mut stream = MmapStream::with_buffers(&device, Type::VideoCapture, 4).map_err(|e| {
            ConnectionError::CaptureDeviceBusy {
                source: self.source_as_string(),
                err_msg: e.to_string(),
            }
        })?;

        // this performs warm-up or something...
        // TODO: look at fr v4l docs to see what that means lol
        stream.next().map_err(|e| ConnectionError::WarmUpFailed {
            source: self.source_as_string(),
            err_msg: e.to_string(),
        })?;

        Ok(())
    }
}

impl<'path, 'conn> VideoCaptureStream<'path, 'conn, V4LSource>
    for V4LVideoCaptureDevice<'path, 'conn>
where
    'path: 'conn,
{
    // FIXME: this isn't actually a buffer. it contains one!
    // consider swapping to some other construct..?
    type Buffer = MmapStream<'conn>;
    type Source = V4LSource;
    type SourceInput = &'path Path;
    type Metadata = v4l::buffer::Metadata;

    #[inline]
    fn read_frame<'func>(
        &'func mut self,
    ) -> Result<(&'func [u8], &'func Self::Metadata), UsageError>
    where
        'path: 'func,
    {
        self.stream.next().map_err(|e| UsageError::IoError {
            source: self.source.user_source_string(),
            err_msg: e.to_string(),
        })
    }
}

impl VideoCaptureConfiguration for V4LVideoCaptureDevice<'_, '_> {
    #[inline]
    fn supported_image_configurations(&self) -> Result<Vec<ImageConfiguration>, ConfigError> {
        let no_cfgs_err = |e: std::io::Error| ConfigError::DeviceDoesntListConfigurations {
            source: self.source_as_string(),
            err_msg: e.to_string(),
        };

        // a list to store the supported fmts
        let mut supported_formats = Vec::new();

        // ask device for supported formats
        let formats = self.device.enum_formats().map_err(no_cfgs_err)?;

        for format in formats {
            let fourcc = format.fourcc;
            let format_rs = Format::new(fourcc.repr);

            let resolutions = self
                .device
                .enum_framesizes(format.fourcc)
                .map_err(no_cfgs_err)?;

            for resolution in resolutions {
                // check the available framerates for this framesize
                let discrete_resolution = match resolution.size {
                    v4l::framesize::FrameSizeEnum::Discrete(res) => res,
                    v4l::framesize::FrameSizeEnum::Stepwise(framesize) => {
                        tracing::warn!("Device at source `{}` returned a stepwise resolution (`{framesize}`). These aren't currently supported.", self.source_as_string());
                        continue;
                    }
                };

                let frame_intervals = self
                    .device
                    .enum_frameintervals(
                        fourcc,
                        discrete_resolution.width,
                        discrete_resolution.height,
                    )
                    .map_err(no_cfgs_err)?;

                for frame_interval in frame_intervals {
                    let interval = match frame_interval.interval {
                        v4l::frameinterval::FrameIntervalEnum::Discrete(frac) => frac,
                        v4l::frameinterval::FrameIntervalEnum::Stepwise(rate) => {
                            tracing::warn!("Device at source `{}` returned a stepwise framerate (`{rate}`). These aren't currently supported.", self.source_as_string());
                            continue;
                        }
                    };

                    // compute the frame rate (a frame rate is 1 / frame_interval)
                    let interval_frac = Fraction::new(interval.numerator, interval.denominator);
                    let rate = Fraction::one() / interval_frac;

                    supported_formats.push(ImageConfiguration {
                        format: format_rs,
                        resolution: SpecificResolution {
                            width: discrete_resolution.width,
                            height: discrete_resolution.height,
                        },
                        framerate: rate,
                    });
                }
            }
        }

        Ok(supported_formats)
    }

    #[inline]
    fn image_configuration(&self) -> Result<ImageConfiguration, ConfigError> {
        let format = self
            .device
            .format()
            .map_err(|e| ConfigError::CouldntGetFormat {
                source: self.source_as_string(),
                err_msg: e.to_string(),
            })?;

        let fd = self.device.handle().fd();
        let v4l2_stream_parm = framerate::V4l2StreamParm::new(self.source_as_string(), fd)?;
        let framerate = v4l2_stream_parm.get_frame_rate();

        Ok(ImageConfiguration {
            format: Format::new(format.fourcc.repr),
            resolution: SpecificResolution::new(format.width, format.height),
            framerate,
        })
    }

    #[inline]
    #[must_use = "The capture device may have used another image configuration
    that does not match the input. Consider checking the output config before continuing."]
    fn set_image_configuration(
        &self,
        conf: &ImageConfiguration,
    ) -> Result<ImageConfiguration, ConfigError> {
        // make a v4l format from the given img conf
        let expected = v4l::Format::new(
            conf.resolution.width,
            conf.resolution.height,
            v4l::FourCC {
                repr: conf.format.array(),
            },
        );

        // send it to the device and get back the info we wanted
        let actual =
            self.device
                .set_format(&expected)
                .map_err(|e| ConfigError::PropertyWriteFailure {
                    source: self.source_as_string(),
                    err_msg: format!("Failed to change image configuration. IO Error {e}"),
                })?;

        // let's also check the framerate. we gotta do it manually, unfortunately
        let framerate =
            framerate::V4l2StreamParm::new(self.source_as_string(), self.device.handle().fd())?
                .get_frame_rate();

        // create a img conf from all that info
        let actual_conf = ImageConfiguration {
            format: actual.into(),
            resolution: actual.into(),
            framerate,
        };

        // compare them and tell user if they're not the same.
        //
        // note that this isn't an error. the trait accounts for the mismatch by returning it.
        if conf != &actual_conf {
            tracing::warn!(
                "Device at source `{}` has format mismatch.\n
                    - Expected: `{}`\n
                    - Got: `{}`",
                self.source_as_string(),
                expected,
                actual
            );
        }

        Ok(actual_conf)
    }
}
