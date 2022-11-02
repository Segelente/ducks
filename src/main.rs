use std::fmt::format;
use std::path::PathBuf;
use eframe::egui;
use eframe::epaint::{ColorImage, Pos2};
use egui::{Rect, vec2, Vec2};
use egui::WidgetType::ImageButton;
use egui_extras::RetainedImage;
use image::DynamicImage;
use native_dialog::FileDialog;
use crate::camera::take_user_picture;
use crate::ducks::*;

mod ducks;
mod camera;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Entenbild Editor Pro Deluxe Edition",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );


}

struct MyApp{
    retained_image: Option<RetainedImage>,
    duck_images: Vec<RetainedImage>,
    scaler: f32,
    duck_scaler: Vec<f32>,
    selected_button: usize,
    save_image: DynamicImage,
    pointer_pos: Option<Pos2>

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            retained_image: None,
            duck_images: load_duck_images().iter_mut().map(|i| RetainedImage::from_color_image("Userpicture", i.clone())).collect(),
            scaler: 0.33,
            duck_scaler: vec![50.0;5],
            selected_button: 0,
            save_image: DynamicImage::default(),
            pointer_pos: None,
        }
    }
}

impl eframe::App for MyApp{
    /// Die Update Funktion wird bei jedem drawen des Fensters aufgerufen.(Es wird jedes mal neu
    /// berechnet wo die Elemente sind etc.)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut vec2 = Vec2::new(self.duck_scaler[self.selected_button], self.duck_scaler[self.selected_button]);
            if ctx.input().pointer.any_click() {
               self.pointer_pos = ctx.pointer_latest_pos();
            }
            if self.pointer_pos != None {
                ui.put(
                    Rect::from_center_size(self.pointer_pos.expect("Position not found"),vec2),
                    egui::Image::new(self.duck_images[self.selected_button].texture_id(ctx).clone(), vec2)
                );
            }
            // TODO: Drag and Drop for duck images
            match &self.retained_image {
                Some(image) => {
                    image.show_scaled(ui, self.scaler);
                }
                None => {
                    ui.label("No image loaded");
                }

            }
            });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(10.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Duck images ");

                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, image) in self.duck_images.iter_mut().enumerate() {
                        let image_button = ui.add(egui::ImageButton::new(image.texture_id(ctx), vec2(self.duck_scaler[index] / 2.0, self.duck_scaler[index] / 2.0)));
                        if image_button.clicked() {
                            self.selected_button = index;
                        }
                    }
                    ui.label(format!("Duck {} selected", self.selected_button+1));
                    ui.add(egui::Slider::new(&mut self.duck_scaler[self.selected_button], 50.0..=200.0).text("Use Slider to enlarge your duck!"));

                });
            });
        egui::TopBottomPanel::bottom("bottom")
            .show(ctx, |ui|{
                if ui.button("Speichern").clicked() {
                    save_user_images(self.save_image.clone());
                }
                if ui.button("Show me your duck").clicked() {
                    let helper = get_images(get_path_for_user_image());
                    self.save_image = helper.0;
                    self.retained_image = Some(RetainedImage::from_color_image("Userpicture", helper.1));
                }
                let picture_button = ui.add(egui::ImageButton::new((RetainedImage::from_image_bytes("camera_icon", include_bytes!("../used_images/icon_camera.png"))).expect("TODO: error message").texture_id(ctx), (20.0, 20.0)));
                if picture_button.clicked() {
                    take_user_picture();
                    let helper = get_images(PathBuf::from("used_images/unnamed.jpg"));
                    self.save_image = helper.0;
                    self.retained_image = Some(RetainedImage::from_color_image("Userpicture", helper.1));
                }
            ui.add(egui::Slider::new(&mut self.scaler, 0.0..=2.0).text("Use Slider to enlarge your image!"));

            });
    }
}