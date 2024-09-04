use error::{VideoCaptureConnectionError as ConnectionError, VideoCaptureUsageError as UsageError};

pub mod backends;
pub mod config;
pub mod error;
pub mod prelude;

// TODO: pub use config::(...);

/// Identifying information for a video capture device.
pub trait VideoCaptureDescriptor {
    type IdentiferTy;
    type ModelTy;

    /// Returns a unique serial ID string for a capture device.
    fn device_identifier(&self) -> Self::IdentiferTy;
    /// Returns the device model of a capture device.
    fn device_model(&self) -> Self::ModelTy;
}

pub trait VideoCaptureConnection<'path, Source>
where
    Source: 'path,
    Self: Sized,
{
    /// The source input type that a stream will take by default.
    ///
    /// Note that this should NEVER be an index. Always require some unique
    /// identifier to connect to a capture device.
    type Source;

    /// Creates a new video capture device representation.
    ///
    /// This will automatically connect to the capture device and return an
    /// error if the connection isn't working as expected.
    ///
    /// # Errors
    ///
    /// This method can error if the video capture device isn't connected to
    /// the system or is already in use.
    fn new(source: Self::Source) -> Result<Self, ConnectionError>;

    /// Attempts to create a new `VideoCapture` by checking for any available
    /// devices on the system.
    ///
    /// # Errors
    ///
    /// Can error if there are no capture devices on the system or the device
    /// is inaccessible.
    fn new_first() -> Result<Self, ConnectionError>;

    /// Attempts to disconnect from the device.
    ///
    /// # Errors
    ///
    /// If the device is already disconnected or isn't responding, this method
    /// may return an error.
    fn disconnect(&mut self) -> Result<(), ConnectionError>;

    /// Attempts to reconnect to an inactive video capture device.
    ///
    /// # Errors
    ///
    /// This method can error if the video capture device isn't connected to
    /// the system, is already in use, or is already connected to this
    /// instance.
    fn reconnect(&mut self) -> Result<(), ConnectionError>;
}

/// Some kind of 'stream' which yields frames when prompted.
///
/// In reality, this is likely calling a function which accesses a stream,
/// but it's effectively a stream either way!
pub trait VideoCaptureStream<'path, 'conn, Source>
where
    'path: 'conn,
    Source: 'path,
    Self: Sized,
{
    /// A buffer type which the stream will write into.
    type Buffer;

    /// The source input type that a stream will take by default.
    ///
    /// Note that this should NEVER be an index. Always require some unique
    /// identifier to connect to a capture device.
    type Source;

    /// A backend's frame type.
    type Metadata;

    /// Attempts to read a frame from the stream into the stream's internal
    /// buffer.
    ///
    /// FIXME: uhh. how do we want users to access the buffer?
    ///      | seems bad to make a pointer, and waiting on a RwLock seems
    ///      | inefficient at best :p
    ///
    /// # Errors
    ///
    /// This can return an error if the backend doesn't support the capture
    /// device, it is disconnected, or it is in use by another application.
    fn read_frame<'func>(
        &'func mut self,
    ) -> Result<(&'func [u8], &'func Self::Metadata), crate::UsageError>
    where
        'path: 'func;

    // Attempts to read a frame from the stream into the given buffer.
    //
    // This will not mutate the stream's internal buffer.
    // fn read_frame_into_buf(&mut self, buf: &mut Self::Buffer) -> Result<(), VideoCaptureReadError>;
}

// TODO: make these docs user-facing. b/c they are lol
//
/// A generic video capture device for any backend.
///
/// Each backend supplies its own type.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VideoCapture<Desc: VideoCaptureDescriptor, Device, Source, Stream> {
    descriptor: Desc,
    device: Device,
    source: Source,
    stream: Stream,
}

// this is just here to help people find the ident/model values.
//
// it's generic, so it's available for all implementations
impl<Desc: VideoCaptureDescriptor, Device, Source, Stream>
    VideoCapture<Desc, Device, Source, Stream>
{
    /// Returns a unique serial ID string for a capture device.
    #[inline]
    pub fn device_identifer(&self) -> Desc::IdentiferTy {
        self.descriptor.device_identifier()
    }

    /// Returns the device model of a capture device.
    #[inline]
    pub fn device_model(&self) -> Desc::ModelTy {
        self.descriptor.device_model()
    }
}
