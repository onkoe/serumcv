use core::fmt::Display;

/// A video capture device's resolution setting.
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ResolutionSetting {
    /// Will use the highest possible resolution for the device.
    Highest,
    /// Attempts to use this exact resolution.
    Custom(SpecificResolution),
    /// Finds the closest possible resolution for the device and uses that.
    Closest(SpecificResolution),
    /// Uses the lowest available resolution for the device.
    Lowest,
}

impl Display for ResolutionSetting {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Highest => f.write_str("ResolutionSetting::Highest"),
            Self::Custom(ref custom) => {
                f.write_fmt(format_args!("ResolutionSetting::Custom({custom})"))
            }
            Self::Closest(ref custom) => {
                f.write_fmt(format_args!("ResolutionSetting::Closest({custom})"))
            }
            Self::Lowest => f.write_str("ResolutionSetting::Lowest"),
        }
    }
}

/// A resolution with specified values.
///
/// These can be used to give resolutions to capture devices, but the
/// capture device may also return its current resolution with this structure.
///
/// Consider using one of the common constants instead of constructing a
/// `SpecificResolution` manually.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SpecificResolution {
    pub width: u32,
    pub height: u32,
}

impl SpecificResolution {
    pub const RES_16X9_4320P: Self = Self::new(7680, 4320);
    pub const RES_16X9_2880P: Self = Self::new(5120, 2880);
    pub const RES_16X9_2160P: Self = Self::new(3840, 2160);
    pub const RES_16X9_1800P: Self = Self::new(3200, 1800);
    pub const RES_16X9_1440P: Self = Self::new(2560, 1440);
    pub const RES_16X9_1080P: Self = Self::new(1920, 1080);
    pub const RES_16X9_720P: Self = Self::new(1280, 720);
    pub const RES_16X9_768P: Self = Self::new(1366, 768);

    pub const RES_4X3_600P: Self = Self::new(800, 600);
    pub const RES_4X3_480P: Self = Self::new(640, 480);
    pub const RES_4X3_240P: Self = Self::new(320, 240);
    pub const RES_4X3_120P: Self = Self::new(160, 120);

    /// Creates a new resolution from the given values.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[allow(
    clippy::from_over_into,
    // reason = "You cannot convert a fraction into a resolution as the `fraction` crate automatically simplifies representations."
)]
impl Into<fraction::Fraction> for SpecificResolution {
    #[inline]
    fn into(self) -> fraction::Fraction {
        fraction::Fraction::new(self.width, self.height)
    }
}

impl Display for SpecificResolution {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "SpecificResolution({} x {})",
            self.width, self.height
        ))
    }
}

impl From<v4l::Format> for SpecificResolution {
    #[inline]
    fn from(value: v4l::Format) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
