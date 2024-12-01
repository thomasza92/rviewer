#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod app;
use crate::app::Rviewer;

fn main() -> Result<(), eframe::Error> {
    egui_logger::builder().init().unwrap();
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_buttons_shown(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-512.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "[ rviewer ]",
        native_options,
        Box::new(|_cc| Ok(Box::new(Rviewer::new(_cc)))),
    )
}
