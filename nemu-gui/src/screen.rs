use eframe::egui;

// mostly for simulating a screen for now, framebuffer for example will be moved to ppu
pub(super) struct Screen {
    framebuffer: Vec<u8>,
    texture: Option<egui::TextureHandle>,
    counter: u32,
}

impl Screen {
    const WIDTH: usize = 160;
    const HEIGHT: usize = 144;

    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; Self::WIDTH * Self::HEIGHT * 3],
            texture: None,
            counter: 0,
        }
    }
    
    pub fn texture(&self) -> Option<&egui::TextureHandle> {
        self.texture.as_ref()
    }

    // utility function for now to simulate framebuffer updates
    pub fn simulate_updates(&mut self, ctx: &egui::Context) {
        self.counter += 1;

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let offset = (y * Self::WIDTH + x) * 3;
                self.framebuffer[offset] = ((x + self.counter as usize) % 256) as u8;
                self.framebuffer[offset + 1] = ((y + self.counter as usize / 2) % 256) as u8;
                self.framebuffer[offset + 2] = ((x + y + self.counter as usize) % 256) as u8;
            }
        }
        
        let color_image = egui::ColorImage::from_rgb(
            [Self::WIDTH, Self::HEIGHT],
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
    }
}