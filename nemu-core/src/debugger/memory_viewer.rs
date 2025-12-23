use eframe::egui;

pub(super) struct MemoryViewer {
    memory_viewer_addr: u16,
    memory_viewer_addr_input: String,
    memory_viewer_data: Vec<u8>,
}

impl MemoryViewer {
    pub(super) fn new() -> Self {
        Self {
            memory_viewer_addr: 0,
            memory_viewer_addr_input: String::from("0000"),
            memory_viewer_data: vec![0; 256],
        }
    }

    pub(super) fn render_memory_viewer(&mut self, ui: &mut egui::Ui, nemu: &crate::Nemu) {
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
                    self.refresh_memory_view(&nemu.bus);
                }
            }

            ui.separator();

            if ui.button("Jump to PC").clicked() {
                self.memory_viewer_addr = nemu.cpu.regs.pc;
                self.refresh_memory_view(&nemu.bus);
            }

            if ui.button("Jump to SP").clicked() {
                self.memory_viewer_addr = nemu.cpu.regs.sp;
                self.refresh_memory_view(&nemu.bus);
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

    pub(super) fn refresh_memory_view(&mut self, bus: &crate::bus::Bus) {
        self.memory_viewer_data.clear();
        self.memory_viewer_data.reserve(256);

        for i in 0..256 {
            let addr = self.memory_viewer_addr.wrapping_add(i as u16);
            self.memory_viewer_data.push(bus.peek(addr));
        }
    }
}