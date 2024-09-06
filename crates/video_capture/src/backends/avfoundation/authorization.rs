/// A representation of a client's authorization to use the capture device.
///
/// See: https://developer.apple.com/documentation/avfoundation/avauthorizationstatus
#[repr(C)]
#[non_exhaustive]
pub(super) enum AVAuthorizationStatus {
    NotDetermined,
    Restricted,
    Denied,
    Authorized,
}

