mod instructions;
mod registers;
mod utils;

use crate::bus::Bus;
use instructions::*;
use registers::{Reg8, Reg16, Registers};
pub(crate) use utils::*;

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

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let (ie, _if) = bus.get_ie_if();
        let int_pending = (ie & _if) & 0x1F;

        if self.halted {
            bus.tick(1);

            if int_pending != 0 {
                self.halted = false;
            }

            return 4;
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

        self.execute(opcode, bus)
    }

    fn service_interrupt(&mut self, int_pending: u8, _if: u8, bus: &mut Bus) -> u8 {
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
        20
    }

    fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            0x00 => 4, // NOP
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
            0x40 => 4, // LD B, B (lmao....)
            0x41 => ld_r8_r8(self, Reg8::B, Reg8::C),
            0x42 => ld_r8_r8(self, Reg8::B, Reg8::D),
            0x43 => ld_r8_r8(self, Reg8::B, Reg8::E),
            0x44 => ld_r8_r8(self, Reg8::B, Reg8::H),
            0x45 => ld_r8_r8(self, Reg8::B, Reg8::L),
            0x46 => ld_r8_mem_r16(self, bus, Reg8::B, Reg16::HL),
            0x47 => ld_r8_r8(self, Reg8::B, Reg8::A),
            0x48 => ld_r8_r8(self, Reg8::C, Reg8::B),
            0x49 => 4, // LD C, C
            0x4A => ld_r8_r8(self, Reg8::C, Reg8::D),
            0x4B => ld_r8_r8(self, Reg8::C, Reg8::E),
            0x4C => ld_r8_r8(self, Reg8::C, Reg8::H),
            0x4D => ld_r8_r8(self, Reg8::C, Reg8::L),
            0x4E => ld_r8_mem_r16(self, bus, Reg8::C, Reg16::HL),
            0x4F => ld_r8_r8(self, Reg8::C, Reg8::A),
            0x50 => ld_r8_r8(self, Reg8::D, Reg8::B),
            0x51 => ld_r8_r8(self, Reg8::D, Reg8::C),
            0x52 => 4, // LD D, D
            0x53 => ld_r8_r8(self, Reg8::D, Reg8::E),
            0x54 => ld_r8_r8(self, Reg8::D, Reg8::H),
            0x55 => ld_r8_r8(self, Reg8::D, Reg8::L),
            0x56 => ld_r8_mem_r16(self, bus, Reg8::D, Reg16::HL),
            0x57 => ld_r8_r8(self, Reg8::D, Reg8::A),
            0x58 => ld_r8_r8(self, Reg8::E, Reg8::B),
            0x59 => ld_r8_r8(self, Reg8::E, Reg8::C),
            0x5A => ld_r8_r8(self, Reg8::E, Reg8::D),
            0x5B => 4, // LD E, E
            0x5C => ld_r8_r8(self, Reg8::E, Reg8::H),
            0x5D => ld_r8_r8(self, Reg8::E, Reg8::L),
            0x5E => ld_r8_mem_r16(self, bus, Reg8::E, Reg16::HL),
            0x5F => ld_r8_r8(self, Reg8::E, Reg8::A),
            0x60 => ld_r8_r8(self, Reg8::H, Reg8::B),
            0x61 => ld_r8_r8(self, Reg8::H, Reg8::C),
            0x62 => ld_r8_r8(self, Reg8::H, Reg8::D),
            0x63 => ld_r8_r8(self, Reg8::H, Reg8::E),
            0x64 => 4, // LD H, H
            0x65 => ld_r8_r8(self, Reg8::H, Reg8::L),
            0x66 => ld_r8_mem_r16(self, bus, Reg8::H, Reg16::HL),
            0x67 => ld_r8_r8(self, Reg8::H, Reg8::A),
            0x68 => ld_r8_r8(self, Reg8::L, Reg8::B),
            0x69 => ld_r8_r8(self, Reg8::L, Reg8::C),
            0x6A => ld_r8_r8(self, Reg8::L, Reg8::D),
            0x6B => ld_r8_r8(self, Reg8::L, Reg8::E),
            0x6C => ld_r8_r8(self, Reg8::L, Reg8::H),
            0x6D => 4, // LD L, L
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
            0x7F => 4, // LD A, A
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
                panic!(
                    "Unknown opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.regs.pc() - 1
                );
            }
        }
    }

    #[inline(never)]
    fn execute_cb(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            // RLC
            0x00 => rlc_r8(self, Reg8::B),
            0x01 => rlc_r8(self, Reg8::C),
            0x02 => rlc_r8(self, Reg8::D),
            0x03 => rlc_r8(self, Reg8::E),
            0x04 => rlc_r8(self, Reg8::H),
            0x05 => rlc_r8(self, Reg8::L),
            0x06 => rlc_mem_hl(self, bus),
            0x07 => rlc_r8(self, Reg8::A),
            0x08 => rrc_r8(self, Reg8::B),
            0x09 => rrc_r8(self, Reg8::C),
            0x0A => rrc_r8(self, Reg8::D),
            0x0B => rrc_r8(self, Reg8::E),
            0x0C => rrc_r8(self, Reg8::H),
            0x0D => rrc_r8(self, Reg8::L),
            0x0E => rrc_mem_hl(self, bus),
            0x0F => rrc_r8(self, Reg8::A),
            0x10 => rl_r8(self, Reg8::B),
            0x11 => rl_r8(self, Reg8::C),
            0x12 => rl_r8(self, Reg8::D),
            0x13 => rl_r8(self, Reg8::E),
            0x14 => rl_r8(self, Reg8::H),
            0x15 => rl_r8(self, Reg8::L),
            0x16 => rl_mem_hl(self, bus),
            0x17 => rl_r8(self, Reg8::A),
            0x18 => rr_r8(self, Reg8::B),
            0x19 => rr_r8(self, Reg8::C),
            0x1A => rr_r8(self, Reg8::D),
            0x1B => rr_r8(self, Reg8::E),
            0x1C => rr_r8(self, Reg8::H),
            0x1D => rr_r8(self, Reg8::L),
            0x1E => rr_mem_hl(self, bus),
            0x1F => rr_r8(self, Reg8::A),
            0x20 => sla_r8(self, Reg8::B),
            0x21 => sla_r8(self, Reg8::C),
            0x22 => sla_r8(self, Reg8::D),
            0x23 => sla_r8(self, Reg8::E),
            0x24 => sla_r8(self, Reg8::H),
            0x25 => sla_r8(self, Reg8::L),
            0x26 => sla_mem_hl(self, bus),
            0x27 => sla_r8(self, Reg8::A),
            0x28 => sra_r8(self, Reg8::B),
            0x29 => sra_r8(self, Reg8::C),
            0x2A => sra_r8(self, Reg8::D),
            0x2B => sra_r8(self, Reg8::E),
            0x2C => sra_r8(self, Reg8::H),
            0x2D => sra_r8(self, Reg8::L),
            0x2E => sra_mem_hl(self, bus),
            0x2F => sra_r8(self, Reg8::A),
            0x30 => swap_r8(self, Reg8::B),
            0x31 => swap_r8(self, Reg8::C),
            0x32 => swap_r8(self, Reg8::D),
            0x33 => swap_r8(self, Reg8::E),
            0x34 => swap_r8(self, Reg8::H),
            0x35 => swap_r8(self, Reg8::L),
            0x36 => swap_mem_hl(self, bus),
            0x37 => swap_r8(self, Reg8::A),
            0x38 => srl_r8(self, Reg8::B),
            0x39 => srl_r8(self, Reg8::C),
            0x3A => srl_r8(self, Reg8::D),
            0x3B => srl_r8(self, Reg8::E),
            0x3C => srl_r8(self, Reg8::H),
            0x3D => srl_r8(self, Reg8::L),
            0x3E => srl_mem_hl(self, bus),
            0x3F => srl_r8(self, Reg8::A),
            0x40 => bit_imm3_r8(self, 0, Reg8::B),
            0x41 => bit_imm3_r8(self, 0, Reg8::C),
            0x42 => bit_imm3_r8(self, 0, Reg8::D),
            0x43 => bit_imm3_r8(self, 0, Reg8::E),
            0x44 => bit_imm3_r8(self, 0, Reg8::H),
            0x45 => bit_imm3_r8(self, 0, Reg8::L),
            0x46 => bit_imm3_mem_hl(self, bus, 0),
            0x47 => bit_imm3_r8(self, 0, Reg8::A),
            0x48 => bit_imm3_r8(self, 1, Reg8::B),
            0x49 => bit_imm3_r8(self, 1, Reg8::C),
            0x4A => bit_imm3_r8(self, 1, Reg8::D),
            0x4B => bit_imm3_r8(self, 1, Reg8::E),
            0x4C => bit_imm3_r8(self, 1, Reg8::H),
            0x4D => bit_imm3_r8(self, 1, Reg8::L),
            0x4E => bit_imm3_mem_hl(self, bus, 1),
            0x4F => bit_imm3_r8(self, 1, Reg8::A),
            0x50 => bit_imm3_r8(self, 2, Reg8::B),
            0x51 => bit_imm3_r8(self, 2, Reg8::C),
            0x52 => bit_imm3_r8(self, 2, Reg8::D),
            0x53 => bit_imm3_r8(self, 2, Reg8::E),
            0x54 => bit_imm3_r8(self, 2, Reg8::H),
            0x55 => bit_imm3_r8(self, 2, Reg8::L),
            0x56 => bit_imm3_mem_hl(self, bus, 2),
            0x57 => bit_imm3_r8(self, 2, Reg8::A),
            0x58 => bit_imm3_r8(self, 3, Reg8::B),
            0x59 => bit_imm3_r8(self, 3, Reg8::C),
            0x5A => bit_imm3_r8(self, 3, Reg8::D),
            0x5B => bit_imm3_r8(self, 3, Reg8::E),
            0x5C => bit_imm3_r8(self, 3, Reg8::H),
            0x5D => bit_imm3_r8(self, 3, Reg8::L),
            0x5E => bit_imm3_mem_hl(self, bus, 3),
            0x5F => bit_imm3_r8(self, 3, Reg8::A),
            0x60 => bit_imm3_r8(self, 4, Reg8::B),
            0x61 => bit_imm3_r8(self, 4, Reg8::C),
            0x62 => bit_imm3_r8(self, 4, Reg8::D),
            0x63 => bit_imm3_r8(self, 4, Reg8::E),
            0x64 => bit_imm3_r8(self, 4, Reg8::H),
            0x65 => bit_imm3_r8(self, 4, Reg8::L),
            0x66 => bit_imm3_mem_hl(self, bus, 4),
            0x67 => bit_imm3_r8(self, 4, Reg8::A),
            0x68 => bit_imm3_r8(self, 5, Reg8::B),
            0x69 => bit_imm3_r8(self, 5, Reg8::C),
            0x6A => bit_imm3_r8(self, 5, Reg8::D),
            0x6B => bit_imm3_r8(self, 5, Reg8::E),
            0x6C => bit_imm3_r8(self, 5, Reg8::H),
            0x6D => bit_imm3_r8(self, 5, Reg8::L),
            0x6E => bit_imm3_mem_hl(self, bus, 5),
            0x6F => bit_imm3_r8(self, 5, Reg8::A),
            0x70 => bit_imm3_r8(self, 6, Reg8::B),
            0x71 => bit_imm3_r8(self, 6, Reg8::C),
            0x72 => bit_imm3_r8(self, 6, Reg8::D),
            0x73 => bit_imm3_r8(self, 6, Reg8::E),
            0x74 => bit_imm3_r8(self, 6, Reg8::H),
            0x75 => bit_imm3_r8(self, 6, Reg8::L),
            0x76 => bit_imm3_mem_hl(self, bus, 6),
            0x77 => bit_imm3_r8(self, 6, Reg8::A),
            0x78 => bit_imm3_r8(self, 7, Reg8::B),
            0x79 => bit_imm3_r8(self, 7, Reg8::C),
            0x7A => bit_imm3_r8(self, 7, Reg8::D),
            0x7B => bit_imm3_r8(self, 7, Reg8::E),
            0x7C => bit_imm3_r8(self, 7, Reg8::H),
            0x7D => bit_imm3_r8(self, 7, Reg8::L),
            0x7E => bit_imm3_mem_hl(self, bus, 7),
            0x7F => bit_imm3_r8(self, 7, Reg8::A),
            0x80 => res_imm3_r8(self, 0, Reg8::B),
            0x81 => res_imm3_r8(self, 0, Reg8::C),
            0x82 => res_imm3_r8(self, 0, Reg8::D),
            0x83 => res_imm3_r8(self, 0, Reg8::E),
            0x84 => res_imm3_r8(self, 0, Reg8::H),
            0x85 => res_imm3_r8(self, 0, Reg8::L),
            0x86 => res_imm3_mem_hl(self, bus, 0),
            0x87 => res_imm3_r8(self, 0, Reg8::A),
            0x88 => res_imm3_r8(self, 1, Reg8::B),
            0x89 => res_imm3_r8(self, 1, Reg8::C),
            0x8A => res_imm3_r8(self, 1, Reg8::D),
            0x8B => res_imm3_r8(self, 1, Reg8::E),
            0x8C => res_imm3_r8(self, 1, Reg8::H),
            0x8D => res_imm3_r8(self, 1, Reg8::L),
            0x8E => res_imm3_mem_hl(self, bus, 1),
            0x8F => res_imm3_r8(self, 1, Reg8::A),
            0x90 => res_imm3_r8(self, 2, Reg8::B),
            0x91 => res_imm3_r8(self, 2, Reg8::C),
            0x92 => res_imm3_r8(self, 2, Reg8::D),
            0x93 => res_imm3_r8(self, 2, Reg8::E),
            0x94 => res_imm3_r8(self, 2, Reg8::H),
            0x95 => res_imm3_r8(self, 2, Reg8::L),
            0x96 => res_imm3_mem_hl(self, bus, 2),
            0x97 => res_imm3_r8(self, 2, Reg8::A),
            0x98 => res_imm3_r8(self, 3, Reg8::B),
            0x99 => res_imm3_r8(self, 3, Reg8::C),
            0x9A => res_imm3_r8(self, 3, Reg8::D),
            0x9B => res_imm3_r8(self, 3, Reg8::E),
            0x9C => res_imm3_r8(self, 3, Reg8::H),
            0x9D => res_imm3_r8(self, 3, Reg8::L),
            0x9E => res_imm3_mem_hl(self, bus, 3),
            0x9F => res_imm3_r8(self, 3, Reg8::A),
            0xA0 => res_imm3_r8(self, 4, Reg8::B),
            0xA1 => res_imm3_r8(self, 4, Reg8::C),
            0xA2 => res_imm3_r8(self, 4, Reg8::D),
            0xA3 => res_imm3_r8(self, 4, Reg8::E),
            0xA4 => res_imm3_r8(self, 4, Reg8::H),
            0xA5 => res_imm3_r8(self, 4, Reg8::L),
            0xA6 => res_imm3_mem_hl(self, bus, 4),
            0xA7 => res_imm3_r8(self, 4, Reg8::A),
            0xA8 => res_imm3_r8(self, 5, Reg8::B),
            0xA9 => res_imm3_r8(self, 5, Reg8::C),
            0xAA => res_imm3_r8(self, 5, Reg8::D),
            0xAB => res_imm3_r8(self, 5, Reg8::E),
            0xAC => res_imm3_r8(self, 5, Reg8::H),
            0xAD => res_imm3_r8(self, 5, Reg8::L),
            0xAE => res_imm3_mem_hl(self, bus, 5),
            0xAF => res_imm3_r8(self, 5, Reg8::A),
            0xB0 => res_imm3_r8(self, 6, Reg8::B),
            0xB1 => res_imm3_r8(self, 6, Reg8::C),
            0xB2 => res_imm3_r8(self, 6, Reg8::D),
            0xB3 => res_imm3_r8(self, 6, Reg8::E),
            0xB4 => res_imm3_r8(self, 6, Reg8::H),
            0xB5 => res_imm3_r8(self, 6, Reg8::L),
            0xB6 => res_imm3_mem_hl(self, bus, 6),
            0xB7 => res_imm3_r8(self, 6, Reg8::A),
            0xB8 => res_imm3_r8(self, 7, Reg8::B),
            0xB9 => res_imm3_r8(self, 7, Reg8::C),
            0xBA => res_imm3_r8(self, 7, Reg8::D),
            0xBB => res_imm3_r8(self, 7, Reg8::E),
            0xBC => res_imm3_r8(self, 7, Reg8::H),
            0xBD => res_imm3_r8(self, 7, Reg8::L),
            0xBE => res_imm3_mem_hl(self, bus, 7),
            0xBF => res_imm3_r8(self, 7, Reg8::A),
            0xC0 => set_imm3_r8(self, 0, Reg8::B),
            0xC1 => set_imm3_r8(self, 0, Reg8::C),
            0xC2 => set_imm3_r8(self, 0, Reg8::D),
            0xC3 => set_imm3_r8(self, 0, Reg8::E),
            0xC4 => set_imm3_r8(self, 0, Reg8::H),
            0xC5 => set_imm3_r8(self, 0, Reg8::L),
            0xC6 => set_imm3_mem_hl(self, bus, 0),
            0xC7 => set_imm3_r8(self, 0, Reg8::A),
            0xC8 => set_imm3_r8(self, 1, Reg8::B),
            0xC9 => set_imm3_r8(self, 1, Reg8::C),
            0xCA => set_imm3_r8(self, 1, Reg8::D),
            0xCB => set_imm3_r8(self, 1, Reg8::E),
            0xCC => set_imm3_r8(self, 1, Reg8::H),
            0xCD => set_imm3_r8(self, 1, Reg8::L),
            0xCE => set_imm3_mem_hl(self, bus, 1),
            0xCF => set_imm3_r8(self, 1, Reg8::A),
            0xD0 => set_imm3_r8(self, 2, Reg8::B),
            0xD1 => set_imm3_r8(self, 2, Reg8::C),
            0xD2 => set_imm3_r8(self, 2, Reg8::D),
            0xD3 => set_imm3_r8(self, 2, Reg8::E),
            0xD4 => set_imm3_r8(self, 2, Reg8::H),
            0xD5 => set_imm3_r8(self, 2, Reg8::L),
            0xD6 => set_imm3_mem_hl(self, bus, 2),
            0xD7 => set_imm3_r8(self, 2, Reg8::A),
            0xD8 => set_imm3_r8(self, 3, Reg8::B),
            0xD9 => set_imm3_r8(self, 3, Reg8::C),
            0xDA => set_imm3_r8(self, 3, Reg8::D),
            0xDB => set_imm3_r8(self, 3, Reg8::E),
            0xDC => set_imm3_r8(self, 3, Reg8::H),
            0xDD => set_imm3_r8(self, 3, Reg8::L),
            0xDE => set_imm3_mem_hl(self, bus, 3),
            0xDF => set_imm3_r8(self, 3, Reg8::A),
            0xE0 => set_imm3_r8(self, 4, Reg8::B),
            0xE1 => set_imm3_r8(self, 4, Reg8::C),
            0xE2 => set_imm3_r8(self, 4, Reg8::D),
            0xE3 => set_imm3_r8(self, 4, Reg8::E),
            0xE4 => set_imm3_r8(self, 4, Reg8::H),
            0xE5 => set_imm3_r8(self, 4, Reg8::L),
            0xE6 => set_imm3_mem_hl(self, bus, 4),
            0xE7 => set_imm3_r8(self, 4, Reg8::A),
            0xE8 => set_imm3_r8(self, 5, Reg8::B),
            0xE9 => set_imm3_r8(self, 5, Reg8::C),
            0xEA => set_imm3_r8(self, 5, Reg8::D),
            0xEB => set_imm3_r8(self, 5, Reg8::E),
            0xEC => set_imm3_r8(self, 5, Reg8::H),
            0xED => set_imm3_r8(self, 5, Reg8::L),
            0xEE => set_imm3_mem_hl(self, bus, 5),
            0xEF => set_imm3_r8(self, 5, Reg8::A),
            0xF0 => set_imm3_r8(self, 6, Reg8::B),
            0xF1 => set_imm3_r8(self, 6, Reg8::C),
            0xF2 => set_imm3_r8(self, 6, Reg8::D),
            0xF3 => set_imm3_r8(self, 6, Reg8::E),
            0xF4 => set_imm3_r8(self, 6, Reg8::H),
            0xF5 => set_imm3_r8(self, 6, Reg8::L),
            0xF6 => set_imm3_mem_hl(self, bus, 6),
            0xF7 => set_imm3_r8(self, 6, Reg8::A),
            0xF8 => set_imm3_r8(self, 7, Reg8::B),
            0xF9 => set_imm3_r8(self, 7, Reg8::C),
            0xFA => set_imm3_r8(self, 7, Reg8::D),
            0xFB => set_imm3_r8(self, 7, Reg8::E),
            0xFC => set_imm3_r8(self, 7, Reg8::H),
            0xFD => set_imm3_r8(self, 7, Reg8::L),
            0xFE => set_imm3_mem_hl(self, bus, 7),
            0xFF => set_imm3_r8(self, 7, Reg8::A),
        }
    }
}
