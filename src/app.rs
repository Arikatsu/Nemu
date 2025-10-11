use eframe::egui;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct MainWindow {
    framebuffer: Vec<u8>,
    texture: Option<egui::TextureHandle>,

    // for simulation
    counter: u32,
}

impl MainWindow {
    // utility function for now to simulate framebuffer updates
    fn simulate_updates(&mut self) {
        self.counter += 1;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset = (y * WIDTH + x) * 3;
                self.framebuffer[offset] = ((x + self.counter as usize) % 256) as u8;
                self.framebuffer[offset + 1] = ((y + self.counter as usize / 2) % 256) as u8;
                self.framebuffer[offset + 2] = ((x + y + self.counter as usize) % 256) as u8;
            }
        }
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            framebuffer: vec![0; WIDTH * HEIGHT * 3],
            texture: None,
            counter: 0,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.simulate_updates();

        let color_image = egui::ColorImage::from_rgb(
            [WIDTH, HEIGHT],
            &self.framebuffer,
        );

        match &mut self.texture {
            Some(texture) => texture.set(color_image, egui::TextureOptions::NEAREST),
            None => {
                self.texture = Some(ctx.load_texture(
                    "screen",
                    color_image,
                    egui::TextureOptions::NEAREST,
                ));
            }
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(0)
            )
            .show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                    id: texture.id(),
                    size: egui::vec2(WIDTH as f32 * 4.0, HEIGHT as f32 * 4.0),
                }));
            }
        });

        ctx.request_repaint();
    }
}