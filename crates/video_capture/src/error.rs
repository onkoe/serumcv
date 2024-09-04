//! Errors for video capture devices and their surrounding operations.

use core::error::Error;
use pisserror::Error;

use crate::config::{Format, ResolutionSetting, VideoCaptureImageConfiguration};

/// An error that occurs when attempting to first access a system video capture
/// device.
///
/// Note that this is non-exhaustive to account for possible new backends and
/// features.
#[derive(Clone, Debug, Error, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VideoCaptureConnectionError {
    #[error("The given source (`{source}`) does not exist or does not contain a capture device.")]
    SourceDoesntExist { source: String },

    /// Only use this variant with the `new_first` or similar methods.
    #[error("The backend did not find any video capture devices.")]
    NoCaptureDevices,

    #[error("The capture device with source `{source}` is busy. I/O error: `{err_msg}`")]
    CaptureDeviceBusy { source: String, err_msg: String },

    /// Display this when we couldn't ask the device to stop.
    #[error("Failed to stop the device gracefully. Source: `{source}`, error: `{err_msg}`")]
    StopError { source: String, err_msg: String },

    /// When attempting to reconnect, we can display this when the device is
    /// connected already.
    #[error("This capture device is already connected to the physical device at `{source}`.")]
    AlreadyConnected { source: String },

    /// For a failure when getting the capture device's info.
    #[error("The capture device at `{source}` failed to return its device info. See: `{err_msg}`")]
    CouldntGetDeviceInfo { source: String, err_msg: String },

    #[error("Reconnection error: the device at `{source}` does not share a serial number with the original device.\
    Original: `{original}`. Now: `{now}`.")]
    ReconnectionSerialMismatch {
        source: String,
        original: String,
        now: String,
    },

    #[error("Reconnection error: the device at `{source}` does not share a model number with the original device.\
    Original: `{original}`. Now: `{now}`.")]
    ReconnectionModelMismatch {
        source: String,
        original: String,
        now: String,
    },

    /// This means that the device gave an error when we tried to first read from it.
    #[error("Failed to warm up device at `{source}`. See: `{err_msg}`")]
    WarmUpFailed { source: String, err_msg: String },
}

/// An error that occurs when we fail to read from a capture device.
#[derive(Clone, Debug, Error, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VideoCaptureUsageError {
    #[error("IO error when attempting to access data from device at `{source}`: `{err_msg}`")]
    IoError { source: String, err_msg: String },
}

/// An error that occurs when configuring a video capture device.
#[derive(Clone, Debug, Error, PartialEq, PartialOrd)]
#[non_exhaustive]
#[rustfmt::skip]
pub enum VideoCaptureConfigError {
    #[error("The capture device at `{source}` does not appear to contain the requested property, `{property_name}`.")]
    PropertyNotFound {
        source: String,
        property_name: String,
    },

    #[error("Failed to write property to device with source `{source}`. See: `{err_msg}`")]
    PropertyWriteFailure { source: String, err_msg: String },

    #[error("The capture device at `{source}` cannot use the given image configuration: {image_conf:?}")]
    UnsupportedImageConfiguration {
        source: String,
        image_conf: VideoCaptureImageConfiguration,
    },

    #[error("The capture device at `{source}` did not respond to the request for format properties. See: `{err_msg}`")]
    CouldntGetFormat {
        source: String,
        err_msg: String,
    },


    #[error("The capture device at `{source}` cannot use the given resolution, `{resolution}`, with the given image configuration.")]
    IncompatibleResolution {
        source: String,
        resolution: ResolutionSetting,
        format: Format,
    },

    #[error("The capture device at `{source}` failed to list any configurations.")]
    DeviceDoesntListConfigurations {
        source: String,
        err_msg: String,
    }
}
