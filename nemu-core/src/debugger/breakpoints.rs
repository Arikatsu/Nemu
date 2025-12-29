use eframe::egui;
use std::collections::HashSet;

pub(super) struct Breakpoints {
    addresses: HashSet<u16>,
    addr_input: String,
}

impl Breakpoints {
    pub(super) fn new() -> Self {
        Self {
            addresses: HashSet::new(),
            addr_input: String::from("0000"),
        }
    }

    pub(super) fn add_breakpoint(&mut self, addr: u16) {
        self.addresses.insert(addr);
    }

    pub(super) fn remove_breakpoint(&mut self, addr: u16) {
        self.addresses.remove(&addr);
    }

    pub(super) fn is_breakpoint(&self, addr: u16) -> bool {
        self.addresses.contains(&addr)
    }

    pub(super) fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Address:");

            ui.add(
                egui::TextEdit::singleline(&mut self.addr_input)
                    .desired_width(80.0)
                    .font(egui::TextStyle::Monospace),
            );

            if ui.button("+ Add").clicked() {
                if let Ok(addr) = u16::from_str_radix(&self.addr_input, 16) {
                    self.add_breakpoint(addr);
                }
            }
        });

        ui.separator();
        ui.add_space(8.0);

        let mut remove = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.addresses.is_empty() {
                ui.weak("No breakpoints");
            } else {
                for &addr in &self.addresses {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        ui.monospace(format!("0x{:04X}", addr));

                        if ui.small_button("Remove").clicked() {
                            remove = Some(addr);
                        }
                    });
                }
            }
        });

        if let Some(addr) = remove {
            self.remove_breakpoint(addr);
        }
    }

}