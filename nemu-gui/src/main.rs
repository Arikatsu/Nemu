mod screen;

use screen::Screen;
use nemu_core::Nemu;

use eframe::egui;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct App {
    screen: Screen,
    nemu: Nemu,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::new(),
            nemu: Nemu::with_rom("tests/cpu_instrs/individual/01-special.gb").expect("Failed to load ROM"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.screen.simulate_updates(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(0)
            )
            .show(ctx, |ui| {
                if let Some(texture) = &self.screen.texture() {
                    ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                        id: texture.id(),
                        size: egui::vec2(WIDTH as f32 * 4.0, HEIGHT as f32 * 4.0),
                    }));
                }

                ui.separator();
                ui.label("Registers:");
                ui.monospace(self.nemu.get_regs_snapshot());
                ui.separator();

                if ui.button("Reset").clicked() {
                    self.nemu.reset();
                }
                if ui.button("Step").clicked() {
                    self.nemu.step();
                }
            });
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

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