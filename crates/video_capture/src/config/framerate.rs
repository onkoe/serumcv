use fraction::Fraction;

/// A framerate value.
///
/// This uses the `fraction::Fraction` type underneath, so it is accurate enough
/// for general computer vision usage.
pub type Framerate = Fraction;

impl FramerateConsts for Framerate {}

/// A simple trait to hold common framerate constants.
///
/// This is implemented on the `Framerate` type alias to dodge Rust's orphan
/// rule (i.e. you can't do `impl Framerate`).
pub trait FramerateConsts {
    /* NTSC */
    /// A common NTSC framerate.
    const FPS_30: Fraction = Fraction::new_raw(30, 1);
    /// A common NTSC framerate. Two times faster than `FPS_30`.
    const FPS_60: Fraction = Fraction::new_raw(60, 1);

    /* PAL */
    /// A common PAL framerate.
    const FPS_25: Fraction = Fraction::new_raw(25, 1);
    /// A common PAL framerate. Two times faster than `FPS_25`.
    const FPS_50: Fraction = Fraction::new_raw(50, 1);

    /* Film */
    const FPS_24: Fraction = Fraction::new_raw(24, 1);

    /* Webcam: some framerates that wabcams just have sometimes... */
    const FPS_20: Fraction = Fraction::new_raw(20, 1);
    const FPS_15: Fraction = Fraction::new_raw(15, 1);
    const FPS_10: Fraction = Fraction::new_raw(10, 1);
    const FPS_7: Fraction = Fraction::new_raw(7, 1);
    const FPS_5: Fraction = Fraction::new_raw(5, 1);
}
