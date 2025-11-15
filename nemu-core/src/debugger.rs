use crate::Nemu;

use eframe::egui;

pub struct Debugger {
    nemu: Nemu,
}

impl Default for Debugger {
    fn default() -> Self {
        Self {
            nemu: Nemu::default(),
        }
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0))
            .show(ctx, |ui| {
                ui.label("Debugger UI coming soon!");
                ui.separator();
                ui.label("Registers:");
                ui.code(self.nemu.get_regs_snapshot());
            });
    }
}
