use std::path::PathBuf;

use serumcv_video_capture::backends::v4l;
use serumcv_video_capture::prelude::VideoCaptureImageConfiguration as ImageConfig;
use serumcv_video_capture::prelude::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // let mut device = v4l::V4LVideoCaptureDevice::new_first().unwrap();
    //
    // note: for some reason, my webcam ABSOLUTELY HATES being initialized at /dev/video0.
    // i MUST use /dev/media0 instead. so here's that hardcoded.
    //
    // please swap the lines to use your own webcam :D
    let mut device = v4l::V4LVideoCaptureDevice::new(PathBuf::from("/dev/media0")).unwrap();
    device.read_frame().unwrap();

    // grab it's image config like so...
    let conf = device.image_configuration().unwrap();
    tracing::info!("hey the webcam is running at {} FPS!", conf.framerate);

    // let's change the capture device settings
    let new_conf = ImageConfig {
        format: Format::AVC,
        resolution: SpecificResolution::RES_4X3_600P,
        framerate: Framerate::FPS_15,
    };
    let real_conf = device.set_image_configuration(&new_conf).unwrap();

    // let's hope they're equal!
    assert_eq!(new_conf, real_conf, "this is a very conservative config, so both configurations should be equal for virtually any device");
}
