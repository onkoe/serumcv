use crate::error::VideoCaptureConfigError as ConfigError;

pub trait VideoCaptureProperties {
    /// Lists the properties available on this device.
    ///
    /// For more information about each `Property`, consult the documentation
    /// for the active `Backend`.
    fn properties(&self) -> Vec<Property>;

    /// Attempts to get a property from the video capture device's
    /// configuration.
    ///
    /// This may return `None` if the property is not found or the device is
    /// disconnected. (TODO: Result if the device isn't connected?)
    fn property(&self, key: PropertyKeyType) -> Option<Property>;

    /// Attempts to set a property.
    ///
    /// # Errors
    ///
    /// This can return an error
    fn set_property(&mut self) -> Result<(), ConfigError>;
}

type PropertyKeyType = String;
type PropertyValueType = String;

type Property = VideoCaptureProperty;

/// One of many adjustable properties that a video capture device has.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VideoCaptureProperty {
    key: PropertyKeyType,
    value: PropertyValueType,
}

impl Property {
    /// Creates a new `Property`.
    #[inline]
    pub fn new<Val: Into<PropertyValueType>>(key: PropertyKeyType, val: Val) -> Self {
        Self {
            key,
            value: val.into(),
        }
    }

    #[inline]
    pub fn key(&self) -> PropertyKeyType {
        self.key.clone()
    }

    #[inline]
    pub fn value(&self) -> PropertyValueType {
        self.value.clone()
    }
}
