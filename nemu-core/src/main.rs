use eframe::egui;
use nemu_core::Debugger;

pub fn set_style(cc: &eframe::CreationContext<'_>) {
    let mut style = (*cc.egui_ctx.style()).clone();

    style.spacing.item_spacing = egui::vec2(6.0, 3.0);
    style.spacing.window_margin = egui::Margin::same(6);
    style.spacing.button_padding = egui::vec2(4.0, 2.0);
    style.spacing.indent = 12.0;

    style.text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::new(15.0, egui::FontFamily::Proportional));

    style.visuals.window_fill = egui::Color32::from_gray(20);
    style.visuals.panel_fill = egui::Color32::from_gray(25);

    cc.egui_ctx.set_style(style);
}

fn main() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Nemu Debugger",
        options,
        Box::new(|cc| {
            set_style(cc);
            Ok(Box::new(Debugger::new(cc)))
        })
    )
}
