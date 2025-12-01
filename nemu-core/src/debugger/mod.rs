mod disassembler;

use crate::Nemu;
use std::time::Instant;

use eframe::egui;

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
    rom_path: String,

    screen_pixels: Vec<u8>,
    screen_tex: egui::TextureHandle,

    running: bool,
    last_update: Instant,
    tick_accumulator: f64,

    disasm_base_pc: u16,
    disasm_lines: Vec<(u16, String, String)>,
    last_disasm_update: Instant,

    memory_viewer_addr: u16,
    memory_viewer_addr_input: String,
    memory_viewer_data: Vec<u8>,
}

impl Debugger {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let blank_image =
            egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &vec![0; WIDTH * HEIGHT * 4]);

        let screen_tex =
            cc.egui_ctx
                .load_texture("screen", blank_image, egui::TextureOptions::NEAREST);

        Self {
            nemu: Nemu::default(),
            rom_path: String::new(),

            screen_tex,
            screen_pixels: vec![0; WIDTH * HEIGHT * 4],

            running: false,
            last_update: Instant::now(),
            tick_accumulator: 0.0,

            disasm_base_pc: 0,
            disasm_lines: Vec::new(),
            last_disasm_update: Instant::now(),

            memory_viewer_addr: 0,
            memory_viewer_addr_input: "0000".to_string(),
            memory_viewer_data: vec![0; 256],
        }
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
            ui.label("ROM Path:");
            ui.text_edit_singleline(&mut self.rom_path);

            if ui.button("Load").clicked() {
                if let Err(e) = self.nemu.load_cartridge(&self.rom_path) {
                    eprintln!("Failed to load ROM: {}", e);
                } else {
                    self.update_screen_texture();
                    self.disasm_base_pc = self.nemu.cpu.regs.pc;
                    self.disasm_lines.clear();
                    self.last_disasm_update = Instant::now();
                    self.refresh_memory_view();
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
                self.disasm_base_pc = self.nemu.cpu.regs.pc;
                self.disasm_lines.clear();
                self.last_disasm_update = Instant::now();
            }

            if ui.button("🔄 Reset").clicked() {
                self.nemu.reset();
                self.running = false;
                self.update_screen_texture();
                self.disasm_base_pc = self.nemu.cpu.regs.pc;
                self.disasm_lines.clear();
                self.last_disasm_update = Instant::now();
            }
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

    fn render_memory_viewer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Address:");
            ui.add(
                egui::TextEdit::singleline(&mut self.memory_viewer_addr_input)
                    .desired_width(80.0)
                    .font(egui::TextStyle::Monospace),
            );

            if ui.button("Go").clicked() {
                if let Ok(addr) = u16::from_str_radix(&self.memory_viewer_addr_input, 16) {
                    self.memory_viewer_addr = addr;
                    self.refresh_memory_view();
                }
            }

            ui.separator();

            if ui.button("Jump to PC").clicked() {
                self.memory_viewer_addr = self.nemu.cpu.regs.pc;
                self.refresh_memory_view();
            }

            if ui.button("Jump to SP").clicked() {
                self.memory_viewer_addr = self.nemu.cpu.regs.sp;
                self.refresh_memory_view();
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        egui::Grid::new("memory_grid")
            .spacing([20.0, 4.0])
            .show(ui, |ui| {
                for row in 0..16 {
                    let row_addr = self.memory_viewer_addr.wrapping_add((row * 16) as u16);

                    ui.label(
                        egui::RichText::new(format!("{:04X}:", row_addr))
                            .monospace()
                            .color(egui::Color32::from_rgb(150, 150, 150)),
                    );

                    let mut hex_parts = Vec::new();
                    let mut ascii = String::new();

                    for col in 0..16 {
                        let idx = row * 16 + col;
                        if idx >= self.memory_viewer_data.len() {
                            break;
                        }

                        let byte = self.memory_viewer_data[idx];
                        if col == 8 {
                            hex_parts.push(" ".to_string());
                        }
                        hex_parts.push(format!("{:02X}", byte));

                        let c = if byte >= 0x20 && byte <= 0x7E { byte as char } else { '.' };
                        ascii.push(c);
                    }

                    let hex_str = hex_parts.join(" ");
                    ui.monospace(hex_str);

                    ui.label(
                        egui::RichText::new(ascii)
                            .monospace()
                            .color(egui::Color32::from_rgb(180, 180, 180)),
                    );

                    ui.end_row();
                }
            });
    }

    fn refresh_memory_view(&mut self) {
        self.memory_viewer_data.clear();
        self.memory_viewer_data.reserve(256);

        let bus = &self.nemu.bus;
        for i in 0..256 {
            let addr = self.memory_viewer_addr.wrapping_add(i as u16);
            self.memory_viewer_data.push(bus.read_debug(addr));
        }
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
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
            }

            self.update_disassembly();
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

        egui::Window::new("CPU")
            .default_size([240.0, 300.0])
            .show(ctx, |ui| {
                self.render_cpu_window(ui);
            });

        egui::Window::new("Disassembly")
            .default_pos([17.0, 280.0])
            .default_size([240.0, 300.0])
            .show(ctx, |ui| {
                self.render_disassembly(ui);
            });

        egui::Window::new("Screen")
            .default_pos([290.0, 55.0])
            .show(ctx, |ui| {
                ui.image(egui::ImageSource::Texture(egui::load::SizedTexture {
                    id: self.screen_tex.id(),
                    size: egui::vec2(WIDTH as f32 * 2.5, HEIGHT as f32 * 2.5),
                }));
            });

        egui::Window::new("Memory Viewer")
            .default_pos([725.0, 55.0])
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                self.render_memory_viewer(ui);
            });
    }
}
