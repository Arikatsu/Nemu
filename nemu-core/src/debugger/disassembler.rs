use super::Debugger;
use crate::bus::Bus;

use eframe::egui;
use std::time::Instant;

const DISASM_WINDOW_SIZE: usize = 30;
const DISASM_MAX_BYTES: usize = 0x100;
const DISASM_REFRESH_MS: u64 = 150;

impl Debugger {
    fn disassemble(info: &OpcodeInfo, bus: &Bus, pc: u16) -> String {
        let next_pc = pc.wrapping_add(info.length as u16);

        let result = match info.operand {
            Operand::None => String::from(info.mnemonic_prefix),
            Operand::U8 => {
                let val = bus.read_debug(pc + 1);
                format!(
                    "{}${:02X}{}",
                    info.mnemonic_prefix, val, info.mnemonic_suffix
                )
            }
            Operand::U16 => {
                let lo = bus.read_debug(pc + 1);
                let hi = bus.read_debug(pc + 2);
                let val = (hi as u16) << 8 | (lo as u16);
                format!(
                    "{}${:04X}{}",
                    info.mnemonic_prefix, val, info.mnemonic_suffix
                )
            }
            Operand::I8 => {
                let offset = bus.read_debug(pc + 1) as i8;
                let target = (next_pc as i32 + offset as i32) as u16;
                format!(
                    "{}${:04X}{}",
                    info.mnemonic_prefix, target, info.mnemonic_suffix
                )
            }
        };

        result
    }

    fn needs_disassemble_rebuild(&self) -> bool {
        let pc = self.nemu.cpu.regs.pc;
        let first_addr = match self.disasm_lines.first() {
            Some((addr, _, _)) => *addr,
            None => return true,
        };
        let last_addr = match self.disasm_lines.last() {
            Some((addr, _, _)) => *addr,
            None => return true,
        };

        pc < first_addr || pc > last_addr
    }

    fn rebuild_disassembly_window(&mut self) {
        self.disasm_lines.clear();

        let mut addr = self.nemu.cpu.regs.pc;
        let mut bytes_consumed = 0;

        while self.disasm_lines.len() < DISASM_WINDOW_SIZE && bytes_consumed < DISASM_MAX_BYTES {
            let opcode = self.nemu.bus.read_debug(addr);
            let info = if opcode == 0xCB {
                let cb_opcode = self.nemu.bus.read_debug(addr.wrapping_add(1));
                &CB_OPCODES[cb_opcode as usize]
            } else {
                &OPCODES[opcode as usize]
            };
            let len = info.length.max(1) as u16;

            let bytes_str = (0..len)
                .map(|i| format!("{:02X}", self.nemu.bus.read_debug(addr.wrapping_add(i))))
                .fold(String::new(), |mut acc, s| {
                    if !acc.is_empty() {
                        acc.push(' ');
                    }
                    acc.push_str(&s);
                    acc
                });

            let instruction = Self::disassemble(info, &self.nemu.bus, addr);

            self.disasm_lines.push((addr, bytes_str, instruction));

            addr = addr.wrapping_add(len);
            bytes_consumed += len as usize;
        }

        self.disasm_base_pc = self.nemu.cpu.regs.pc;
    }

    pub(crate) fn update_disassembly(&mut self) {
        if !self.running {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_disasm_update)
            < std::time::Duration::from_millis(DISASM_REFRESH_MS)
        {
            return;
        }

        self.rebuild_disassembly_window();
        self.last_disasm_update = now;
    }

    pub(crate) fn render_disassembly(&mut self, ui: &mut egui::Ui) {
        if !self.running && self.needs_disassemble_rebuild() {
            self.rebuild_disassembly_window();
        }

        let pc = self.nemu.cpu.regs.pc;

        egui::ScrollArea::vertical()
            .id_salt("disassembly_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("disassembly")
                    .spacing([12.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.heading("Addr");
                        ui.heading("Bytes");
                        ui.heading("Instruction");
                        ui.end_row();

                        for (addr, bytes, text) in &self.disasm_lines {
                            let is_current = *addr == pc;

                            if is_current {
                                let highlight_color = egui::Color32::from_rgb(50, 150, 50);
                                ui.colored_label(highlight_color, egui::RichText::new(format!("{:04X}", addr)).monospace());
                                ui.colored_label(highlight_color, egui::RichText::new(bytes).monospace());
                                ui.colored_label(highlight_color, egui::RichText::new(text).monospace());
                            } else {
                                ui.monospace(format!("{:04X}", addr));
                                ui.monospace(bytes);
                                ui.monospace(text);
                            }

                            ui.end_row();
                        }
                    });
            });
    }
}

// --- Code below is auto-generated. ---

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    None,
    U8,
    U16,
    I8,
}

#[derive(Clone, Copy, Debug)]
pub struct OpcodeInfo {
    pub mnemonic_prefix: &'static str,
    pub mnemonic_suffix: &'static str,
    pub length: u8,
    pub operand: Operand,
}

pub static OPCODES: [OpcodeInfo; 256] = [
    // 0x00
    OpcodeInfo {
        mnemonic_prefix: "NOP",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x01
    OpcodeInfo {
        mnemonic_prefix: "LD BC, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0x02
    OpcodeInfo {
        mnemonic_prefix: "LD (BC), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x03
    OpcodeInfo {
        mnemonic_prefix: "INC BC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x04
    OpcodeInfo {
        mnemonic_prefix: "INC B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x05
    OpcodeInfo {
        mnemonic_prefix: "DEC B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x06
    OpcodeInfo {
        mnemonic_prefix: "LD B, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x07
    OpcodeInfo {
        mnemonic_prefix: "RLCA",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x08
    OpcodeInfo {
        mnemonic_prefix: "LD (",
        mnemonic_suffix: "),SP",
        length: 3,
        operand: Operand::U16,
    },
    // 0x09
    OpcodeInfo {
        mnemonic_prefix: "ADD HL, BC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x0A
    OpcodeInfo {
        mnemonic_prefix: "LD A, (BC)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x0B
    OpcodeInfo {
        mnemonic_prefix: "DEC BC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x0C
    OpcodeInfo {
        mnemonic_prefix: "INC C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x0D
    OpcodeInfo {
        mnemonic_prefix: "DEC C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x0E
    OpcodeInfo {
        mnemonic_prefix: "LD C, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x0F
    OpcodeInfo {
        mnemonic_prefix: "RRCA",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x10
    OpcodeInfo {
        mnemonic_prefix: "STOP",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0x11
    OpcodeInfo {
        mnemonic_prefix: "LD DE, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0x12
    OpcodeInfo {
        mnemonic_prefix: "LD (DE), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x13
    OpcodeInfo {
        mnemonic_prefix: "INC DE",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x14
    OpcodeInfo {
        mnemonic_prefix: "INC D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x15
    OpcodeInfo {
        mnemonic_prefix: "DEC D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x16
    OpcodeInfo {
        mnemonic_prefix: "LD D, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x17
    OpcodeInfo {
        mnemonic_prefix: "RLA",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x18
    OpcodeInfo {
        mnemonic_prefix: "JR ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0x19
    OpcodeInfo {
        mnemonic_prefix: "ADD HL, DE",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x1A
    OpcodeInfo {
        mnemonic_prefix: "LD A, (DE)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x1B
    OpcodeInfo {
        mnemonic_prefix: "DEC DE",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x1C
    OpcodeInfo {
        mnemonic_prefix: "INC E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x1D
    OpcodeInfo {
        mnemonic_prefix: "DEC E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x1E
    OpcodeInfo {
        mnemonic_prefix: "LD E, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x1F
    OpcodeInfo {
        mnemonic_prefix: "RRA",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x20
    OpcodeInfo {
        mnemonic_prefix: "JR NZ, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0x21
    OpcodeInfo {
        mnemonic_prefix: "LD HL, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0x22
    OpcodeInfo {
        mnemonic_prefix: "LD (HL+), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x23
    OpcodeInfo {
        mnemonic_prefix: "INC HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x24
    OpcodeInfo {
        mnemonic_prefix: "INC H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x25
    OpcodeInfo {
        mnemonic_prefix: "DEC H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x26
    OpcodeInfo {
        mnemonic_prefix: "LD H, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x27
    OpcodeInfo {
        mnemonic_prefix: "DAA",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x28
    OpcodeInfo {
        mnemonic_prefix: "JR Z, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0x29
    OpcodeInfo {
        mnemonic_prefix: "ADD HL, HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x2A
    OpcodeInfo {
        mnemonic_prefix: "LD A, (HL+)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x2B
    OpcodeInfo {
        mnemonic_prefix: "DEC HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x2C
    OpcodeInfo {
        mnemonic_prefix: "INC L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x2D
    OpcodeInfo {
        mnemonic_prefix: "DEC L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x2E
    OpcodeInfo {
        mnemonic_prefix: "LD L, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x2F
    OpcodeInfo {
        mnemonic_prefix: "CPL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x30
    OpcodeInfo {
        mnemonic_prefix: "JR NC, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0x31
    OpcodeInfo {
        mnemonic_prefix: "LD SP, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0x32
    OpcodeInfo {
        mnemonic_prefix: "LD (HL-), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x33
    OpcodeInfo {
        mnemonic_prefix: "INC SP",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x34
    OpcodeInfo {
        mnemonic_prefix: "INC (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x35
    OpcodeInfo {
        mnemonic_prefix: "DEC (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x36
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x37
    OpcodeInfo {
        mnemonic_prefix: "SCF",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x38
    OpcodeInfo {
        mnemonic_prefix: "JR C, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0x39
    OpcodeInfo {
        mnemonic_prefix: "ADD HL, SP",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x3A
    OpcodeInfo {
        mnemonic_prefix: "LD A, (HL-)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x3B
    OpcodeInfo {
        mnemonic_prefix: "DEC SP",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x3C
    OpcodeInfo {
        mnemonic_prefix: "INC A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x3D
    OpcodeInfo {
        mnemonic_prefix: "DEC A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x3E
    OpcodeInfo {
        mnemonic_prefix: "LD A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0x3F
    OpcodeInfo {
        mnemonic_prefix: "CCF",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x40
    OpcodeInfo {
        mnemonic_prefix: "LD B, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x41
    OpcodeInfo {
        mnemonic_prefix: "LD B, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x42
    OpcodeInfo {
        mnemonic_prefix: "LD B, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x43
    OpcodeInfo {
        mnemonic_prefix: "LD B, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x44
    OpcodeInfo {
        mnemonic_prefix: "LD B, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x45
    OpcodeInfo {
        mnemonic_prefix: "LD B, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x46
    OpcodeInfo {
        mnemonic_prefix: "LD B, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x47
    OpcodeInfo {
        mnemonic_prefix: "LD B, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x48
    OpcodeInfo {
        mnemonic_prefix: "LD C, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x49
    OpcodeInfo {
        mnemonic_prefix: "LD C, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4A
    OpcodeInfo {
        mnemonic_prefix: "LD C, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4B
    OpcodeInfo {
        mnemonic_prefix: "LD C, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4C
    OpcodeInfo {
        mnemonic_prefix: "LD C, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4D
    OpcodeInfo {
        mnemonic_prefix: "LD C, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4E
    OpcodeInfo {
        mnemonic_prefix: "LD C, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x4F
    OpcodeInfo {
        mnemonic_prefix: "LD C, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x50
    OpcodeInfo {
        mnemonic_prefix: "LD D, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x51
    OpcodeInfo {
        mnemonic_prefix: "LD D, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x52
    OpcodeInfo {
        mnemonic_prefix: "LD D, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x53
    OpcodeInfo {
        mnemonic_prefix: "LD D, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x54
    OpcodeInfo {
        mnemonic_prefix: "LD D, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x55
    OpcodeInfo {
        mnemonic_prefix: "LD D, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x56
    OpcodeInfo {
        mnemonic_prefix: "LD D, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x57
    OpcodeInfo {
        mnemonic_prefix: "LD D, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x58
    OpcodeInfo {
        mnemonic_prefix: "LD E, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x59
    OpcodeInfo {
        mnemonic_prefix: "LD E, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5A
    OpcodeInfo {
        mnemonic_prefix: "LD E, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5B
    OpcodeInfo {
        mnemonic_prefix: "LD E, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5C
    OpcodeInfo {
        mnemonic_prefix: "LD E, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5D
    OpcodeInfo {
        mnemonic_prefix: "LD E, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5E
    OpcodeInfo {
        mnemonic_prefix: "LD E, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x5F
    OpcodeInfo {
        mnemonic_prefix: "LD E, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x60
    OpcodeInfo {
        mnemonic_prefix: "LD H, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x61
    OpcodeInfo {
        mnemonic_prefix: "LD H, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x62
    OpcodeInfo {
        mnemonic_prefix: "LD H, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x63
    OpcodeInfo {
        mnemonic_prefix: "LD H, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x64
    OpcodeInfo {
        mnemonic_prefix: "LD H, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x65
    OpcodeInfo {
        mnemonic_prefix: "LD H, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x66
    OpcodeInfo {
        mnemonic_prefix: "LD H, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x67
    OpcodeInfo {
        mnemonic_prefix: "LD H, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x68
    OpcodeInfo {
        mnemonic_prefix: "LD L, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x69
    OpcodeInfo {
        mnemonic_prefix: "LD L, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6A
    OpcodeInfo {
        mnemonic_prefix: "LD L, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6B
    OpcodeInfo {
        mnemonic_prefix: "LD L, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6C
    OpcodeInfo {
        mnemonic_prefix: "LD L, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6D
    OpcodeInfo {
        mnemonic_prefix: "LD L, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6E
    OpcodeInfo {
        mnemonic_prefix: "LD L, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x6F
    OpcodeInfo {
        mnemonic_prefix: "LD L, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x70
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x71
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x72
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x73
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x74
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x75
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x76
    OpcodeInfo {
        mnemonic_prefix: "HALT",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x77
    OpcodeInfo {
        mnemonic_prefix: "LD (HL), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x78
    OpcodeInfo {
        mnemonic_prefix: "LD A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x79
    OpcodeInfo {
        mnemonic_prefix: "LD A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7A
    OpcodeInfo {
        mnemonic_prefix: "LD A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7B
    OpcodeInfo {
        mnemonic_prefix: "LD A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7C
    OpcodeInfo {
        mnemonic_prefix: "LD A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7D
    OpcodeInfo {
        mnemonic_prefix: "LD A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7E
    OpcodeInfo {
        mnemonic_prefix: "LD A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x7F
    OpcodeInfo {
        mnemonic_prefix: "LD A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x80
    OpcodeInfo {
        mnemonic_prefix: "ADD A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x81
    OpcodeInfo {
        mnemonic_prefix: "ADD A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x82
    OpcodeInfo {
        mnemonic_prefix: "ADD A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x83
    OpcodeInfo {
        mnemonic_prefix: "ADD A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x84
    OpcodeInfo {
        mnemonic_prefix: "ADD A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x85
    OpcodeInfo {
        mnemonic_prefix: "ADD A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x86
    OpcodeInfo {
        mnemonic_prefix: "ADD A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x87
    OpcodeInfo {
        mnemonic_prefix: "ADD A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x88
    OpcodeInfo {
        mnemonic_prefix: "ADC A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x89
    OpcodeInfo {
        mnemonic_prefix: "ADC A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8A
    OpcodeInfo {
        mnemonic_prefix: "ADC A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8B
    OpcodeInfo {
        mnemonic_prefix: "ADC A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8C
    OpcodeInfo {
        mnemonic_prefix: "ADC A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8D
    OpcodeInfo {
        mnemonic_prefix: "ADC A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8E
    OpcodeInfo {
        mnemonic_prefix: "ADC A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x8F
    OpcodeInfo {
        mnemonic_prefix: "ADC A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x90
    OpcodeInfo {
        mnemonic_prefix: "SUB A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x91
    OpcodeInfo {
        mnemonic_prefix: "SUB A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x92
    OpcodeInfo {
        mnemonic_prefix: "SUB A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x93
    OpcodeInfo {
        mnemonic_prefix: "SUB A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x94
    OpcodeInfo {
        mnemonic_prefix: "SUB A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x95
    OpcodeInfo {
        mnemonic_prefix: "SUB A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x96
    OpcodeInfo {
        mnemonic_prefix: "SUB A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x97
    OpcodeInfo {
        mnemonic_prefix: "SUB A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x98
    OpcodeInfo {
        mnemonic_prefix: "SBC A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x99
    OpcodeInfo {
        mnemonic_prefix: "SBC A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9A
    OpcodeInfo {
        mnemonic_prefix: "SBC A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9B
    OpcodeInfo {
        mnemonic_prefix: "SBC A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9C
    OpcodeInfo {
        mnemonic_prefix: "SBC A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9D
    OpcodeInfo {
        mnemonic_prefix: "SBC A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9E
    OpcodeInfo {
        mnemonic_prefix: "SBC A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0x9F
    OpcodeInfo {
        mnemonic_prefix: "SBC A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA0
    OpcodeInfo {
        mnemonic_prefix: "AND A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA1
    OpcodeInfo {
        mnemonic_prefix: "AND A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA2
    OpcodeInfo {
        mnemonic_prefix: "AND A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA3
    OpcodeInfo {
        mnemonic_prefix: "AND A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA4
    OpcodeInfo {
        mnemonic_prefix: "AND A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA5
    OpcodeInfo {
        mnemonic_prefix: "AND A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA6
    OpcodeInfo {
        mnemonic_prefix: "AND A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA7
    OpcodeInfo {
        mnemonic_prefix: "AND A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA8
    OpcodeInfo {
        mnemonic_prefix: "XOR A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xA9
    OpcodeInfo {
        mnemonic_prefix: "XOR A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAA
    OpcodeInfo {
        mnemonic_prefix: "XOR A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAB
    OpcodeInfo {
        mnemonic_prefix: "XOR A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAC
    OpcodeInfo {
        mnemonic_prefix: "XOR A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAD
    OpcodeInfo {
        mnemonic_prefix: "XOR A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAE
    OpcodeInfo {
        mnemonic_prefix: "XOR A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xAF
    OpcodeInfo {
        mnemonic_prefix: "XOR A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB0
    OpcodeInfo {
        mnemonic_prefix: "OR A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB1
    OpcodeInfo {
        mnemonic_prefix: "OR A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB2
    OpcodeInfo {
        mnemonic_prefix: "OR A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB3
    OpcodeInfo {
        mnemonic_prefix: "OR A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB4
    OpcodeInfo {
        mnemonic_prefix: "OR A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB5
    OpcodeInfo {
        mnemonic_prefix: "OR A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB6
    OpcodeInfo {
        mnemonic_prefix: "OR A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB7
    OpcodeInfo {
        mnemonic_prefix: "OR A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB8
    OpcodeInfo {
        mnemonic_prefix: "CP A, B",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xB9
    OpcodeInfo {
        mnemonic_prefix: "CP A, C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBA
    OpcodeInfo {
        mnemonic_prefix: "CP A, D",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBB
    OpcodeInfo {
        mnemonic_prefix: "CP A, E",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBC
    OpcodeInfo {
        mnemonic_prefix: "CP A, H",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBD
    OpcodeInfo {
        mnemonic_prefix: "CP A, L",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBE
    OpcodeInfo {
        mnemonic_prefix: "CP A, (HL)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xBF
    OpcodeInfo {
        mnemonic_prefix: "CP A, A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC0
    OpcodeInfo {
        mnemonic_prefix: "RET NZ",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC1
    OpcodeInfo {
        mnemonic_prefix: "POP BC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC2
    OpcodeInfo {
        mnemonic_prefix: "JP NZ, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xC3
    OpcodeInfo {
        mnemonic_prefix: "JP ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xC4
    OpcodeInfo {
        mnemonic_prefix: "CALL NZ, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xC5
    OpcodeInfo {
        mnemonic_prefix: "PUSH BC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC6
    OpcodeInfo {
        mnemonic_prefix: "ADD A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xC7
    OpcodeInfo {
        mnemonic_prefix: "RST 00h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC8
    OpcodeInfo {
        mnemonic_prefix: "RET Z",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xC9
    OpcodeInfo {
        mnemonic_prefix: "RET",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xCA
    OpcodeInfo {
        mnemonic_prefix: "JP Z, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xCB
    OpcodeInfo {
        mnemonic_prefix: "PREFIX CB",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xCC
    OpcodeInfo {
        mnemonic_prefix: "CALL Z, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xCD
    OpcodeInfo {
        mnemonic_prefix: "CALL ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xCE
    OpcodeInfo {
        mnemonic_prefix: "ADC A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xCF
    OpcodeInfo {
        mnemonic_prefix: "RST 08h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD0
    OpcodeInfo {
        mnemonic_prefix: "RET NC",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD1
    OpcodeInfo {
        mnemonic_prefix: "POP DE",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD2
    OpcodeInfo {
        mnemonic_prefix: "JP NC, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xD3
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD4
    OpcodeInfo {
        mnemonic_prefix: "CALL NC, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xD5
    OpcodeInfo {
        mnemonic_prefix: "PUSH DE",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD6
    OpcodeInfo {
        mnemonic_prefix: "SUB A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xD7
    OpcodeInfo {
        mnemonic_prefix: "RST 10h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD8
    OpcodeInfo {
        mnemonic_prefix: "RET C",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xD9
    OpcodeInfo {
        mnemonic_prefix: "RETI",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xDA
    OpcodeInfo {
        mnemonic_prefix: "JP C, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xDB
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xDC
    OpcodeInfo {
        mnemonic_prefix: "CALL C, ",
        mnemonic_suffix: "",
        length: 3,
        operand: Operand::U16,
    },
    // 0xDD
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xDE
    OpcodeInfo {
        mnemonic_prefix: "SBC A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xDF
    OpcodeInfo {
        mnemonic_prefix: "RST 18h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE0
    OpcodeInfo {
        mnemonic_prefix: "LD (",
        mnemonic_suffix: "),A",
        length: 2,
        operand: Operand::U8,
    },
    // 0xE1
    OpcodeInfo {
        mnemonic_prefix: "POP HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE2
    OpcodeInfo {
        mnemonic_prefix: "LD (FF00+C), A",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE3
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE4
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE5
    OpcodeInfo {
        mnemonic_prefix: "PUSH HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE6
    OpcodeInfo {
        mnemonic_prefix: "AND A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xE7
    OpcodeInfo {
        mnemonic_prefix: "RST 20h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xE8
    OpcodeInfo {
        mnemonic_prefix: "ADD SP, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0xE9
    OpcodeInfo {
        mnemonic_prefix: "JP HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xEA
    OpcodeInfo {
        mnemonic_prefix: "LD (",
        mnemonic_suffix: "),A",
        length: 3,
        operand: Operand::U16,
    },
    // 0xEB
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xEC
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xED
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xEE
    OpcodeInfo {
        mnemonic_prefix: "XOR A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xEF
    OpcodeInfo {
        mnemonic_prefix: "RST 28h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF0
    OpcodeInfo {
        mnemonic_prefix: "LD A, (",
        mnemonic_suffix: ")",
        length: 2,
        operand: Operand::U8,
    },
    // 0xF1
    OpcodeInfo {
        mnemonic_prefix: "POP AF",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF2
    OpcodeInfo {
        mnemonic_prefix: "LD A, (FF00+C)",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF3
    OpcodeInfo {
        mnemonic_prefix: "DI",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF4
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF5
    OpcodeInfo {
        mnemonic_prefix: "PUSH AF",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF6
    OpcodeInfo {
        mnemonic_prefix: "OR A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xF7
    OpcodeInfo {
        mnemonic_prefix: "RST 30h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xF8
    OpcodeInfo {
        mnemonic_prefix: "LD HL, SP+",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::I8,
    },
    // 0xF9
    OpcodeInfo {
        mnemonic_prefix: "LD SP, HL",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xFA
    OpcodeInfo {
        mnemonic_prefix: "LD A, (",
        mnemonic_suffix: ")",
        length: 3,
        operand: Operand::U16,
    },
    // 0xFB
    OpcodeInfo {
        mnemonic_prefix: "EI",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xFC
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xFD
    OpcodeInfo {
        mnemonic_prefix: "UNUSED",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
    // 0xFE
    OpcodeInfo {
        mnemonic_prefix: "CP A, ",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::U8,
    },
    // 0xFF
    OpcodeInfo {
        mnemonic_prefix: "RST 38h",
        mnemonic_suffix: "",
        length: 1,
        operand: Operand::None,
    },
];

pub static CB_OPCODES: [OpcodeInfo; 256] = [
    // 0xCB00
    OpcodeInfo {
        mnemonic_prefix: "RLC B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB01
    OpcodeInfo {
        mnemonic_prefix: "RLC C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB02
    OpcodeInfo {
        mnemonic_prefix: "RLC D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB03
    OpcodeInfo {
        mnemonic_prefix: "RLC E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB04
    OpcodeInfo {
        mnemonic_prefix: "RLC H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB05
    OpcodeInfo {
        mnemonic_prefix: "RLC L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB06
    OpcodeInfo {
        mnemonic_prefix: "RLC (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB07
    OpcodeInfo {
        mnemonic_prefix: "RLC A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB08
    OpcodeInfo {
        mnemonic_prefix: "RRC B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB09
    OpcodeInfo {
        mnemonic_prefix: "RRC C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0A
    OpcodeInfo {
        mnemonic_prefix: "RRC D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0B
    OpcodeInfo {
        mnemonic_prefix: "RRC E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0C
    OpcodeInfo {
        mnemonic_prefix: "RRC H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0D
    OpcodeInfo {
        mnemonic_prefix: "RRC L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0E
    OpcodeInfo {
        mnemonic_prefix: "RRC (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB0F
    OpcodeInfo {
        mnemonic_prefix: "RRC A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB10
    OpcodeInfo {
        mnemonic_prefix: "RL B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB11
    OpcodeInfo {
        mnemonic_prefix: "RL C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB12
    OpcodeInfo {
        mnemonic_prefix: "RL D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB13
    OpcodeInfo {
        mnemonic_prefix: "RL E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB14
    OpcodeInfo {
        mnemonic_prefix: "RL H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB15
    OpcodeInfo {
        mnemonic_prefix: "RL L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB16
    OpcodeInfo {
        mnemonic_prefix: "RL (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB17
    OpcodeInfo {
        mnemonic_prefix: "RL A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB18
    OpcodeInfo {
        mnemonic_prefix: "RR B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB19
    OpcodeInfo {
        mnemonic_prefix: "RR C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1A
    OpcodeInfo {
        mnemonic_prefix: "RR D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1B
    OpcodeInfo {
        mnemonic_prefix: "RR E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1C
    OpcodeInfo {
        mnemonic_prefix: "RR H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1D
    OpcodeInfo {
        mnemonic_prefix: "RR L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1E
    OpcodeInfo {
        mnemonic_prefix: "RR (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB1F
    OpcodeInfo {
        mnemonic_prefix: "RR A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB20
    OpcodeInfo {
        mnemonic_prefix: "SLA B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB21
    OpcodeInfo {
        mnemonic_prefix: "SLA C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB22
    OpcodeInfo {
        mnemonic_prefix: "SLA D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB23
    OpcodeInfo {
        mnemonic_prefix: "SLA E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB24
    OpcodeInfo {
        mnemonic_prefix: "SLA H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB25
    OpcodeInfo {
        mnemonic_prefix: "SLA L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB26
    OpcodeInfo {
        mnemonic_prefix: "SLA (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB27
    OpcodeInfo {
        mnemonic_prefix: "SLA A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB28
    OpcodeInfo {
        mnemonic_prefix: "SRA B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB29
    OpcodeInfo {
        mnemonic_prefix: "SRA C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2A
    OpcodeInfo {
        mnemonic_prefix: "SRA D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2B
    OpcodeInfo {
        mnemonic_prefix: "SRA E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2C
    OpcodeInfo {
        mnemonic_prefix: "SRA H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2D
    OpcodeInfo {
        mnemonic_prefix: "SRA L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2E
    OpcodeInfo {
        mnemonic_prefix: "SRA (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB2F
    OpcodeInfo {
        mnemonic_prefix: "SRA A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB30
    OpcodeInfo {
        mnemonic_prefix: "SWAP B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB31
    OpcodeInfo {
        mnemonic_prefix: "SWAP C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB32
    OpcodeInfo {
        mnemonic_prefix: "SWAP D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB33
    OpcodeInfo {
        mnemonic_prefix: "SWAP E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB34
    OpcodeInfo {
        mnemonic_prefix: "SWAP H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB35
    OpcodeInfo {
        mnemonic_prefix: "SWAP L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB36
    OpcodeInfo {
        mnemonic_prefix: "SWAP (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB37
    OpcodeInfo {
        mnemonic_prefix: "SWAP A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB38
    OpcodeInfo {
        mnemonic_prefix: "SRL B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB39
    OpcodeInfo {
        mnemonic_prefix: "SRL C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3A
    OpcodeInfo {
        mnemonic_prefix: "SRL D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3B
    OpcodeInfo {
        mnemonic_prefix: "SRL E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3C
    OpcodeInfo {
        mnemonic_prefix: "SRL H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3D
    OpcodeInfo {
        mnemonic_prefix: "SRL L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3E
    OpcodeInfo {
        mnemonic_prefix: "SRL (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB3F
    OpcodeInfo {
        mnemonic_prefix: "SRL A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB40
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB41
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB42
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB43
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB44
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB45
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB46
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB47
    OpcodeInfo {
        mnemonic_prefix: "BIT 0, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB48
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB49
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4A
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4B
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4C
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4D
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4E
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB4F
    OpcodeInfo {
        mnemonic_prefix: "BIT 1, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB50
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB51
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB52
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB53
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB54
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB55
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB56
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB57
    OpcodeInfo {
        mnemonic_prefix: "BIT 2, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB58
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB59
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5A
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5B
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5C
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5D
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5E
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB5F
    OpcodeInfo {
        mnemonic_prefix: "BIT 3, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB60
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB61
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB62
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB63
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB64
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB65
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB66
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB67
    OpcodeInfo {
        mnemonic_prefix: "BIT 4, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB68
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB69
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6A
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6B
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6C
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6D
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6E
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB6F
    OpcodeInfo {
        mnemonic_prefix: "BIT 5, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB70
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB71
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB72
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB73
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB74
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB75
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB76
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB77
    OpcodeInfo {
        mnemonic_prefix: "BIT 6, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB78
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB79
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7A
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7B
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7C
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7D
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7E
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB7F
    OpcodeInfo {
        mnemonic_prefix: "BIT 7, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB80
    OpcodeInfo {
        mnemonic_prefix: "RES 0, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB81
    OpcodeInfo {
        mnemonic_prefix: "RES 0, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB82
    OpcodeInfo {
        mnemonic_prefix: "RES 0, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB83
    OpcodeInfo {
        mnemonic_prefix: "RES 0, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB84
    OpcodeInfo {
        mnemonic_prefix: "RES 0, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB85
    OpcodeInfo {
        mnemonic_prefix: "RES 0, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB86
    OpcodeInfo {
        mnemonic_prefix: "RES 0, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB87
    OpcodeInfo {
        mnemonic_prefix: "RES 0, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB88
    OpcodeInfo {
        mnemonic_prefix: "RES 1, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB89
    OpcodeInfo {
        mnemonic_prefix: "RES 1, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8A
    OpcodeInfo {
        mnemonic_prefix: "RES 1, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8B
    OpcodeInfo {
        mnemonic_prefix: "RES 1, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8C
    OpcodeInfo {
        mnemonic_prefix: "RES 1, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8D
    OpcodeInfo {
        mnemonic_prefix: "RES 1, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8E
    OpcodeInfo {
        mnemonic_prefix: "RES 1, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB8F
    OpcodeInfo {
        mnemonic_prefix: "RES 1, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB90
    OpcodeInfo {
        mnemonic_prefix: "RES 2, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB91
    OpcodeInfo {
        mnemonic_prefix: "RES 2, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB92
    OpcodeInfo {
        mnemonic_prefix: "RES 2, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB93
    OpcodeInfo {
        mnemonic_prefix: "RES 2, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB94
    OpcodeInfo {
        mnemonic_prefix: "RES 2, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB95
    OpcodeInfo {
        mnemonic_prefix: "RES 2, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB96
    OpcodeInfo {
        mnemonic_prefix: "RES 2, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB97
    OpcodeInfo {
        mnemonic_prefix: "RES 2, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB98
    OpcodeInfo {
        mnemonic_prefix: "RES 3, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB99
    OpcodeInfo {
        mnemonic_prefix: "RES 3, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9A
    OpcodeInfo {
        mnemonic_prefix: "RES 3, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9B
    OpcodeInfo {
        mnemonic_prefix: "RES 3, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9C
    OpcodeInfo {
        mnemonic_prefix: "RES 3, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9D
    OpcodeInfo {
        mnemonic_prefix: "RES 3, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9E
    OpcodeInfo {
        mnemonic_prefix: "RES 3, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCB9F
    OpcodeInfo {
        mnemonic_prefix: "RES 3, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA0
    OpcodeInfo {
        mnemonic_prefix: "RES 4, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA1
    OpcodeInfo {
        mnemonic_prefix: "RES 4, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA2
    OpcodeInfo {
        mnemonic_prefix: "RES 4, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA3
    OpcodeInfo {
        mnemonic_prefix: "RES 4, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA4
    OpcodeInfo {
        mnemonic_prefix: "RES 4, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA5
    OpcodeInfo {
        mnemonic_prefix: "RES 4, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA6
    OpcodeInfo {
        mnemonic_prefix: "RES 4, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA7
    OpcodeInfo {
        mnemonic_prefix: "RES 4, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA8
    OpcodeInfo {
        mnemonic_prefix: "RES 5, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBA9
    OpcodeInfo {
        mnemonic_prefix: "RES 5, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAA
    OpcodeInfo {
        mnemonic_prefix: "RES 5, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAB
    OpcodeInfo {
        mnemonic_prefix: "RES 5, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAC
    OpcodeInfo {
        mnemonic_prefix: "RES 5, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAD
    OpcodeInfo {
        mnemonic_prefix: "RES 5, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAE
    OpcodeInfo {
        mnemonic_prefix: "RES 5, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBAF
    OpcodeInfo {
        mnemonic_prefix: "RES 5, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB0
    OpcodeInfo {
        mnemonic_prefix: "RES 6, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB1
    OpcodeInfo {
        mnemonic_prefix: "RES 6, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB2
    OpcodeInfo {
        mnemonic_prefix: "RES 6, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB3
    OpcodeInfo {
        mnemonic_prefix: "RES 6, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB4
    OpcodeInfo {
        mnemonic_prefix: "RES 6, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB5
    OpcodeInfo {
        mnemonic_prefix: "RES 6, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB6
    OpcodeInfo {
        mnemonic_prefix: "RES 6, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB7
    OpcodeInfo {
        mnemonic_prefix: "RES 6, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB8
    OpcodeInfo {
        mnemonic_prefix: "RES 7, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBB9
    OpcodeInfo {
        mnemonic_prefix: "RES 7, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBA
    OpcodeInfo {
        mnemonic_prefix: "RES 7, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBB
    OpcodeInfo {
        mnemonic_prefix: "RES 7, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBC
    OpcodeInfo {
        mnemonic_prefix: "RES 7, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBD
    OpcodeInfo {
        mnemonic_prefix: "RES 7, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBE
    OpcodeInfo {
        mnemonic_prefix: "RES 7, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBBF
    OpcodeInfo {
        mnemonic_prefix: "RES 7, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC0
    OpcodeInfo {
        mnemonic_prefix: "SET 0, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC1
    OpcodeInfo {
        mnemonic_prefix: "SET 0, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC2
    OpcodeInfo {
        mnemonic_prefix: "SET 0, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC3
    OpcodeInfo {
        mnemonic_prefix: "SET 0, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC4
    OpcodeInfo {
        mnemonic_prefix: "SET 0, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC5
    OpcodeInfo {
        mnemonic_prefix: "SET 0, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC6
    OpcodeInfo {
        mnemonic_prefix: "SET 0, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC7
    OpcodeInfo {
        mnemonic_prefix: "SET 0, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC8
    OpcodeInfo {
        mnemonic_prefix: "SET 1, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBC9
    OpcodeInfo {
        mnemonic_prefix: "SET 1, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCA
    OpcodeInfo {
        mnemonic_prefix: "SET 1, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCB
    OpcodeInfo {
        mnemonic_prefix: "SET 1, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCC
    OpcodeInfo {
        mnemonic_prefix: "SET 1, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCD
    OpcodeInfo {
        mnemonic_prefix: "SET 1, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCE
    OpcodeInfo {
        mnemonic_prefix: "SET 1, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBCF
    OpcodeInfo {
        mnemonic_prefix: "SET 1, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD0
    OpcodeInfo {
        mnemonic_prefix: "SET 2, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD1
    OpcodeInfo {
        mnemonic_prefix: "SET 2, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD2
    OpcodeInfo {
        mnemonic_prefix: "SET 2, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD3
    OpcodeInfo {
        mnemonic_prefix: "SET 2, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD4
    OpcodeInfo {
        mnemonic_prefix: "SET 2, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD5
    OpcodeInfo {
        mnemonic_prefix: "SET 2, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD6
    OpcodeInfo {
        mnemonic_prefix: "SET 2, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD7
    OpcodeInfo {
        mnemonic_prefix: "SET 2, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD8
    OpcodeInfo {
        mnemonic_prefix: "SET 3, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBD9
    OpcodeInfo {
        mnemonic_prefix: "SET 3, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDA
    OpcodeInfo {
        mnemonic_prefix: "SET 3, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDB
    OpcodeInfo {
        mnemonic_prefix: "SET 3, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDC
    OpcodeInfo {
        mnemonic_prefix: "SET 3, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDD
    OpcodeInfo {
        mnemonic_prefix: "SET 3, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDE
    OpcodeInfo {
        mnemonic_prefix: "SET 3, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBDF
    OpcodeInfo {
        mnemonic_prefix: "SET 3, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE0
    OpcodeInfo {
        mnemonic_prefix: "SET 4, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE1
    OpcodeInfo {
        mnemonic_prefix: "SET 4, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE2
    OpcodeInfo {
        mnemonic_prefix: "SET 4, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE3
    OpcodeInfo {
        mnemonic_prefix: "SET 4, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE4
    OpcodeInfo {
        mnemonic_prefix: "SET 4, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE5
    OpcodeInfo {
        mnemonic_prefix: "SET 4, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE6
    OpcodeInfo {
        mnemonic_prefix: "SET 4, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE7
    OpcodeInfo {
        mnemonic_prefix: "SET 4, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE8
    OpcodeInfo {
        mnemonic_prefix: "SET 5, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBE9
    OpcodeInfo {
        mnemonic_prefix: "SET 5, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBEA
    OpcodeInfo {
        mnemonic_prefix: "SET 5, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBEB
    OpcodeInfo {
        mnemonic_prefix: "SET 5, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBEC
    OpcodeInfo {
        mnemonic_prefix: "SET 5, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBED
    OpcodeInfo {
        mnemonic_prefix: "SET 5, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBEE
    OpcodeInfo {
        mnemonic_prefix: "SET 5, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBEF
    OpcodeInfo {
        mnemonic_prefix: "SET 5, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF0
    OpcodeInfo {
        mnemonic_prefix: "SET 6, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF1
    OpcodeInfo {
        mnemonic_prefix: "SET 6, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF2
    OpcodeInfo {
        mnemonic_prefix: "SET 6, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF3
    OpcodeInfo {
        mnemonic_prefix: "SET 6, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF4
    OpcodeInfo {
        mnemonic_prefix: "SET 6, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF5
    OpcodeInfo {
        mnemonic_prefix: "SET 6, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF6
    OpcodeInfo {
        mnemonic_prefix: "SET 6, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF7
    OpcodeInfo {
        mnemonic_prefix: "SET 6, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF8
    OpcodeInfo {
        mnemonic_prefix: "SET 7, B",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBF9
    OpcodeInfo {
        mnemonic_prefix: "SET 7, C",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFA
    OpcodeInfo {
        mnemonic_prefix: "SET 7, D",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFB
    OpcodeInfo {
        mnemonic_prefix: "SET 7, E",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFC
    OpcodeInfo {
        mnemonic_prefix: "SET 7, H",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFD
    OpcodeInfo {
        mnemonic_prefix: "SET 7, L",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFE
    OpcodeInfo {
        mnemonic_prefix: "SET 7, (HL)",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
    // 0xCBFF
    OpcodeInfo {
        mnemonic_prefix: "SET 7, A",
        mnemonic_suffix: "",
        length: 2,
        operand: Operand::None,
    },
];
