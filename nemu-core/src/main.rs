use nemu_core::Debugger;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Nemu Debugger",
        options,
        Box::new(|_cc| Ok(Box::<Debugger>::default())),
    )
}
