use crate::Nemu;
use std::time::Instant;

use eframe::egui;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const GB_CYCLES_PER_SEC: f64 = 4_194_304.0;
const MAX_CYCLES_PER_FRAME: i32 = 50000;

const SHADES: [[u8; 3]; 4] = [
    [0xE0, 0xF8, 0xD0],
    [0x88, 0xC0, 0x70],
    [0x34, 0x68, 0x56],
    [0x08, 0x18, 0x20],
];

const PALETTE_RGBA: [[u8; 4]; 256] = {
    let mut table = [[0u8; 4]; 256];
    let mut i = 0;
    while i < 256 {
        let color = i & 0x3;
        table[i] = [SHADES[color][0], SHADES[color][1], SHADES[color][2], 255];
        i += 1;
    }
    table
};

pub struct Debugger {
    nemu: Nemu,
    rom_path: String,

    screen_pixels: Vec<u8>,
    screen_tex: egui::TextureHandle,

    running: bool,
    last_update: Instant,
    tick_accumulator: f64,
}

impl Debugger {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let blank_image = egui::ColorImage::from_rgba_unmultiplied(
            [WIDTH, HEIGHT],
            &vec![0; WIDTH * HEIGHT * 4],
        );

        let screen_tex = cc.egui_ctx.load_texture(
            "screen",
            blank_image,
            egui::TextureOptions::NEAREST,
        );

        Self {
            nemu: Nemu::default(),
            rom_path: String::new(),

            screen_tex,
            screen_pixels: vec![0; WIDTH * HEIGHT * 4],

            running: false,
            last_update: Instant::now(),
            tick_accumulator: 0.0,
        }
    }

    fn update_screen_texture(&mut self) {
        let fb = self.nemu.get_framebuffer();

        for (i, &pixel) in fb.iter().enumerate() {
            let rgba = &PALETTE_RGBA[pixel as usize];
            let offset = i * 4;
            self.screen_pixels[offset..offset + 4].copy_from_slice(rgba);
        }

        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &self.screen_pixels);

        self.screen_tex
            .set(color_image, egui::TextureOptions::NEAREST);
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ROM Path:");
            ui.text_edit_singleline(&mut self.rom_path);

            if ui.button("Load").clicked() {
                if let Err(e) = self.nemu.load_cartridge(&self.rom_path) {
                    eprintln!("Failed to load ROM: {}", e);
                } else {
                    self.update_screen_texture();
                }
            }

            ui.separator();

            if ui
                .button(if self.running { "⏸ Pause" } else { "▶ Run" })
                .clicked()
            {
                self.running = !self.running;
            }

            if ui.button("⏭ Step").clicked() {
                if self.running {
                    self.running = false;
                }
                self.nemu.step();
                self.update_screen_texture();
            }

            if ui.button("🔄 Reset").clicked() {
                self.nemu.reset();
                self.running = false;
                self.update_screen_texture();
            }
        });
    }

    fn render_cpu_window(&mut self, ui: &mut egui::Ui) {
        let regs = &self.nemu.cpu.regs;

        egui::Grid::new("cpu_regs")
            .spacing([20.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("A"); ui.monospace(format!("{:02X}", regs.a));
                ui.label("F"); ui.monospace(format!("{:02X}", regs.f)); ui.end_row();
                ui.label("B"); ui.monospace(format!("{:02X}", regs.b));
                ui.label("C"); ui.monospace(format!("{:02X}", regs.c)); ui.end_row();
                ui.label("D"); ui.monospace(format!("{:02X}", regs.d));
                ui.label("E"); ui.monospace(format!("{:02X}", regs.e)); ui.end_row();
                ui.label("H"); ui.monospace(format!("{:02X}", regs.h));
                ui.label("L"); ui.monospace(format!("{:02X}", regs.l)); ui.end_row();
                ui.label("SP"); ui.monospace(format!("{:04X}", regs.sp));
                ui.label("PC"); ui.monospace(format!("{:04X}", regs.pc)); ui.end_row();
            });

        ui.add_space(10.0);
        ui.separator();

        let flag = |ui: &mut egui::Ui, label: &str, on: bool| {
            let color = if on {
                egui::Color32::from_rgb(0, 180, 230)
            } else {
                egui::Color32::from_rgb(80, 80, 80)
            };

            egui::Frame::default()
                .inner_margin(egui::Margin { left: 4, right: 4, top: 2, bottom: 2 })
                .show(ui, |ui| {
                    ui.colored_label(color, egui::RichText::new(label).heading().size(16.0));
                });
        };

        ui.horizontal(|ui| {
            flag(ui, "Z", regs.zero_flag());
            flag(ui, "N", regs.subtract_flag());
            flag(ui, "H", regs.half_carry_flag());
            flag(ui, "C", regs.carry_flag());

            ui.separator();

            flag(ui, "IME", self.nemu.cpu.ime != crate::cpu::InterruptMode::Disabled);
        });
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            let now = Instant::now();
            let delta = now.duration_since(self.last_update).as_secs_f64();
            self.last_update = now;

            let ticks_to_run = (delta * GB_CYCLES_PER_SEC) as i32;
            self.tick_accumulator += ticks_to_run as f64;

            let cycles_this_frame = self.tick_accumulator.min(MAX_CYCLES_PER_FRAME as f64) as i32;

            let mut cycles_run = 0;
            while cycles_run < cycles_this_frame {
                let ticks = self.nemu.step();
                cycles_run += ticks as i32;
            }

            self.tick_accumulator -= cycles_run as f64;

            if self.nemu.has_frame() {
                self.update_screen_texture();
            }

            ctx.request_repaint();
        } else {
            self.last_update = Instant::now();
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                self.render_header(ui);
            });

        egui::Window::new("CPU")
            .default_size([0.0, 0.0])
            .show(ctx, |ui| {
            self.render_cpu_window(ui);
        });

        egui::Window::new("Screen")
            .default_pos([260.0, 55.0])
            .show(ctx, |ui| {
            ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                id: self.screen_tex.id(),
                size: egui::vec2(WIDTH as f32 * 2.5, HEIGHT as f32 * 2.5),
            }));
        });
    }
}
