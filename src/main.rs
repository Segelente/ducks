use std::fmt::format;
use std::ops::{Add, Index};
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
    pointer_pos: Vec<(Pos2, usize)>,
    image_pos_index: usize,
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
            pointer_pos: vec![],
            image_pos_index: 0,
        }
    }
}

impl eframe::App for MyApp{
    /// Die Update Funktion wird bei jedem drawen des Fensters aufgerufen.(Es wird jedes mal neu
    /// berechnet wo die Elemente sind etc.)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut vec2 = Vec2::new(self.duck_scaler[self.selected_button], self.duck_scaler[self.selected_button]);
            if self.retained_image.is_some() {
                let resolution_image = self.retained_image.as_mut().expect("No image loaded").size_vec2();
                ui.add(egui::TextEdit::singleline(&mut format!("{:?}", resolution_image)));
                if ctx.input().pointer.any_click(){
                    if vec2.x <= resolution_image.x && vec2.y <= resolution_image.y{
                        self.pointer_pos.push((ctx.pointer_latest_pos().expect("Position not set"), self.selected_button));
                    }
            }

            }
            if !self.pointer_pos.is_empty(){
                    for (index, (position, duck_selected)) in self.pointer_pos.iter().enumerate(){
                        ui.put(
                            Rect::from_center_size(position.clone(), vec2),
                            egui::Image::new(self.duck_images[duck_selected.clone()].texture_id(ctx).clone(), vec2)
                        );
                    }

                    self.image_pos_index += 1;
                }
            // TODO: Fix Duck scaling
            //TODO: Save ducks on top of images
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