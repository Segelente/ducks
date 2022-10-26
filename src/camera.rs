use egui::ColorImage;
use image::DynamicImage;
use nokhwa;
use nokhwa::{FrameFormat, Camera, CameraFormat, KnownCameraControls};
use nokhwa::KnownCameraControls::Brightness;

pub fn take_user_picture() {
    let path = "last_picture/unnamed.jpg";
// set up the Camera
    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(1280, 720, FrameFormat::MJPEG, 30)),
    )
        .unwrap();
// open stream
    camera.open_stream().unwrap();
    let frame = camera.frame().unwrap();
// save image
    image::save_buffer(path, &frame, frame.width(), frame.height(),
                       image::ColorType::Rgb8).expect("save_buffer");
    camera.stop_stream().expect("TODO: panic message");
}