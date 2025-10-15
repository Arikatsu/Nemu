mod registers;
mod opcodes;

use opcodes::*;
use registers::{Reg8, Reg16, Registers};
use crate::traits::Bus;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpu<B: Bus> {
    pub(crate) regs: Registers,
    pub(crate) memory: Rc<RefCell<B>>,
    pub(crate) sp: u16,   // Stack Pointer
    pub(crate) pc: u16,   // Program Counter
    pub(crate) ime: bool, // Interrupt Master Enable
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: Rc<RefCell<B>>) -> Self {
        Self {
            regs: Registers::new(),
            memory: bus,
            sp: 0xFFFE,
            pc: 0x0100,
            ime: false,
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.memory.borrow().read(self.pc);
        self.inc_pc(1);
        self.execute(opcode)
    }

    fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => 4, // NOP
            0x01 => ld_r16_imm16(self, Reg16::BC),
            0x02 => ld_mem_r16_r8(self, Reg16::BC, Reg8::A),
            0x03 => inc_r16(self, Reg16::BC),
            0x04 => inc_r8(self, Reg8::B),
            0x05 => dec_r8(self, Reg8::B),
            0x06 => ld_r8_imm8(self, Reg8::B),
            0x07 => rlca(self),
            0x08 => ld_mem_imm16_sp(self),
            0x0A => ld_r8_mem_r16(self, Reg8::A, Reg16::BC),
            0x0C => inc_r8(self, Reg8::C),
            0x0D => dec_r8(self, Reg8::C),
            0x0E => ld_r8_imm8(self, Reg8::C),
            0x11 => ld_r16_imm16(self, Reg16::DE),
            0x12 => ld_mem_r16_r8(self, Reg16::DE, Reg8::A),
            0x13 => inc_r16(self, Reg16::DE),
            0x14 => inc_r8(self, Reg8::D),
            0x15 => dec_r8(self, Reg8::D),
            0x16 => ld_r8_imm8(self, Reg8::D),
            0x1A => ld_r8_mem_r16(self, Reg8::A, Reg16::DE),
            0x1C => inc_r8(self, Reg8::E),
            0x1E => ld_r8_imm8(self, Reg8::E),
            0x20 => jr_nz_imm8(self),
            0x21 => ld_r16_imm16(self, Reg16::HL),
            0x22 => ld_mem_hli_a(self),
            0x26 => ld_r8_imm8(self, Reg8::H),
            0x2A => ld_a_mem_hli(self),
            0x2E => ld_r8_imm8(self, Reg8::L),
            0x31 => ld_sp_imm16(self),
            0x32 => ld_mem_hld_a(self),
            0x36 => ld_mem_r16_imm8(self, Reg16::HL),
            0x3A => ld_a_mem_hld(self),
            0x3E => ld_r8_imm8(self, Reg8::A),
            0x40 => 4, // LD B, B (lmao....)
            0x41 => ld_r8_r8(self, Reg8::B, Reg8::C),
            0x42 => ld_r8_r8(self, Reg8::B, Reg8::D),
            0x43 => ld_r8_r8(self, Reg8::B, Reg8::E),
            0x44 => ld_r8_r8(self, Reg8::B, Reg8::H),
            0x45 => ld_r8_r8(self, Reg8::B, Reg8::L),
            0x46 => ld_r8_mem_r16(self, Reg8::B, Reg16::HL),
            0x47 => ld_r8_r8(self, Reg8::B, Reg8::A),
            0x48 => ld_r8_r8(self, Reg8::C, Reg8::B),
            0x49 => 4, // LD C, C
            0x4A => ld_r8_r8(self, Reg8::C, Reg8::D),
            0x4B => ld_r8_r8(self, Reg8::C, Reg8::E),
            0x4C => ld_r8_r8(self, Reg8::C, Reg8::H),
            0x4D => ld_r8_r8(self, Reg8::C, Reg8::L),
            0x4E => ld_r8_mem_r16(self, Reg8::C, Reg16::HL),
            0x4F => ld_r8_r8(self, Reg8::C, Reg8::A),
            0x50 => ld_r8_r8(self, Reg8::D, Reg8::B),
            0x51 => ld_r8_r8(self, Reg8::D, Reg8::C),
            0x52 => 4, // LD D, D
            0x53 => ld_r8_r8(self, Reg8::D, Reg8::E),
            0x54 => ld_r8_r8(self, Reg8::D, Reg8::H),
            0x55 => ld_r8_r8(self, Reg8::D, Reg8::L),
            0x56 => ld_r8_mem_r16(self, Reg8::D, Reg16::HL),
            0x57 => ld_r8_r8(self, Reg8::D, Reg8::A),
            0x58 => ld_r8_r8(self, Reg8::E, Reg8::B),
            0x59 => ld_r8_r8(self, Reg8::E, Reg8::C),
            0x5A => ld_r8_r8(self, Reg8::E, Reg8::D),
            0x5B => 4, // LD E, E
            0x5C => ld_r8_r8(self, Reg8::E, Reg8::H),
            0x5D => ld_r8_r8(self, Reg8::E, Reg8::L),
            0x5E => ld_r8_mem_r16(self, Reg8::E, Reg16::HL),
            0x5F => ld_r8_r8(self, Reg8::E, Reg8::A),
            0x60 => ld_r8_r8(self, Reg8::H, Reg8::B),
            0x61 => ld_r8_r8(self, Reg8::H, Reg8::C),
            0x62 => ld_r8_r8(self, Reg8::H, Reg8::D),
            0x63 => ld_r8_r8(self, Reg8::H, Reg8::E),
            0x64 => 4, // LD H, H
            0x65 => ld_r8_r8(self, Reg8::H, Reg8::L),
            0x66 => ld_r8_mem_r16(self, Reg8::H, Reg16::HL),
            0x67 => ld_r8_r8(self, Reg8::H, Reg8::A),
            0x68 => ld_r8_r8(self, Reg8::L, Reg8::B),
            0x69 => ld_r8_r8(self, Reg8::L, Reg8::C),
            0x6A => ld_r8_r8(self, Reg8::L, Reg8::D),
            0x6B => ld_r8_r8(self, Reg8::L, Reg8::E),
            0x6C => ld_r8_r8(self, Reg8::L, Reg8::H),
            0x6D => 4, // LD L, L
            0x6E => ld_r8_mem_r16(self, Reg8::L, Reg16::HL),
            0x6F => ld_r8_r8(self, Reg8::L, Reg8::A),
            0x70 => ld_mem_r16_r8(self, Reg16::HL, Reg8::B),
            0x71 => ld_mem_r16_r8(self, Reg16::HL, Reg8::C),
            0x72 => ld_mem_r16_r8(self, Reg16::HL, Reg8::D),
            0x73 => ld_mem_r16_r8(self, Reg16::HL, Reg8::E),
            0x74 => ld_mem_r16_r8(self, Reg16::HL, Reg8::H),
            0x75 => ld_mem_r16_r8(self, Reg16::HL, Reg8::L),
            0x77 => ld_mem_r16_r8(self, Reg16::HL, Reg8::A),
            0x78 => ld_r8_r8(self, Reg8::A, Reg8::B),
            0x79 => ld_r8_r8(self, Reg8::A, Reg8::C),
            0x7A => ld_r8_r8(self, Reg8::A, Reg8::D),
            0x7B => ld_r8_r8(self, Reg8::A, Reg8::E),
            0x7C => ld_r8_r8(self, Reg8::A, Reg8::H),
            0x7D => ld_r8_r8(self, Reg8::A, Reg8::L),
            0x76 => ld_r8_mem_r16(self, Reg8::A, Reg16::HL),
            0x7F => 4, // LD A, A
            0x80 => add_a_r8(self, Reg8::B),
            0x81 => add_a_r8(self, Reg8::C),
            0x82 => add_a_r8(self, Reg8::D),
            0x83 => add_a_r8(self, Reg8::E),
            0x84 => add_a_r8(self, Reg8::H),
            0x85 => add_a_r8(self, Reg8::L),
            0x86 => add_a_mem_hl(self),
            0x87 => add_a_r8(self, Reg8::A),
            0xC1 => pop_r16(self, Reg16::BC),
            0xC3 => jp_imm16(self),
            0xC5 => push_r16(self, Reg16::BC),
            0xD1 => pop_r16(self, Reg16::DE),
            0xD5 => push_r16(self, Reg16::DE),
            0xE0 => ldh_mem_imm8_a(self),
            0xE1 => pop_r16(self, Reg16::HL),
            0xE2 => ldh_mem_c_a(self),
            0xE5 => push_r16(self, Reg16::HL),
            0xEA => ld_mem_imm16_a(self),
            0xF0 => ldh_a_mem_imm8(self),
            0xF1 => pop_r16(self, Reg16::AF),
            0xF2 => ldh_a_mem_c(self),
            0xF5 => push_r16(self, Reg16::AF),
            0xF8 => ld_hl_sp_imm8(self),
            0xF9 => ld_sp_hl(self),

            _ => {
                unimplemented!("Unimplemented opcode: {:02X}", opcode);
            }
        }
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
}