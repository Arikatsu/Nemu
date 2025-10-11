use super::screen::Screen;

use eframe::egui;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct App {
    screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::new(),
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
            });

        ctx.request_repaint();
    }
}