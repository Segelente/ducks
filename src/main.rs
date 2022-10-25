use eframe::egui;
use egui::vec2;
use egui::WidgetType::ImageButton;
use egui_extras::RetainedImage;
use native_dialog::FileDialog;

use crate::ducks::*;

mod ducks;

fn main() {
    // In die options könnten wir so Dinge wie die Größe des Fensters und Fullscreen reinpacken
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

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            retained_image: None,
            duck_images: load_duck_images().iter_mut().map(|i| RetainedImage::from_color_image("Userpicture", i.clone())).collect(),
            scaler: 0.33,
            duck_scaler: vec![50.0;3],
            selected_button: 0,
        }
    }
}

impl eframe::App for MyApp{
    /// Die Update Funktion wird bei jedem drawen des Fensters aufgerufen.(Es wird jedes mal neu
    /// berechnet wo die Elemente sind etc.)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Jedes UI Element muss in einem Panel sein, damit egui weiß wohin damit

        egui::CentralPanel::default().show(ctx, |ui| {
            // Wir können `ui` mit Funktionen bearbeiten um ein Bild oder Text anzuzeigen.
            match &self.retained_image {
                Some(image) => {
                    // Hier fügen wir ein Bild mit Maximalgröße 500*500 hinzu
                    image.show_scaled(ui, self.scaler);
                }
                None => {
                    ui.label("No image loaded");
                }

            }
            // Wir können auch einen Button hinzufügen. Jedes UI Element wird über eine Funktion hinzugefügt
            if ui.button("Show me your duck").clicked() {
                // In egui_extras steht was von image support also man kann die auch von der Library
                // laden
                self.retained_image = Some(RetainedImage::from_color_image("Userpicture", get_images(get_path_for_user_image())));
            }
            });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(50.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Duck images");

                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, image) in self.duck_images.iter_mut().enumerate() {
                        let image_button = ui.add(egui::ImageButton::new(image.texture_id(ctx), vec2(self.duck_scaler[index] / 2.0, self.duck_scaler[index] / 2.0)));
                        if image_button.clicked() {
                            self.selected_button = index;
                        }
                    }
                });
            });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui|{
            ui.add(egui::Slider::new(&mut self.scaler, 0.0..=2.0).text("Use Slider to enlarge your image!"));
            ui.add(egui::Slider::new(&mut self.duck_scaler[self.selected_button], 50.0..=200.0).text("Use Slider to enlarge your duck!"));

        });
    }
}