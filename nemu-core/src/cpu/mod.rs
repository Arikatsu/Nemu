mod instructions;
mod registers;
mod utils;

use crate::bus::Bus;
use instructions::*;
use registers::{Reg8, Reg16, Registers};
use utils::*;

pub struct Cpu {
    pub(crate) regs: Registers,
    pub(crate) ime: InterruptMode,
    pub(crate) halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            ime: InterruptMode::Disabled,
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
        self.ime = InterruptMode::Disabled;
        self.halted = false;
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let (ie, _if) = bus.get_ie_if();
        let int_pending = (ie & _if) & 0x1F;

        if self.halted {
            bus.tick(1);

            if int_pending != 0 {
                self.halted = false;
            }

            return;
        }

        if let InterruptMode::Enabled = self.ime {
            if int_pending != 0 {
                return self.service_interrupt(int_pending, _if, bus);
            }
        }

        if let InterruptMode::Pending = self.ime {
            self.ime = InterruptMode::Enabled;
        }

        let opcode = bus.read(self.regs.pc());
        self.regs.inc_pc(1);

        if opcode == 0xCB {
            let cb_opcode = bus.read(self.regs.pc());
            self.regs.inc_pc(1);
            return self.execute_cb(cb_opcode, bus);
        }

        self.execute(opcode, bus);
    }

    fn service_interrupt(&mut self, int_pending: u8, _if: u8, bus: &mut Bus) {
        for (bit, addr) in [
            (0, 0x40), // V-Blank
            (1, 0x48), // LCD STAT
            (2, 0x50), // Timer
            (3, 0x58), // Serial
            (4, 0x60), // Joypad
        ] {
            if (int_pending & (1 << bit)) != 0 {
                self.ime = InterruptMode::Disabled;
                bus.write(0xFF0F, _if & !(1 << bit));

                let sp = self.regs.sp().wrapping_sub(2);
                bus.write_u16(sp, self.regs.pc());

                self.regs.set_sp(sp);
                self.regs.set_pc(addr);

                bus.tick(2);

                self.halted = false;
                break;
            }
        }
    }

    fn execute(&mut self, opcode: u8, bus: &mut Bus) {
        match opcode {
            0x00 => {} // NOP
            0x01 => ld_r16_imm16(self, bus, Reg16::BC),
            0x02 => ld_mem_r16_r8(self, bus, Reg16::BC, Reg8::A),
            0x03 => inc_r16(self, bus, Reg16::BC),
            0x04 => inc_r8(self, Reg8::B),
            0x05 => dec_r8(self, Reg8::B),
            0x06 => ld_r8_imm8(self, bus, Reg8::B),
            0x07 => rlca(self),
            0x08 => ld_mem_imm16_sp(self, bus),
            0x09 => add_hl_r16(self, bus, Reg16::BC),
            0x0A => ld_r8_mem_r16(self, bus, Reg8::A, Reg16::BC),
            0x0B => dec_r16(self, bus, Reg16::BC),
            0x0C => inc_r8(self, Reg8::C),
            0x0D => dec_r8(self, Reg8::C),
            0x0E => ld_r8_imm8(self, bus, Reg8::C),
            0x0F => rrca(self),
            0x10 => stop(self),
            0x11 => ld_r16_imm16(self, bus, Reg16::DE),
            0x12 => ld_mem_r16_r8(self, bus, Reg16::DE, Reg8::A),
            0x13 => inc_r16(self, bus, Reg16::DE),
            0x14 => inc_r8(self, Reg8::D),
            0x15 => dec_r8(self, Reg8::D),
            0x16 => ld_r8_imm8(self, bus, Reg8::D),
            0x17 => rla(self),
            0x18 => jr_imm8(self, bus),
            0x19 => add_hl_r16(self, bus, Reg16::DE),
            0x1A => ld_r8_mem_r16(self, bus, Reg8::A, Reg16::DE),
            0x1B => dec_r16(self, bus, Reg16::DE),
            0x1C => inc_r8(self, Reg8::E),
            0x1D => dec_r8(self, Reg8::E),
            0x1E => ld_r8_imm8(self, bus, Reg8::E),
            0x1F => rra(self),
            0x20 => jr_cond_imm8(self, bus, JumpCond::NZ),
            0x21 => ld_r16_imm16(self, bus, Reg16::HL),
            0x22 => ld_mem_hli_a(self, bus),
            0x23 => inc_r16(self, bus, Reg16::HL),
            0x24 => inc_r8(self, Reg8::H),
            0x25 => dec_r8(self, Reg8::H),
            0x26 => ld_r8_imm8(self, bus, Reg8::H),
            0x27 => daa(self),
            0x28 => jr_cond_imm8(self, bus, JumpCond::Z),
            0x29 => add_hl_r16(self, bus, Reg16::HL),
            0x2A => ld_a_mem_hli(self, bus),
            0x2B => dec_r16(self, bus, Reg16::HL),
            0x2C => inc_r8(self, Reg8::L),
            0x2D => dec_r8(self, Reg8::L),
            0x2E => ld_r8_imm8(self, bus, Reg8::L),
            0x2F => cpl(self),
            0x30 => jr_cond_imm8(self, bus, JumpCond::NC),
            0x31 => ld_sp_imm16(self, bus),
            0x32 => ld_mem_hld_a(self, bus),
            0x33 => inc_sp(self, bus),
            0x34 => inc_mem_hl(self, bus),
            0x35 => dec_mem_hl(self, bus),
            0x36 => ld_mem_r16_imm8(self, bus, Reg16::HL),
            0x37 => scf(self),
            0x38 => jr_cond_imm8(self, bus, JumpCond::C),
            0x39 => add_hl_sp(self, bus),
            0x3A => ld_a_mem_hld(self, bus),
            0x3B => dec_sp(self, bus),
            0x3C => inc_r8(self, Reg8::A),
            0x3D => dec_r8(self, Reg8::A),
            0x3E => ld_r8_imm8(self, bus, Reg8::A),
            0x3F => ccf(self),
            0x40 => {} // LD B, B (lmao....)
            0x41 => ld_r8_r8(self, Reg8::B, Reg8::C),
            0x42 => ld_r8_r8(self, Reg8::B, Reg8::D),
            0x43 => ld_r8_r8(self, Reg8::B, Reg8::E),
            0x44 => ld_r8_r8(self, Reg8::B, Reg8::H),
            0x45 => ld_r8_r8(self, Reg8::B, Reg8::L),
            0x46 => ld_r8_mem_r16(self, bus, Reg8::B, Reg16::HL),
            0x47 => ld_r8_r8(self, Reg8::B, Reg8::A),
            0x48 => ld_r8_r8(self, Reg8::C, Reg8::B),
            0x49 => {} // LD C, C
            0x4A => ld_r8_r8(self, Reg8::C, Reg8::D),
            0x4B => ld_r8_r8(self, Reg8::C, Reg8::E),
            0x4C => ld_r8_r8(self, Reg8::C, Reg8::H),
            0x4D => ld_r8_r8(self, Reg8::C, Reg8::L),
            0x4E => ld_r8_mem_r16(self, bus, Reg8::C, Reg16::HL),
            0x4F => ld_r8_r8(self, Reg8::C, Reg8::A),
            0x50 => ld_r8_r8(self, Reg8::D, Reg8::B),
            0x51 => ld_r8_r8(self, Reg8::D, Reg8::C),
            0x52 => {} // LD D, D
            0x53 => ld_r8_r8(self, Reg8::D, Reg8::E),
            0x54 => ld_r8_r8(self, Reg8::D, Reg8::H),
            0x55 => ld_r8_r8(self, Reg8::D, Reg8::L),
            0x56 => ld_r8_mem_r16(self, bus, Reg8::D, Reg16::HL),
            0x57 => ld_r8_r8(self, Reg8::D, Reg8::A),
            0x58 => ld_r8_r8(self, Reg8::E, Reg8::B),
            0x59 => ld_r8_r8(self, Reg8::E, Reg8::C),
            0x5A => ld_r8_r8(self, Reg8::E, Reg8::D),
            0x5B => {} // LD E, E
            0x5C => ld_r8_r8(self, Reg8::E, Reg8::H),
            0x5D => ld_r8_r8(self, Reg8::E, Reg8::L),
            0x5E => ld_r8_mem_r16(self, bus, Reg8::E, Reg16::HL),
            0x5F => ld_r8_r8(self, Reg8::E, Reg8::A),
            0x60 => ld_r8_r8(self, Reg8::H, Reg8::B),
            0x61 => ld_r8_r8(self, Reg8::H, Reg8::C),
            0x62 => ld_r8_r8(self, Reg8::H, Reg8::D),
            0x63 => ld_r8_r8(self, Reg8::H, Reg8::E),
            0x64 => {} // LD H, H
            0x65 => ld_r8_r8(self, Reg8::H, Reg8::L),
            0x66 => ld_r8_mem_r16(self, bus, Reg8::H, Reg16::HL),
            0x67 => ld_r8_r8(self, Reg8::H, Reg8::A),
            0x68 => ld_r8_r8(self, Reg8::L, Reg8::B),
            0x69 => ld_r8_r8(self, Reg8::L, Reg8::C),
            0x6A => ld_r8_r8(self, Reg8::L, Reg8::D),
            0x6B => ld_r8_r8(self, Reg8::L, Reg8::E),
            0x6C => ld_r8_r8(self, Reg8::L, Reg8::H),
            0x6D => {} // LD L, L
            0x6E => ld_r8_mem_r16(self, bus, Reg8::L, Reg16::HL),
            0x6F => ld_r8_r8(self, Reg8::L, Reg8::A),
            0x70 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::B),
            0x71 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::C),
            0x72 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::D),
            0x73 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::E),
            0x74 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::H),
            0x75 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::L),
            0x76 => halt(self),
            0x77 => ld_mem_r16_r8(self, bus, Reg16::HL, Reg8::A),
            0x78 => ld_r8_r8(self, Reg8::A, Reg8::B),
            0x79 => ld_r8_r8(self, Reg8::A, Reg8::C),
            0x7A => ld_r8_r8(self, Reg8::A, Reg8::D),
            0x7B => ld_r8_r8(self, Reg8::A, Reg8::E),
            0x7C => ld_r8_r8(self, Reg8::A, Reg8::H),
            0x7D => ld_r8_r8(self, Reg8::A, Reg8::L),
            0x7E => ld_r8_mem_r16(self, bus, Reg8::A, Reg16::HL),
            0x7F => {} // LD A, A
            0x80 => add_r8(self, Reg8::B),
            0x81 => add_r8(self, Reg8::C),
            0x82 => add_r8(self, Reg8::D),
            0x83 => add_r8(self, Reg8::E),
            0x84 => add_r8(self, Reg8::H),
            0x85 => add_r8(self, Reg8::L),
            0x86 => add_mem_hl(self, bus),
            0x87 => add_r8(self, Reg8::A),
            0x88 => adc_r8(self, Reg8::B),
            0x89 => adc_r8(self, Reg8::C),
            0x8A => adc_r8(self, Reg8::D),
            0x8B => adc_r8(self, Reg8::E),
            0x8C => adc_r8(self, Reg8::H),
            0x8D => adc_r8(self, Reg8::L),
            0x8E => adc_mem_hl(self, bus),
            0x8F => adc_r8(self, Reg8::A),
            0x90 => sub_r8(self, Reg8::B),
            0x91 => sub_r8(self, Reg8::C),
            0x92 => sub_r8(self, Reg8::D),
            0x93 => sub_r8(self, Reg8::E),
            0x94 => sub_r8(self, Reg8::H),
            0x95 => sub_r8(self, Reg8::L),
            0x96 => sub_mem_hl(self, bus),
            0x97 => sub_r8(self, Reg8::A),
            0x98 => sbc_r8(self, Reg8::B),
            0x99 => sbc_r8(self, Reg8::C),
            0x9A => sbc_r8(self, Reg8::D),
            0x9B => sbc_r8(self, Reg8::E),
            0x9C => sbc_r8(self, Reg8::H),
            0x9D => sbc_r8(self, Reg8::L),
            0x9E => sbc_mem_hl(self, bus),
            0x9F => sbc_r8(self, Reg8::A),
            0xA0 => and_r8(self, Reg8::B),
            0xA1 => and_r8(self, Reg8::C),
            0xA2 => and_r8(self, Reg8::D),
            0xA3 => and_r8(self, Reg8::E),
            0xA4 => and_r8(self, Reg8::H),
            0xA5 => and_r8(self, Reg8::L),
            0xA6 => and_mem_hl(self, bus),
            0xA7 => and_r8(self, Reg8::A),
            0xA8 => xor_r8(self, Reg8::B),
            0xA9 => xor_r8(self, Reg8::C),
            0xAA => xor_r8(self, Reg8::D),
            0xAB => xor_r8(self, Reg8::E),
            0xAC => xor_r8(self, Reg8::H),
            0xAD => xor_r8(self, Reg8::L),
            0xAE => xor_mem_hl(self, bus),
            0xAF => xor_r8(self, Reg8::A),
            0xB0 => or_r8(self, Reg8::B),
            0xB1 => or_r8(self, Reg8::C),
            0xB2 => or_r8(self, Reg8::D),
            0xB3 => or_r8(self, Reg8::E),
            0xB4 => or_r8(self, Reg8::H),
            0xB5 => or_r8(self, Reg8::L),
            0xB6 => or_mem_hl(self, bus),
            0xB7 => or_r8(self, Reg8::A),
            0xB8 => cp_r8(self, Reg8::B),
            0xB9 => cp_r8(self, Reg8::C),
            0xBA => cp_r8(self, Reg8::D),
            0xBB => cp_r8(self, Reg8::E),
            0xBC => cp_r8(self, Reg8::H),
            0xBD => cp_r8(self, Reg8::L),
            0xBE => cp_mem_hl(self, bus),
            0xBF => cp_r8(self, Reg8::A),
            0xC0 => ret_cond(self, bus, JumpCond::NZ),
            0xC1 => pop_r16(self, bus, Reg16::BC),
            0xC2 => jp_cond_imm16(self, bus, JumpCond::NZ),
            0xC3 => jp_imm16(self, bus),
            0xC4 => call_cond_imm16(self, bus, JumpCond::NZ),
            0xC5 => push_r16(self, bus, Reg16::BC),
            0xC6 => add_imm8(self, bus),
            0xC7 => rst(self, bus, 0x00),
            0xC8 => ret_cond(self, bus, JumpCond::Z),
            0xC9 => ret(self, bus),
            0xCA => jp_cond_imm16(self, bus, JumpCond::Z),
            // 0xCB => PREFIX CB
            0xCC => call_cond_imm16(self, bus, JumpCond::Z),
            0xCD => call_imm16(self, bus),
            0xCE => adc_imm8(self, bus),
            0xCF => rst(self, bus, 0x08),
            0xD0 => ret_cond(self, bus, JumpCond::NC),
            0xD1 => pop_r16(self, bus, Reg16::DE),
            0xD2 => jp_cond_imm16(self, bus, JumpCond::NC),
            // 0xD3 => UNUSED
            0xD4 => call_cond_imm16(self, bus, JumpCond::NC),
            0xD5 => push_r16(self, bus, Reg16::DE),
            0xD6 => sub_imm8(self, bus),
            0xD7 => rst(self, bus, 0x10),
            0xD8 => ret_cond(self, bus, JumpCond::C),
            0xD9 => reti(self, bus),
            0xDA => jp_cond_imm16(self, bus, JumpCond::C),
            // 0xDB => UNUSED
            0xDC => call_cond_imm16(self, bus, JumpCond::C),
            // 0xDD => UNUSED
            0xDE => sbc_imm8(self, bus),
            0xDF => rst(self, bus, 0x18),
            0xE0 => ldh_mem_imm8_a(self, bus),
            0xE1 => pop_r16(self, bus, Reg16::HL),
            0xE2 => ldh_mem_c_a(self, bus),
            // 0xE3 => UNUSED
            // 0xE4 => UNUSED
            0xE5 => push_r16(self, bus, Reg16::HL),
            0xE6 => and_imm8(self, bus),
            0xE7 => rst(self, bus, 0x20),
            0xE8 => add_sp_imm8(self, bus),
            0xE9 => jp_hl(self),
            0xEA => ld_mem_imm16_a(self, bus),
            // 0xEB => UNUSED
            // 0xEC => UNUSED
            // 0xED => UNUSED
            0xEE => xor_imm8(self, bus),
            0xEF => rst(self, bus, 0x28),
            0xF0 => ldh_a_mem_imm8(self, bus),
            0xF1 => pop_r16(self, bus, Reg16::AF),
            0xF2 => ldh_a_mem_c(self, bus),
            0xF3 => di(self),
            // 0xF4 => UNUSED
            0xF5 => push_r16(self, bus, Reg16::AF),
            0xF6 => or_imm8(self, bus),
            0xF7 => rst(self, bus, 0x30),
            0xF8 => ld_hl_sp_imm8(self, bus),
            0xF9 => ld_sp_hl(self, bus),
            0xFA => ld_a_mem_imm16(self, bus),
            0xFB => ei(self),
            // 0xFC => UNUSED
            // 0xFD => UNUSED
            0xFE => cp_imm8(self, bus),
            0xFF => rst(self, bus, 0x38),

            _ => {
                panic!("Unknown opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.regs.pc() - 1);
            }
        }
    }

    fn execute_cb(&mut self, opcode: u8, bus: &mut Bus) {
        match opcode {
            0x00..=0x07 => match reg_cb!(opcode) {
                Some(r) => rlc_r8(self, r),
                None => rlc_mem_hl(self, bus),
            },
            0x08..=0x0F => match reg_cb!(opcode) {
                Some(r) => rrc_r8(self, r),
                None => rrc_mem_hl(self, bus),
            },
            0x10..=0x17 => match reg_cb!(opcode) {
                Some(r) => rl_r8(self, r),
                None => rl_mem_hl(self, bus),
            },
            0x18..=0x1F => match reg_cb!(opcode) {
                Some(r) => rr_r8(self, r),
                None => rr_mem_hl(self, bus),
            },
            0x20..=0x27 => match reg_cb!(opcode) {
                Some(r) => sla_r8(self, r),
                None => sla_mem_hl(self, bus),
            },
            0x28..=0x2F => match reg_cb!(opcode) {
                Some(r) => sra_r8(self, r),
                None => sra_mem_hl(self, bus),
            },
            0x30..=0x37 => match reg_cb!(opcode) {
                Some(r) => swap_r8(self, r),
                None => swap_mem_hl(self, bus),
            },
            0x38..=0x3F => match reg_cb!(opcode) {
                Some(r) => srl_r8(self, r),
                None => srl_mem_hl(self, bus),
            },
            0x40..=0x7F => {
                let bit = (opcode - 0x40) / 8;
                match reg_cb!(opcode) {
                    Some(r) => bit_imm3_r8(self, bit, r),
                    None => bit_imm3_mem_hl(self, bus, bit),
                }
            }
            0x80..=0xBF => {
                let bit = (opcode - 0x80) / 8;
                match reg_cb!(opcode) {
                    Some(r) => res_imm3_r8(self, bit, r),
                    None => res_imm3_mem_hl(self, bus, bit),
                }
            }
            0xC0..=0xFF => {
                let bit = (opcode - 0xC0) / 8;
                match reg_cb!(opcode) {
                    Some(r) => set_imm3_r8(self, bit, r),
                    None => set_imm3_mem_hl(self, bus, bit),
                }
            }
        }
    }
}
