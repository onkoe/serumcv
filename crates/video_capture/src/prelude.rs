//! The useful traits and types from the `serumcv_video_capture` crate.

pub use super::backends::Backend;
pub use super::config::{
    Format, Framerate, FramerateConsts, ResolutionSetting, SpecificResolution,
    VideoCaptureConfiguration, VideoCaptureImageConfiguration, VideoCaptureProperty,
};
pub use super::error::{
    VideoCaptureConfigError, VideoCaptureConnectionError, VideoCaptureUsageError,
};
pub use super::{VideoCaptureConnection, VideoCaptureDescriptor, VideoCaptureStream};
