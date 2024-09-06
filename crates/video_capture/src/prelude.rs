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

#[expect(unused)]
#[expect(unreachable_pub)]
pub(crate) mod internal_prelude {
    pub use crate::config::{Format, SpecificResolution};
    pub use crate::{
        backends::Backend,
        config::{
            VideoCaptureConfiguration as Configuration,
            VideoCaptureImageConfiguration as ImageConfiguration,
        },
        error::VideoCaptureConfigError as ConfigError,
        error::VideoCaptureConnectionError as ConnectionError,
        error::VideoCaptureUsageError as UsageError,
        VideoCapture, VideoCaptureConnection as Connection, VideoCaptureDescriptor as Descriptor,
        VideoCaptureStream as Stream,
    };
}
