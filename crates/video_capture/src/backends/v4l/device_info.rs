use std::{fs, io, os::fd::AsRawFd, path::Path};

use core::ffi::{c_char, CStr};

use nix::ioctl_readwrite;

/// A struct that contains information about some Linux media device.
///
/// See: https://docs.kernel.org/userspace-api/media/mediactl/media-ioc-device-info.html
#[repr(C)]
pub(super) struct MediaDeviceInfo {
    driver: [c_char; 16],
    model: [c_char; 32],
    serial: [c_char; 40],
    bus_info: [c_char; 32],
    media_version: u32,
    hw_revision: u32,
    driver_version: u32,
    reserved: [u32; 31],
}

const MEDIA_IOC_DEVICE_INFO_SEQ_NUM: u8 = 0x00;
const IOCTL_MEDIA_COMMAND_IDENT: u8 = b'|';

// call `media_ioc_device_info` to execute the `ioctl`
ioctl_readwrite!(
    media_ioc_device_info,
    IOCTL_MEDIA_COMMAND_IDENT,
    MEDIA_IOC_DEVICE_INFO_SEQ_NUM,
    MediaDeviceInfo
);

impl MediaDeviceInfo {
    /// Attempts to get information about the media device at the given path.
    #[tracing::instrument]
    pub(crate) fn get(path: &Path) -> Result<Self, io::Error> {
        // create an uninitialized MediaDeviceInfo
        //
        // SAFETY: This is fine since the kernel will write to this zeroed memory.
        let mut info = unsafe { core::mem::zeroed::<Self>() };
        tracing::trace!("successfully zeroed media_device_info struct memory");

        // grab the file descriptor
        let file = fs::File::open(path)?;
        let fd = file.as_raw_fd();
        tracing::trace!(
            "device fd for path `{}` created (`{}`)",
            path.to_string_lossy().to_string(),
            fd
        );

        // perform the ioctl
        //
        // SAFETY: The kernel will either write to this or fail to do so.
        //
        // If it does fail, we return using the question mark operator.
        unsafe {
            media_ioc_device_info(fd, &mut info)?;
        }
        tracing::trace!("ioctl `MEDIA_IOC_DEVICE_INFO` completed successfully!");
        Ok(info)
    }

    pub(crate) fn model(&self) -> String {
        // SAFETY: if the string isn't valid, that will be reported and the string
        // will be given a safe default
        unsafe {
            CStr::from_ptr(self.model.as_ptr())
                .to_str()
                .unwrap_or_else(|_| {
                    tracing::error!("`ioctl` to get capture device model contained invalid UTF-8");
                    "Model was not valid UTF-8"
                })
                .to_string()
        }
    }

    pub(crate) fn serial(&self) -> String {
        // SAFETY: if the string isn't valid, that will be reported and the string
        // will be given a safe default
        unsafe {
            CStr::from_ptr(self.serial.as_ptr())
                .to_str()
                .unwrap_or_else(|_| {
                    tracing::error!("`ioctl` to get capture device serial contained invalid UTF-8");
                    "Serial was not valid UTF-8"
                })
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_media_device_info() {
        let info = MediaDeviceInfo::get(&PathBuf::from("/dev/media0")).unwrap();
        assert_eq!(String::from("C922 Pro Stream Webcam"), info.model());
    }
}
