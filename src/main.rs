mod app;
mod core;

use crate::app::App;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([160.0 * 4.0, 200.0 * 4.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Nemu",
        options,
        Box::new(|_cc| Ok(Box::<App>::default()),
    ))
}
