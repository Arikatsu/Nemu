use eframe::egui;
use nemu_core::Debugger;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Nemu Debugger",
        options,
        Box::new(|_cc| Ok(Box::<Debugger>::default())),
    )
}
