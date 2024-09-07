// use objc2::rc::autoreleasepool;
// use objc2::{class, msg_send};
// use objc2_foundation::NSString;

// use serumcv_video_capture::backends::avfoundation::AvFoundationBackend;

// #[expect(clippy::print_stdout)]
// fn main() {
//     let rust_string = av_media_type_video();
//     println!("AVMediaTypeVideo: {rust_string}");
// }

// fn av_media_type_video() -> String {
//     // Getting the AVMediaTypeVideo NSString from the AVFoundation framework.
//     // AFETY: ...
//     unsafe {
//         let av_media_type_video: *mut NSString = msg_send![class!(AVMediaType), AVMediaTypeVideo];
//         let reference = av_media_type_video.as_ref().unwrap();
//         autoreleasepool(|handle| reference.as_str(handle).to_string())
//     }
// }

use serumcv_video_capture::backends::avfoundation::AvBackend;
use serumcv_video_capture::prelude::*;

fn main() {
    list_devices();
}

#[expect(clippy::print_stdout)]
#[expect(clippy::use_debug)]
fn list_devices() {
    println!("{:#?}", AvBackend::list_connected_devices());
}
