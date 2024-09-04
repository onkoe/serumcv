/// A FourCC format.
///
/// Consider using one of the pre-defined constants to get this type. They're
/// safe defaults!
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Format([u8; 4]);

impl Format {
    /// An early, well-supported, inefficient 'format'.
    ///
    /// Note that MJPEG isn't formally defined, so most user-facing software
    /// will treat it like garbage.
    ///
    /// Consider using `MJPEG2000` instead if your uses support it.
    pub const MJPEG: Self = Self(*b"MPEG");

    /// A modern, efficient format that has low utilization.
    ///
    /// Note that MJPEG2000, like many MJPEG papers, directs codecs to do few
    /// cross-frame optimizations. This results in lower efficiency, but
    /// immediate stream recovery, making it decent for simple streaming tasks.
    pub const MJPEG2000: Self = Self(*b"MJP2");

    /// A popular (though now supplanted) format.
    ///
    /// Often known as "H.264". Effectively replaced by `HEVC`.
    pub const AVC: Self = Self(*b"H264");

    /// A modern format which is significantly more efficient then AVC.
    ///
    /// Sometimes known as H.265.
    pub const HEVC: Self = Self(*b"HEVC");

    /// A well-established, popular format that's good for older devices.
    ///
    /// Replaced by `AV1`.
    pub const VP9: Self = Self(*b"VP90");

    /// A modern, open format with high adoption and great efficiency.
    pub const AV1: Self = Self(*b"av10");

    /// Creates a new FourCC format identifier.
    ///
    /// Note that input isn't checked with any database. Consider using the
    /// given **constants on this type instead**.
    ///
    /// Also, you can pass in byte strings instead of manually creating arrays:
    ///
    /// ```
    /// use serumcv::video::config::Format;
    ///
    /// let weird_format = Format::new(*b"apch");
    /// ```
    #[inline]
    pub const fn new(input: [u8; 4]) -> Self {
        Self(input)
    }

    /// Returns the inner FourCC byte array representing the format type.
    #[inline]
    pub const fn array(self) -> [u8; 4] {
        self.0
    }
}

#[cfg_attr(
    feature = "linux_v4l",
    cfg(any(target_os = "linux", target_os = "freebsd"))
)]
impl From<v4l::Format> for Format {
    #[inline]
    fn from(value: v4l::Format) -> Self {
        Self(value.fourcc.repr)
    }
}
