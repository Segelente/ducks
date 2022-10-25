use std::fs;
use std::path::PathBuf;
use egui::ColorImage;
use image::{DynamicImage, ImageResult};
use native_dialog::FileDialog;

pub fn get_path_for_user_image() -> PathBuf {
    FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap().expect("No Image selected")

}
pub fn get_images(path: PathBuf) -> ColorImage{

    let img = image::open(path).expect("File not supported");
    let size = [img.width() as _, img.height() as _];
    let image_buffer = img.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice()
    )
}
pub fn load_duck_images() -> Vec<ColorImage> {
    let mut image_list = vec![];
    let paths = fs::read_dir("duck_images").unwrap();
    for path in paths {
        image_list.push(get_images(path.expect("No Image selected").path()));
    }
        image_list
}