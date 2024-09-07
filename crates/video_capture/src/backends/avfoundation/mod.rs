//! A SerumCV backend for macOS and general Apple devices.

// TODO: remove this
#![expect(clippy::todo)]

mod authorization;

use cidre::{
    arc::{Retained, ReturnedAutoReleased},
    av::{
        capture::{session::Session as CidreSession, VideoDataOutput},
        CaptureDevice as CidreCaptureDevice, CaptureDeviceDiscoverySession, CaptureDeviceInput,
        CaptureDevicePos, CaptureDeviceType as DeviceType, MediaType,
    },
    ns::Array,
};

use crate::prelude::internal_prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AvBackend;

impl<'src, 'conn> Backend<'src, 'conn> for AvBackend
where
    'src: 'conn,
{
    type Descriptor = AvDescriptor;

    type Device = AvVideoCaptureDevice
    where
        Self::Source: 'src;

    type Source = AvSource;

    type SourceInput = AvDescriptor;

    #[inline]
    fn list_connected_devices() -> Vec<Self::SourceInput> {
        let discovery = Self::discovery();
        let devices = discovery.devices();

        devices
            .iter()
            .map(|device| AvDescriptor {
                device_identifier: device.unique_id().to_string(),
                device_model: device.localized_name().to_string(),
            })
            .collect::<Vec<_>>()
    }

    #[inline]
    fn backend_type() -> super::BackendType {
        super::BackendType::AvFoundation
    }
}

impl AvBackend {
    pub(super) fn discovery() -> ReturnedAutoReleased<CaptureDeviceDiscoverySession> {
        let ar = Array::from_slice(&[
            DeviceType::external(),
            DeviceType::built_in_dual_wide_camera(),
            DeviceType::built_in_telephoto_camera(),
            DeviceType::built_in_true_depth_camera(),
            DeviceType::built_in_wide_angle_camera(),
            DeviceType::continuity_camera(),
            DeviceType::desk_view_camera(),
        ])
        .autoreleased();

        // let devices = av::capture::input::DeviceInput(av::capture::input::Input())

        CaptureDeviceDiscoverySession::with_device_types_media_and_pos_ar(
            ar,
            Some(MediaType::video()),
            CaptureDevicePos::Unspecified,
        )
    }
}

/// A descriptor for an AvFoundation device.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AvDescriptor {
    /// This device's unique identifier.
    pub device_identifier: String,
    /// Manufacturer-provided model information.
    pub device_model: String,
}

impl Descriptor for AvDescriptor {
    /// Veeeery unique. See `[AvSource]` for information.
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

pub type AvDevice = Retained<CidreCaptureDevice>;
pub type AvBuffer<'appl_internal_buf> = &'appl_internal_buf [u8];
pub type AvStream = Retained<CidreSession>;

/// Since a `Source` must be unique on the system, we use a device's `uniqueID`
/// from AVFoundation. This is just a string.
pub(crate) type AvSource = ();

// TODO: make this actual image metadata somehow.
pub type AvMetadata = String;

/// A capture device from AVFoundation.
///
/// Note that AVFoundation does not use paths to describe these devices, and
/// finding them as a user is difficult.
///
/// Make sure to use the `AvBackend::list_available_devices` method to make an
/// AvSource.
pub type AvVideoCaptureDevice = VideoCapture<AvDescriptor, AvDevice, AvSource, AvStream>;

impl Connection<'_, AvSource> for AvVideoCaptureDevice {
    type Source = AvDescriptor;

    #[inline]
    fn new(source: Self::Source) -> Result<Self, ConnectionError> {
        todo!()
    }

    #[inline]
    fn new_first() -> Result<Self, ConnectionError> {
        todo!()
    }

    #[inline]
    fn disconnect(&mut self) -> Result<(), ConnectionError> {
        todo!()
    }

    #[inline]
    fn reconnect(&mut self) -> Result<(), ConnectionError> {
        todo!()
    }
}

impl<'src, 'conn> Stream<'src, 'conn, AvSource> for AvVideoCaptureDevice
where
    'src: 'conn,
{
    // note: does the apple internal buffer *actually* live for 'conn???
    // unless it's our own buffer. idk yet. we'll see
    type Buffer = AvBuffer<'conn>;

    type Source = AvSource;

    // not good... there is no nice input.
    //
    // TODO: consider making this some weird type (like never, `!`?) to force
    // users to avoid it..?
    type SourceInput = AvSource;

    type Metadata = AvMetadata;

    #[inline]
    fn read_frame<'func>(
        &'func mut self,
    ) -> Result<(&'func [u8], &'func Self::Metadata), crate::UsageError>
    where
        'src: 'func,
    {
        todo!()
    }
}

impl Configuration for AvVideoCaptureDevice {
    #[inline]
    fn supported_image_configurations(&self) -> Result<Vec<ImageConfiguration>, ConfigError> {
        todo!()
    }

    #[inline]
    fn image_configuration(&self) -> Result<ImageConfiguration, ConfigError> {
        todo!()
    }

    #[inline]
    fn set_image_configuration(
        &self,
        conf: &ImageConfiguration,
    ) -> Result<ImageConfiguration, ConfigError> {
        todo!()
    }
}
