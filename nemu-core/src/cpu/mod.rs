mod registers;
mod instructions;

use instructions::*;
use registers::{Reg8, Reg16, Registers};
use crate::memory::Memory;

#[cfg(feature = "debug_logging")]
use std::io::{BufWriter, Write};

pub(in self) enum InterruptMode {
    Enabled,
    Disabled,
    Pending,
}

macro_rules! reg_cb {
    ($code:expr) => {
        {
            match $code & 0x07 {
                0 => Some(Reg8::B),
                1 => Some(Reg8::C),
                2 => Some(Reg8::D),
                3 => Some(Reg8::E),
                4 => Some(Reg8::H),
                5 => Some(Reg8::L),
                6 => None, // (HL)
                7 => Some(Reg8::A),
                _ => unreachable!(),
            }
        }
    };
}

pub struct Cpu {
    pub(crate) regs: Registers,
    sp: u16,                 // Stack Pointer
    pc: u16,                 // Program Counter
    ime: InterruptMode,      // Interrupt Master Enable Flag
    halted: bool,

    #[cfg(feature = "debug_logging")]
    log_writer: BufWriter<std::fs::File>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            sp: 0xFFFE,
            pc: 0x0100,
            ime: InterruptMode::Disabled,

            #[cfg(feature = "debug_logging")]
            log_writer: BufWriter::with_capacity(64 * 1024, std::fs::File::create("cpu_trace.log").unwrap()),
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
        self.sp = 0xFFFE;
        self.pc = 0x0100;
        self.ime = InterruptMode::Disabled;
    }

    #[inline]
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    #[inline]
    pub fn inc_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }

    #[inline]
    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    #[inline]
    pub fn inc_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_add(value);
    }

    #[inline]
    pub fn dec_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(value);
    }

    pub fn step(&mut self, memory: &mut Memory) -> u8 {
        #[cfg(feature = "debug_logging")]
        self.write_log_line(memory);

        if let InterruptMode::Pending = self.ime {
            self.ime = InterruptMode::Enabled;
        }

        let opcode = memory.read(self.pc);
        self.inc_pc(1);

        if opcode == 0xCB {
            let cb_opcode = memory.read(self.pc);
            self.inc_pc(1);
            return self.execute_cb(cb_opcode, memory);
        }

        self.execute(opcode, memory)
    }

    fn execute(&mut self, opcode: u8) -> u8 {

    fn execute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let mut ctx = InstructionContext {
            cpu: self,
            memory,
        };

        match opcode {
            0x00 => 4, // NOP
            0x01 => ld_r16_imm16(&mut ctx, Reg16::BC),
            0x02 => ld_mem_r16_r8(&mut ctx, Reg16::BC, Reg8::A),
            0x03 => inc_r16(&mut ctx, Reg16::BC),
            0x04 => inc_r8(&mut ctx, Reg8::B),
            0x05 => dec_r8(&mut ctx, Reg8::B),
            0x06 => ld_r8_imm8(&mut ctx, Reg8::B),
            0x07 => rlca(&mut ctx),
            0x08 => ld_mem_imm16_sp(&mut ctx),
            0x09 => add_hl_r16(&mut ctx, Reg16::BC),
            0x0A => ld_r8_mem_r16(&mut ctx, Reg8::A, Reg16::BC),
            0x0B => dec_r16(&mut ctx, Reg16::BC),
            0x0C => inc_r8(&mut ctx, Reg8::C),
            0x0D => dec_r8(&mut ctx, Reg8::C),
            0x0E => ld_r8_imm8(&mut ctx, Reg8::C),
            0x0F => rrca(&mut ctx),
            0x10 => stop(&mut ctx),
            0x11 => ld_r16_imm16(&mut ctx, Reg16::DE),
            0x12 => ld_mem_r16_r8(&mut ctx, Reg16::DE, Reg8::A),
            0x13 => inc_r16(&mut ctx, Reg16::DE),
            0x14 => inc_r8(&mut ctx, Reg8::D),
            0x15 => dec_r8(&mut ctx, Reg8::D),
            0x16 => ld_r8_imm8(&mut ctx, Reg8::D),
            0x17 => rla(&mut ctx),
            0x18 => jr_imm8(&mut ctx),
            0x19 => add_hl_r16(&mut ctx, Reg16::DE),
            0x1A => ld_r8_mem_r16(&mut ctx, Reg8::A, Reg16::DE),
            0x1B => dec_r16(&mut ctx, Reg16::DE),
            0x1C => inc_r8(&mut ctx, Reg8::E),
            0x1D => dec_r8(&mut ctx, Reg8::E),
            0x1E => ld_r8_imm8(&mut ctx, Reg8::E),
            0x1F => rra(&mut ctx),
            0x20 => jr_cond_imm8(&mut ctx, JumpCond::NZ),
            0x21 => ld_r16_imm16(&mut ctx, Reg16::HL),
            0x22 => ld_mem_hli_a(&mut ctx),
            0x23 => inc_r16(&mut ctx, Reg16::HL),
            0x24 => inc_r8(&mut ctx, Reg8::H),
            0x25 => dec_r8(&mut ctx, Reg8::H),
            0x26 => ld_r8_imm8(&mut ctx, Reg8::H),
            0x27 => daa(&mut ctx),
            0x28 => jr_cond_imm8(&mut ctx, JumpCond::Z),
            0x29 => add_hl_r16(&mut ctx, Reg16::HL),
            0x2A => ld_a_mem_hli(&mut ctx),
            0x2B => dec_r16(&mut ctx, Reg16::HL),
            0x2C => inc_r8(&mut ctx, Reg8::L),
            0x2D => dec_r8(&mut ctx, Reg8::L),
            0x2E => ld_r8_imm8(&mut ctx, Reg8::L),
            0x2F => cpl(&mut ctx),
            0x30 => jr_cond_imm8(&mut ctx, JumpCond::NC),
            0x31 => ld_sp_imm16(&mut ctx),
            0x32 => ld_mem_hld_a(&mut ctx),
            0x33 => inc_sp(&mut ctx),
            0x34 => inc_mem_hl(&mut ctx),
            0x35 => dec_mem_hl(&mut ctx),
            0x36 => ld_mem_r16_imm8(&mut ctx, Reg16::HL),
            0x37 => scf(&mut ctx),
            0x38 => jr_cond_imm8(&mut ctx, JumpCond::C),
            0x39 => add_hl_sp(&mut ctx),
            0x3A => ld_a_mem_hld(&mut ctx),
            0x3B => dec_sp(&mut ctx),
            0x3C => inc_r8(&mut ctx, Reg8::A),
            0x3D => dec_r8(&mut ctx, Reg8::A),
            0x3E => ld_r8_imm8(&mut ctx, Reg8::A),
            0x3F => ccf(&mut ctx),
            0x40 => 4, // LD B, B (lmao....)
            0x41 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::C),
            0x42 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::D),
            0x43 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::E),
            0x44 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::H),
            0x45 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::L),
            0x46 => ld_r8_mem_r16(&mut ctx, Reg8::B, Reg16::HL),
            0x47 => ld_r8_r8(&mut ctx, Reg8::B, Reg8::A),
            0x48 => ld_r8_r8(&mut ctx, Reg8::C, Reg8::B),
            0x49 => 4, // LD C, C
            0x4A => ld_r8_r8(&mut ctx, Reg8::C, Reg8::D),
            0x4B => ld_r8_r8(&mut ctx, Reg8::C, Reg8::E),
            0x4C => ld_r8_r8(&mut ctx, Reg8::C, Reg8::H),
            0x4D => ld_r8_r8(&mut ctx, Reg8::C, Reg8::L),
            0x4E => ld_r8_mem_r16(&mut ctx, Reg8::C, Reg16::HL),
            0x4F => ld_r8_r8(&mut ctx, Reg8::C, Reg8::A),
            0x50 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::B),
            0x51 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::C),
            0x52 => 4, // LD D, D
            0x53 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::E),
            0x54 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::H),
            0x55 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::L),
            0x56 => ld_r8_mem_r16(&mut ctx, Reg8::D, Reg16::HL),
            0x57 => ld_r8_r8(&mut ctx, Reg8::D, Reg8::A),
            0x58 => ld_r8_r8(&mut ctx, Reg8::E, Reg8::B),
            0x59 => ld_r8_r8(&mut ctx, Reg8::E, Reg8::C),
            0x5A => ld_r8_r8(&mut ctx, Reg8::E, Reg8::D),
            0x5B => 4, // LD E, E
            0x5C => ld_r8_r8(&mut ctx, Reg8::E, Reg8::H),
            0x5D => ld_r8_r8(&mut ctx, Reg8::E, Reg8::L),
            0x5E => ld_r8_mem_r16(&mut ctx, Reg8::E, Reg16::HL),
            0x5F => ld_r8_r8(&mut ctx, Reg8::E, Reg8::A),
            0x60 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::B),
            0x61 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::C),
            0x62 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::D),
            0x63 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::E),
            0x64 => 4, // LD H, H
            0x65 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::L),
            0x66 => ld_r8_mem_r16(&mut ctx, Reg8::H, Reg16::HL),
            0x67 => ld_r8_r8(&mut ctx, Reg8::H, Reg8::A),
            0x68 => ld_r8_r8(&mut ctx, Reg8::L, Reg8::B),
            0x69 => ld_r8_r8(&mut ctx, Reg8::L, Reg8::C),
            0x6A => ld_r8_r8(&mut ctx, Reg8::L, Reg8::D),
            0x6B => ld_r8_r8(&mut ctx, Reg8::L, Reg8::E),
            0x6C => ld_r8_r8(&mut ctx, Reg8::L, Reg8::H),
            0x6D => 4, // LD L, L
            0x6E => ld_r8_mem_r16(&mut ctx, Reg8::L, Reg16::HL),
            0x6F => ld_r8_r8(&mut ctx, Reg8::L, Reg8::A),
            0x70 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::B),
            0x71 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::C),
            0x72 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::D),
            0x73 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::E),
            0x74 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::H),
            0x75 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::L),
            0x76 => halt(&mut ctx),
            0x77 => ld_mem_r16_r8(&mut ctx, Reg16::HL, Reg8::A),
            0x78 => ld_r8_r8(&mut ctx, Reg8::A, Reg8::B),
            0x79 => ld_r8_r8(&mut ctx, Reg8::A, Reg8::C),
            0x7A => ld_r8_r8(&mut ctx, Reg8::A, Reg8::D),
            0x7B => ld_r8_r8(&mut ctx, Reg8::A, Reg8::E),
            0x7C => ld_r8_r8(&mut ctx, Reg8::A, Reg8::H),
            0x7D => ld_r8_r8(&mut ctx, Reg8::A, Reg8::L),
            0x7E => ld_r8_mem_r16(&mut ctx, Reg8::A, Reg16::HL),
            0x7F => 4, // LD A, A
            0x80 => add_r8(&mut ctx, Reg8::B),
            0x81 => add_r8(&mut ctx, Reg8::C),
            0x82 => add_r8(&mut ctx, Reg8::D),
            0x83 => add_r8(&mut ctx, Reg8::E),
            0x84 => add_r8(&mut ctx, Reg8::H),
            0x85 => add_r8(&mut ctx, Reg8::L),
            0x86 => add_mem_hl(&mut ctx),
            0x87 => add_r8(&mut ctx, Reg8::A),
            0x88 => adc_r8(&mut ctx, Reg8::B),
            0x89 => adc_r8(&mut ctx, Reg8::C),
            0x8A => adc_r8(&mut ctx, Reg8::D),
            0x8B => adc_r8(&mut ctx, Reg8::E),
            0x8C => adc_r8(&mut ctx, Reg8::H),
            0x8D => adc_r8(&mut ctx, Reg8::L),
            0x8E => adc_mem_hl(&mut ctx),
            0x8F => adc_r8(&mut ctx, Reg8::A),
            0x90 => sub_r8(&mut ctx, Reg8::B),
            0x91 => sub_r8(&mut ctx, Reg8::C),
            0x92 => sub_r8(&mut ctx, Reg8::D),
            0x93 => sub_r8(&mut ctx, Reg8::E),
            0x94 => sub_r8(&mut ctx, Reg8::H),
            0x95 => sub_r8(&mut ctx, Reg8::L),
            0x96 => sub_mem_hl(&mut ctx),
            0x97 => sub_r8(&mut ctx, Reg8::A),
            0x98 => sbc_r8(&mut ctx, Reg8::B),
            0x99 => sbc_r8(&mut ctx, Reg8::C),
            0x9A => sbc_r8(&mut ctx, Reg8::D),
            0x9B => sbc_r8(&mut ctx, Reg8::E),
            0x9C => sbc_r8(&mut ctx, Reg8::H),
            0x9D => sbc_r8(&mut ctx, Reg8::L),
            0x9E => sbc_mem_hl(&mut ctx),
            0x9F => sbc_r8(&mut ctx, Reg8::A),
            0xA0 => and_r8(&mut ctx, Reg8::B),
            0xA1 => and_r8(&mut ctx, Reg8::C),
            0xA2 => and_r8(&mut ctx, Reg8::D),
            0xA3 => and_r8(&mut ctx, Reg8::E),
            0xA4 => and_r8(&mut ctx, Reg8::H),
            0xA5 => and_r8(&mut ctx, Reg8::L),
            0xA6 => and_mem_hl(&mut ctx),
            0xA7 => and_r8(&mut ctx, Reg8::A),
            0xA8 => xor_r8(&mut ctx, Reg8::B),
            0xA9 => xor_r8(&mut ctx, Reg8::C),
            0xAA => xor_r8(&mut ctx, Reg8::D),
            0xAB => xor_r8(&mut ctx, Reg8::E),
            0xAC => xor_r8(&mut ctx, Reg8::H),
            0xAD => xor_r8(&mut ctx, Reg8::L),
            0xAE => xor_mem_hl(&mut ctx),
            0xAF => xor_r8(&mut ctx, Reg8::A),
            0xB0 => or_r8(&mut ctx, Reg8::B),
            0xB1 => or_r8(&mut ctx, Reg8::C),
            0xB2 => or_r8(&mut ctx, Reg8::D),
            0xB3 => or_r8(&mut ctx, Reg8::E),
            0xB4 => or_r8(&mut ctx, Reg8::H),
            0xB5 => or_r8(&mut ctx, Reg8::L),
            0xB6 => or_mem_hl(&mut ctx),
            0xB7 => or_r8(&mut ctx, Reg8::A),
            0xB8 => cp_r8(&mut ctx, Reg8::B),
            0xB9 => cp_r8(&mut ctx, Reg8::C),
            0xBA => cp_r8(&mut ctx, Reg8::D),
            0xBB => cp_r8(&mut ctx, Reg8::E),
            0xBC => cp_r8(&mut ctx, Reg8::H),
            0xBD => cp_r8(&mut ctx, Reg8::L),
            0xBE => cp_mem_hl(&mut ctx),
            0xBF => cp_r8(&mut ctx, Reg8::A),
            0xC0 => ret_cond(&mut ctx, JumpCond::NZ),
            0xC1 => pop_r16(&mut ctx, Reg16::BC),
            0xC2 => jp_cond_imm16(&mut ctx, JumpCond::NZ),
            0xC3 => jp_imm16(&mut ctx),
            0xC4 => call_cond_imm16(&mut ctx, JumpCond::NZ),
            0xC5 => push_r16(&mut ctx, Reg16::BC),
            0xC6 => add_imm8(&mut ctx),
            0xC7 => rst(&mut ctx, 0x00),
            0xC8 => ret_cond(&mut ctx, JumpCond::Z),
            0xC9 => ret(&mut ctx),
            0xCA => jp_cond_imm16(&mut ctx, JumpCond::Z),
            // 0xCB => PREFIX CB
            0xCC => call_cond_imm16(&mut ctx, JumpCond::Z),
            0xCD => call_imm16(&mut ctx),
            0xCE => adc_imm8(&mut ctx),
            0xCF => rst(&mut ctx, 0x08),
            0xD0 => ret_cond(&mut ctx, JumpCond::NC),
            0xD1 => pop_r16(&mut ctx, Reg16::DE),
            0xD2 => jp_cond_imm16(&mut ctx, JumpCond::NC),
            // 0xD3 => UNUSED
            0xD4 => call_cond_imm16(&mut ctx, JumpCond::NC),
            0xD5 => push_r16(&mut ctx, Reg16::DE),
            0xD6 => sub_imm8(&mut ctx),
            0xD7 => rst(&mut ctx, 0x10),
            0xD8 => ret_cond(&mut ctx, JumpCond::C),
            0xD9 => reti(&mut ctx),
            0xDA => jp_cond_imm16(&mut ctx, JumpCond::C),
            // 0xDB => UNUSED
            0xDC => call_cond_imm16(&mut ctx, JumpCond::C),
            // 0xDD => UNUSED
            0xDE => sbc_imm8(&mut ctx),
            0xDF => rst(&mut ctx, 0x18),
            0xE0 => ldh_mem_imm8_a(&mut ctx),
            0xE1 => pop_r16(&mut ctx, Reg16::HL),
            0xE2 => ldh_mem_c_a(&mut ctx),
            // 0xE3 => UNUSED
            // 0xE4 => UNUSED
            0xE5 => push_r16(&mut ctx, Reg16::HL),
            0xE6 => and_imm8(&mut ctx),
            0xE7 => rst(&mut ctx, 0x20),
            0xE8 => add_sp_imm8(&mut ctx),
            0xE9 => jp_hl(&mut ctx),
            0xEA => ld_mem_imm16_a(&mut ctx),
            // 0xEB => UNUSED
            // 0xEC => UNUSED
            // 0xED => UNUSED
            0xEE => xor_imm8(&mut ctx),
            0xEF => rst(&mut ctx, 0x28),
            0xF0 => ldh_a_mem_imm8(&mut ctx),
            0xF1 => pop_r16(&mut ctx, Reg16::AF),
            0xF2 => ldh_a_mem_c(&mut ctx),
            0xF3 => di(&mut ctx),
            // 0xF4 => UNUSED
            0xF5 => push_r16(&mut ctx, Reg16::AF),
            0xF6 => or_imm8(&mut ctx),
            0xF7 => rst(&mut ctx, 0x30),
            0xF8 => ld_hl_sp_imm8(&mut ctx),
            0xF9 => ld_sp_hl(&mut ctx),
            0xFA => ld_a_mem_imm16(&mut ctx),
            0xFB => ei(&mut ctx),
            // 0xFC => UNUSED
            // 0xFD => UNUSED
            0xFE => cp_imm8(&mut ctx),
            0xFF => rst(&mut ctx, 0x38),

            _ => {
                unimplemented!("Unimplemented opcode: {:02X}", opcode);
            }
        }
    }

    fn execute_cb(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let mut ctx = InstructionContext {
            cpu: self,
            memory,
        };

        match opcode {
            0x00..=0x07 => {
                match reg_cb!(opcode) {
                    Some(r) => rlc_r8(&mut ctx, r),
                    None => rlc_mem_hl(&mut ctx),
                }
            }
            0x08..=0x0F => {
                match reg_cb!(opcode) {
                    Some(r) => rrc_r8(&mut ctx, r),
                    None => rrc_mem_hl(&mut ctx),
                }
            }
            0x10..=0x17 => {
                match reg_cb!(opcode) {
                    Some(r) => rl_r8(&mut ctx, r),
                    None => rl_mem_hl(&mut ctx),
                }
            }
            0x18..=0x1F => {
                match reg_cb!(opcode) {
                    Some(r) => rr_r8(&mut ctx, r),
                    None => rr_mem_hl(&mut ctx),
                }
            }
            0x20..=0x27 => {
                match reg_cb!(opcode) {
                    Some(r) => sla_r8(&mut ctx, r),
                    None => sla_mem_hl(&mut ctx),
                }
            }
            0x28..=0x2F => {
                match reg_cb!(opcode) {
                    Some(r) => sra_r8(&mut ctx, r),
                    None => sra_mem_hl(&mut ctx),
                }
            }
            0x30..=0x37 => {
                match reg_cb!(opcode) {
                    Some(r) => swap_r8(&mut ctx, r),
                    None => swap_mem_hl(&mut ctx),
                }
            }
            0x38..=0x3F => {
                match reg_cb!(opcode) {
                    Some(r) => srl_r8(&mut ctx, r),
                    None => srl_mem_hl(&mut ctx),
                }
            }
            0x40..=0x7F => {
                let bit = (opcode - 0x40) / 8;
                match reg_cb!(opcode) {
                    Some(r) => bit_imm3_r8(&mut ctx, bit, r),
                    None => bit_imm3_mem_hl(&mut ctx, bit),
                }
            }
            0x80..=0xBF => {
                let bit = (opcode - 0x80) / 8;
                match reg_cb!(opcode) {
                    Some(r) => res_imm3_r8(&mut ctx, bit, r),
                    None => res_imm3_mem_hl(&mut ctx, bit),
                }
            }
            0xC0..=0xFF => {
                let bit = (opcode - 0xC0) / 8;
                match reg_cb!(opcode) {
                    Some(r) => set_imm3_r8(&mut ctx, bit, r),
                    None => set_imm3_mem_hl(&mut ctx, bit),
                }
            }
        }
    }

    #[cfg(feature = "debug_logging")]
    #[inline]
    fn write_log_line(&mut self, memory: &Memory) {
        let pc0 = memory.read(self.pc);
        let pc1 = memory.read(self.pc.wrapping_add(1));
        let pc2 = memory.read(self.pc.wrapping_add(2));
        let pc3 = memory.read(self.pc.wrapping_add(3));

        writeln!(
            self.log_writer,
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.regs.a(),
            self.regs.f(),
            self.regs.b(),
            self.regs.c(),
            self.regs.d(),
            self.regs.e(),
            self.regs.h(),
            self.regs.l(),
            self.sp,
            self.pc,
            pc0, pc1, pc2, pc3,
        ).unwrap();
    }
}