mod disassembler;
mod fps_tracker;
mod memory_viewer;

use eframe::egui;
use std::time::Instant;

use crate::Nemu;
use disassembler::Disassembler;
use fps_tracker::FpsTracker;
use memory_viewer::MemoryViewer;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const GB_CYCLES_PER_SEC: f64 = 4_194_304.0;

const PALETTE: [u32; 4] = [
    u32::from_le_bytes([0xE0, 0xF8, 0xD0, 0xFF]),
    u32::from_le_bytes([0x88, 0xC0, 0x70, 0xFF]),
    u32::from_le_bytes([0x34, 0x68, 0x56, 0xFF]),
    u32::from_le_bytes([0x08, 0x18, 0x20, 0xFF]),
];

pub struct Debugger {
    nemu: Nemu,
    cur_rom: String,

    screen_pixels: Vec<u8>,
    screen_tex: egui::TextureHandle,

    running: bool,
    last_update: Instant,
    tick_accumulator: f64,

    memory_viewer: MemoryViewer,
    disassembler: Disassembler,
    fps_tracker: FpsTracker,
}

impl Debugger {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let blank_image =
            egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &vec![0; WIDTH * HEIGHT * 4]);

        let screen_tex =
            cc.egui_ctx
                .load_texture("screen", blank_image, egui::TextureOptions::NEAREST);

        let mut debugger = Self {
            nemu: Nemu::default(),
            cur_rom: String::new(),

            screen_tex,
            screen_pixels: vec![0; WIDTH * HEIGHT * 4],

            running: false,
            last_update: Instant::now(),
            tick_accumulator: 0.0,

            memory_viewer: MemoryViewer::new(),
            disassembler: Disassembler::new(),
            fps_tracker: FpsTracker::new(),
        };

        debugger.memory_viewer.refresh_memory_view(&debugger.nemu.bus);

        debugger
    }

    fn update_screen_texture(&mut self) {
        let fb = self.nemu.get_framebuffer();

        let pixels_u32 = unsafe {
            std::slice::from_raw_parts_mut(
                self.screen_pixels.as_mut_ptr() as *mut u32,
                WIDTH * HEIGHT,
            )
        };

        for i in 0..fb.len() {
            unsafe {
                *pixels_u32.get_unchecked_mut(i) =
                    *PALETTE.get_unchecked(*fb.get_unchecked(i) as usize);
            }
        }

        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &self.screen_pixels);

        self.screen_tex
            .set(color_image, egui::TextureOptions::NEAREST);
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {

            if ui.button("📂").on_hover_text("Open ROM").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("GameBoy", &["gb", "bin"])
                    .pick_file()
                {
                    if let Err(e) = self.nemu.load_cartridge(&path) {
                        eprintln!("Failed to load ROM: {}", e);
                    } else {
                        self.cur_rom = path.file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        self.update_screen_texture();
                        self.fps_tracker.reset();
                        self.disassembler.invalidate_cache();
                        self.memory_viewer.refresh_memory_view(&self.nemu.bus);
                    }
                }
            }

            ui.separator();

            if !self.cur_rom.is_empty() {
                ui.label(egui::RichText::new(&self.cur_rom).monospace());
            } else {
                ui.label(egui::RichText::new("No ROM Loaded").italics().weak());
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("FPS: {:.2}", self.fps_tracker.fps));

                ui.separator();

                if ui.button("🔄 Reset").clicked() {
                    self.nemu.reset();
                    self.running = false;
                    self.update_screen_texture();
                    self.fps_tracker.reset();
                    self.disassembler.invalidate_cache();
                    self.memory_viewer.refresh_memory_view(&self.nemu.bus);
                }

                if ui.button("⏭ Step").clicked() {
                    self.running = false;
                    self.nemu.step();
                    self.update_screen_texture();
                }

                if ui.button(if self.running { "⏸ Pause" } else { "▶ Run" }).clicked() {
                    self.running = !self.running;
                }
            });
        });
    }

    fn render_cpu_window(&mut self, ui: &mut egui::Ui) {
        let regs = &self.nemu.cpu.regs;

        egui::Grid::new("cpu_regs")
            .spacing([20.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("A");
                ui.monospace(format!("{:02X}", regs.a));
                ui.label("F");
                ui.monospace(format!("{:02X}", regs.f));
                ui.end_row();
                ui.label("B");
                ui.monospace(format!("{:02X}", regs.b));
                ui.label("C");
                ui.monospace(format!("{:02X}", regs.c));
                ui.end_row();
                ui.label("D");
                ui.monospace(format!("{:02X}", regs.d));
                ui.label("E");
                ui.monospace(format!("{:02X}", regs.e));
                ui.end_row();
                ui.label("H");
                ui.monospace(format!("{:02X}", regs.h));
                ui.label("L");
                ui.monospace(format!("{:02X}", regs.l));
                ui.end_row();
                ui.label("SP");
                ui.monospace(format!("{:04X}", regs.sp));
                ui.label("PC");
                ui.monospace(format!("{:04X}", regs.pc));
                ui.end_row();
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
                .inner_margin(egui::Margin {
                    left: 4,
                    right: 4,
                    top: 2,
                    bottom: 2,
                })
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

            flag(
                ui,
                "IME",
                self.nemu.cpu.ime != crate::cpu::InterruptMode::Disabled,
            );
        });
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() {
            return;
        }

        ctx.input(|i| {
           let joypad = &mut self.nemu.bus.joypad;

            joypad.set_joypad(crate::JoypadButton::RightOrA, i.key_down(egui::Key::Z), false);
            joypad.set_joypad(crate::JoypadButton::LeftOrB, i.key_down(egui::Key::X), false);
            joypad.set_joypad(crate::JoypadButton::DownOrStart, i.key_down(egui::Key::Enter), false);
            joypad.set_joypad(crate::JoypadButton::UpOrSelect, i.key_down(egui::Key::Space), false);

            joypad.set_joypad(crate::JoypadButton::RightOrA, i.key_down(egui::Key::ArrowRight), true);
            joypad.set_joypad(crate::JoypadButton::LeftOrB, i.key_down(egui::Key::ArrowLeft), true);
            joypad.set_joypad(crate::JoypadButton::DownOrStart, i.key_down(egui::Key::ArrowDown), true);
            joypad.set_joypad(crate::JoypadButton::UpOrSelect, i.key_down(egui::Key::ArrowUp), true);
        });
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            self.handle_input(ctx);

            let now = Instant::now();
            let dt = now.duration_since(self.last_update).as_secs_f64();
            self.last_update = now;

            const MAX_ACCUM: f64 = GB_CYCLES_PER_SEC / 20.0;
            self.tick_accumulator += dt * GB_CYCLES_PER_SEC;

            if self.tick_accumulator > MAX_ACCUM {
                self.tick_accumulator = MAX_ACCUM;
            }

            while self.tick_accumulator > 0.0 {
                let cycles = self.nemu.step();
                self.tick_accumulator -= cycles as f64;
            }

            if self.nemu.has_frame() {
                self.update_screen_texture();
                self.fps_tracker.update();
            }

            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        } else {
            self.last_update = Instant::now();
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                self.render_header(ui);
            });

        egui::Window::new("Screen")
            .show(ctx, |ui| {
                ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                    id: self.screen_tex.id(),
                    size: egui::vec2((WIDTH * 2) as f32, (HEIGHT * 2) as f32),
                }));
            });

        egui::Window::new("Disassembly")
            .default_pos([17.0, 400.0])
            .default_size([300.0, 550.0])
            .min_width(300.0)
            .show(ctx, |ui| {
                self.disassembler.render(ui, &self.nemu);
            });

        egui::Window::new("CPU")
            .default_pos([360.0, 55.0])
            .default_size([240.0, 300.0])
            .show(ctx, |ui| {
                self.render_cpu_window(ui);
            });

        egui::Window::new("Memory Viewer")
            .default_pos([625.0, 55.0])
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                self.memory_viewer.render(ui, &self.nemu);
            });
    }
}
