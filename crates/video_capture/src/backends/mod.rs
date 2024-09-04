use crate::{VideoCaptureDescriptor, VideoCaptureStream};

/// A user's selected backend.
#[expect(clippy::exhaustive_enums, reason = "this enum will never expand")]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum BackendSelection {
    Auto,
    Custom(BackendType),
}

/// A list of all supported backends.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum BackendType {
    /// [Video4Linux 2](https://docs.kernel.org/driver-api/media/v4l2-intro.html).
    V4L2,
    /// An agnostic backend which has logic for older systems.
    FFmpeg,
    /// [AVFoundation](https://developer.apple.com/documentation/avfoundation/capture_setup) is the modern Apple backend.
    AvFoundation,
    /// [MediaFoundation](https://learn.microsoft.com/en-us/windows/win32/medfound/audio-video-capture-in-media-foundation) is the new Windows (10+) backend.
    MediaFoundation,
    /// An older backend for Windows. Somewhat slow.
    DirectShow,
}

pub trait Backend<'path, 'conn>
where
    'path: 'conn,
{
    type Descriptor: VideoCaptureDescriptor;
    type Device: VideoCaptureStream<'path, 'conn, Self::Source>
    where
        Self::Source: 'path;
    type Source;
    type OwnedSource;

    /// Returns a list of video capture devices, like webcams and cameras, that
    /// are currently connected to the system (according to the backend).
    ///
    /// Note that a backend may list several capture devices for one physical
    /// device, completely miss a device, or some other issue.
    ///
    /// If you do encounter such a problem, please report it.
    fn list_connected_devices() -> Vec<Self::OwnedSource>;

    /// Returns an identifier for this backend.
    fn backend_type() -> BackendType;
}
