//! A SerumCV backend for macOS and general Apple devices.

mod authorization;

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

    type SourceInput = String;

    #[inline]
    fn list_connected_devices() -> Vec<Self::SourceInput> {
        todo!()
    }

    #[inline]
    fn backend_type() -> super::BackendType {
        todo!()
    }
}

/// A descriptor for an AvFoundation device.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AvDescriptor {
    /// This device's unique identifier.
    pub device_identifier: AvSource,
    /// Manufacturer-provided model information.
    pub device_model: String,
}

impl Descriptor for AvDescriptor {
    /// Veeeery unique. See `[AvSource]` for information.
    type IdentiferTy = AvSource;
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

pub type AvDevice = (); // note: is this the same as AvStream under AVFoundation?
pub type AvBuffer<'appl_internal_buf> = &'appl_internal_buf [u8];
pub type AvStream = ();

/// Since a `Source` must be unique on the system, we use a device's `uniqueID`
/// from AVFoundation. This is just a string.
///
/// We don't expect users to know it and should warn them if they try to
/// instantiate a device using the `CaptureDevice::new()` associated function.
pub type AvSource = String;

// TODO: make this actual image metadata somehow.
pub type AvMetadata = String;

/// A capture device from AVFoundation.
///
/// Note that AVFoundation does not use paths to describe these devices, and
/// finding them as a user is difficult. As such, avoid using the `new()`
/// function to create this object.
///
/// Instead, get a list of identifiers from the `AvBackend`, or use the
/// `new_first()` function if you only need one device.
pub type AvVideoCaptureDevice = VideoCapture<AvDescriptor, AvDevice, AvSource, AvStream>;

impl Connection<'_, AvSource> for AvVideoCaptureDevice {
    type Source = AvSource;

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
