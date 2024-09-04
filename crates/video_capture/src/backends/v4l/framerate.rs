use fraction::{Fraction, One};
use nix::ioctl_readwrite;

use crate::error::VideoCaptureConfigError as ConfigError;

const VIDIOC_G_PARM: u8 = 21;
const IOCTL_MEDIA_COMMAND: u8 = b'V';

ioctl_readwrite!(
    vidioc_g_parm,
    IOCTL_MEDIA_COMMAND,
    VIDIOC_G_PARM,
    V4l2StreamParm
);

#[repr(C)]
pub(super) struct V4l2StreamParm {
    r#type: u32,
    parm: Parm,
}

impl V4l2StreamParm {
    /// Talks to the kernel to fill a `v4l2_stream_parm` structure.
    ///
    /// # Errors
    ///
    /// This can fail if the `ioctl` call has its invariants broken.
    ///
    /// See [the kernel docs](https://docs.kernel.org/userspace-api/media/v4l/vidioc-g-parm.html#c.V4L.VIDIOC_G_PARM) for more information.
    #[tracing::instrument]
    pub(crate) fn new(source_str: String, fd: i32) -> Result<Self, ConfigError> {
        // SAFETY: this creates a zeroed-out version of the `V412StreamParm`
        // struct, which is expected by the kernel.
        //
        // the kernel will then fill in the device details as necessary.
        let mut stream_parm = unsafe { core::mem::zeroed::<Self>() };
        tracing::trace!("successfully zeroed v4l2_stream_parm struct memory");

        // SAFETY: the kernel should fill in the struct correctly or return an
        // error code we can use to fail gracefully.
        let result = unsafe { vidioc_g_parm(fd, &mut stream_parm) };
        tracing::trace!("completed ioctl call w/ `VIDIOC_G_PARM`");

        match result {
            Ok(_) => Ok(stream_parm),
            Err(errno) => Err(ConfigError::CouldntGetFormat {
                source: source_str,
                err_msg: format!("ioctl call for `VIDIOC_G_PARM` failed with error code {errno}."),
            }),
        }
    }

    /// Gets the frame interval from the internal v4l2_captureparm union field.
    pub(crate) fn get_frame_interval(&self) -> Fraction {
        // get the union type
        //
        // SAFETY: we ALWAYS do a `VIDIOC_G_PARM`, which doesn't set anything.
        // in other words, we'll never use the other union fields.
        let frame_interval = unsafe { self.parm.v4l2_captureparm }.time_per_frame;

        // make it into a `fraction::Fraction`
        Fraction::new(frame_interval.numerator, frame_interval.denominator)
    }

    /// Creates a frame rate from the internal frame interval.
    ///
    /// A frame rate is 1 / `frame_interval`.
    pub(crate) fn get_frame_rate(&self) -> Fraction {
        Fraction::one() / self.get_frame_interval()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union Parm {
    v4l2_captureparm: V4l2CaptureParm,
    v4l2_outputparm: V4l2OutputParm,
    raw_data: [u8; 200],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct V4l2CaptureParm {
    capability: u32,
    capture_mode: u32,
    time_per_frame: V4l2Fract,
    extended_mode: u32,
    read_buffers: u32,
    reserved: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct V4l2OutputParm {
    capability: u32,
    output_mode: u32,
    time_per_frame: V4l2Fract,
    extended_mode: u32,
    write_buffers: u32,
    reserved: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct V4l2Fract {
    numerator: u32,
    denominator: u32,
}
