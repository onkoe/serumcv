use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

extern crate alloc;

use anyhow::{anyhow, bail};

use crate::error::VideoCaptureConnectionError as ConnectionError;

/// Determining both of these upon instantiation allows users to
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4LSource {
    /// The user's input path, no matter how awful.
    pub given: PathBuf,
    /// A path to the `/dev/mediaX` for this capture device.
    pub media: PathBuf,
    /// The capture device's V4L representation path.
    pub video: PathBuf,
}

impl V4LSource {
    pub(crate) fn new(user_input: &Path) -> Result<Self, ConnectionError> {
        // grab the path if it exists...
        let path = match fs::read_link(user_input) {
            Ok(p) => p,
            Err(e) => {
                // this isn't an error. we just have a file, not a symlink!
                // let's see if it even exists and handle the various error cases
                match e.kind() {
                    ErrorKind::InvalidInput => {} // this is good. the file exists and isn't a symlink
                    ErrorKind::NotFound => {
                        return Err(ConnectionError::CouldntGetDeviceInfo {
                            source: user_input.to_string_lossy().into(), 
                            err_msg: "The device representation in `/dev/` was not found.".into() 
                        })
                    },
                    ek => {
                        return Err(ConnectionError::CouldntGetDeviceInfo {
                            source: user_input.to_string_lossy().into(), 
                            err_msg: format!("The given device couldn't be accessed as a symlink and wasn't a normal file. See: {ek}") 
                        })
                    },
                };

                user_input.to_owned()
            }
        };

        // by this point, we either have a `mediaX` or `videoY` path.
        // let's check...
        let Some(file_name) = path.file_name().map(|s| s.to_string_lossy().to_string()) else {
            return Err(ConnectionError::CouldntGetDeviceInfo {
                source: user_input.to_string_lossy().into(),
                err_msg: "The given device file points to a directory!".into(),
            });
        };

        if file_name.starts_with("media") {
            // we have `/dev/mediaX`. let's get its matching `/dev/videoY`
            // this is significantly more annoying than the other case!!!
            return Ok(Self {
                given: user_input.to_owned(),
                media: path.clone(),
                video: find_video_y(&path).map_err(|e| ConnectionError::CouldntGetDeviceInfo {
                    source: user_input.to_string_lossy().into(),
                    err_msg: e.to_string(),
                })?,
            });
        } else if file_name.starts_with("video") {
            // we have `/dev/videoY`. let's quickly grab its `/dev/mediaX`
            return Ok(Self {
                given: user_input.to_owned(),
                media: find_media_x(&path, user_input).map_err(|e| {
                    ConnectionError::CouldntGetDeviceInfo {
                        source: user_input.to_string_lossy().into(),
                        err_msg: e.to_string(),
                    }
                })?,
                video: path,
            });
        }
        Err(ConnectionError::CouldntGetDeviceInfo {
            source: user_input.to_string_lossy().into(),
            err_msg: "Didn't".into(),
        })
    }

    /// Makes a string from the user's given path input.
    pub(crate) fn user_source_string(&self) -> String {
        self.given.display().to_string()
    }
}

/// Given a `/dev/videoY` file, let's find a `/dev/mediaX` file that matches!
///
/// I'm using `anyhow` for this and converting the display message. Just makes things easier.
fn find_media_x(video_y_path: &Path, user_input: &Path) -> anyhow::Result<PathBuf> {
    let video_y_filename = video_y_path
        .file_name()
        .ok_or_else(|| anyhow!("failed to get filename for `videoY` file"))?;

    let devices_dir = PathBuf::from("/sys/class/video4linux/")
        .join(video_y_filename)
        .join("device");

    // let video_y_path_canon = video_y_path.canonicalize().map_err(|e| anyhow!("failed to get real path of `videoY` file. err: {e}"))?;

    Ok(std::fs::read_dir(&devices_dir).map_err(|e| anyhow!("Couldn't find a Video4Linux capture device (unparsed source: `{}`) directory at `{}`. IO Error: `{e}`", user_input.display(), devices_dir.display()))?
        .flatten()
        .find(|entry| entry.path().to_string_lossy().to_string().contains("media"))
        .ok_or_else(|| anyhow!("didn't find a matching `mediaX` for the `videoY file."))?
        .path()
    ).map(|v| fs::read_link(&v).map_or(v, |link| link))
}

/// Given a `/dev/mediaX` file, let's find a `/dev/videoY` file that matches.
///
/// I'm using `anyhow` for this and converting the display message. Just makes things easier.
fn find_video_y(media_x_path: &Path) -> anyhow::Result<PathBuf> {
    // get the filename
    let media_x_filename = media_x_path
        .file_name()
        .ok_or_else(|| anyhow!("failed to get filename for `videoY` file"))?;

    let media_x_path_canon = media_x_path
        .canonicalize()
        .map_err(|e| anyhow!("failed to get real path of `videoY` file. err: {e}"))?;

    // make a list of dirs in `/sys/class/video4linux`.
    // then, check each dir for a matching mediaX, and return the `videoY` path that worked!
    fs::read_dir("/sys/class/video4linux/")
        .map_err(|e| anyhow!("The `/sys/class/video4linux/` directory should exist and be readable. Got IO error: {e}"))?
        .flatten()
        // get the path of each `/sys/class/v.../videoY` entry
        .map(|entry| entry.path())
        // filter the paths to make sure they're all directories
        .filter(|path| path.is_dir()) 
        // add `device/mediaX` to them
        .map(|dir| dir.join("device").join(media_x_filename))
        // make sure they have filenames
        .filter_map(|d| d.file_name().map(|filename| (d.clone(), filename.to_owned()))) 
        .map(|(dir, filename)| {tracing::trace!("to find video here: {dir:?}. the fr one is `{media_x_path_canon:?}`"); (dir, filename)})
        // see if we found any 
        .find(|(_, filename)| filename.as_os_str() == media_x_filename)  
        .ok_or_else(|| anyhow!("No Video4Linux devices appear to own the given `mediaX` file at `{}`.", media_x_path.display()))
        .map(|(path, _)| { 
            // let's remove comps off the path until we find the `videoY` part
            for component in path.components() {
                let strd = component.as_os_str().to_string_lossy();
                if strd.contains("video") && strd != "video4linux" {
                    let real_path = PathBuf::from("/dev/").join(strd.to_string());
                    return if real_path.exists() {
                        Ok(real_path)
                    } else {
                        bail!("The videoY we found (`{}`) for the given path does not exist.", real_path.display())
                    };
                }
            }
            bail!("No path contained `videoY`... which is weird. All of them should.")        
        })?

}
