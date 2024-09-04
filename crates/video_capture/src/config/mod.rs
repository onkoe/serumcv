mod format;
mod framerate;
mod properties;
mod resolution;

pub use format::Format;

use crate::error::VideoCaptureConfigError as ConfigError;

// re-exports
pub use framerate::{Framerate, FramerateConsts};
pub use properties::{VideoCaptureProperties, VideoCaptureProperty};
pub use resolution::{ResolutionSetting, SpecificResolution};

/// Methods to configure a video capture device.
pub trait VideoCaptureConfiguration {
    /// Makes a list of ALL supported image configurations.
    ///
    /// # Errors
    ///
    /// Fails when the device is disconnected or is in use by another program.
    fn supported_image_configurations(&self) -> Result<Vec<ImageConfiguration>, ConfigError>;

    /// Gets the capture device's image configuration.
    ///
    /// # Errors
    ///
    /// This can fail if the device isn't connected, is being used by another
    /// program, or doesn't support the given configuration.
    fn image_configuration(&self) -> Result<ImageConfiguration, ConfigError>;

    /// Sets the device's image configuration given the input. It will then
    /// return the format the device is now using afterwards.
    ///
    /// # Errors
    ///
    /// This can fail if the device isn't connected, is being used by another
    /// program, or doesn't support the given configuration.
    #[must_use = "The capture device may have used another image configuration
    that does not match the input. Consider checking the output config before continuing."]
    fn set_image_configuration(
        &self,
        conf: &ImageConfiguration,
    ) -> Result<ImageConfiguration, ConfigError>;
}

type ImageConfiguration = VideoCaptureImageConfiguration;

/// A small bundle of the essential image properties: format, resolution,
/// and frame-rate.
///
/// The fields of this struct are all public. Create a new img. conf. using
/// manual struct construction syntax.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VideoCaptureImageConfiguration {
    pub format: Format,
    pub resolution: SpecificResolution,
    pub framerate: Framerate,
}

// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
// pub struct VideoCaptureConfig<Props: Properties> {
//     resolution: ResolutionSetting,
//     format: Format,
//     properties: Props,
// }
