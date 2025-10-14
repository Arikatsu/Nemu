use super::registers::{Reg8, Reg16, Registers};
use super::opcodes;
use crate::bus::Bus;

pub struct CPU<B: Bus> {
    pub(crate) regs: Registers,
    pub(crate) memory: B,
}

impl<B: Bus> CPU<B> {
    pub fn new(memory: B) -> Self {
        Self {
            regs: Registers::new(),
            memory,
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.memory.read(self.regs.pc());
        self.regs.inc_pc(1);
        self.execute(opcode)
    }

    fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => 4, // NOP
            0x01 => opcodes::ld_r16_imm16(self, Reg16::BC),
            0x02 => opcodes::ld_mem_r16_r8(self, Reg16::BC, Reg8::A),
            0x03 => opcodes::inc_r16(self, Reg16::BC),
            0x04 => opcodes::inc_r8(self, Reg8::B),
            0x05 => opcodes::dec_r8(self, Reg8::B),
            0x06 => opcodes::ld_r8_imm8(self, Reg8::B),
            0x07 => opcodes::rlca(self),
            0x08 => opcodes::ld_mem_imm16_sp(self),
            0x0A => opcodes::ld_r8_mem_r16(self, Reg8::A, Reg16::BC),
            0x0E => opcodes::ld_r8_imm8(self, Reg8::C),
            0x12 => opcodes::ld_mem_r16_r8(self, Reg16::DE, Reg8::A),
            0x16 => opcodes::ld_r8_imm8(self, Reg8::D),
            0x1A => opcodes::ld_r8_mem_r16(self, Reg8::A, Reg16::DE),
            0x1E => opcodes::ld_r8_imm8(self, Reg8::E),
            0x22 => opcodes::ld_mem_hli_a(self),
            0x26 => opcodes::ld_r8_imm8(self, Reg8::H),
            0x2A => opcodes::ld_a_mem_hli(self),
            0x2E => opcodes::ld_r8_imm8(self, Reg8::L),
            0x32 => opcodes::ld_mem_hld_a(self),
            0x36 => opcodes::ld_mem_r16_imm8(self, Reg16::HL),
            0x3A => opcodes::ld_a_mem_hld(self),
            0x3E => opcodes::ld_r8_imm8(self, Reg8::A),
            0x40 => 4, // LD B, B (lmao....)
            0x41 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::C),
            0x42 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::D),
            0x43 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::E),
            0x44 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::H),
            0x45 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::L),
            0x46 => opcodes::ld_r8_mem_r16(self, Reg8::B, Reg16::HL),
            0x47 => opcodes::ld_r8_r8(self, Reg8::B, Reg8::A),
            0x48 => opcodes::ld_r8_r8(self, Reg8::C, Reg8::B),
            0x49 => 4, // LD C, C
            0x4A => opcodes::ld_r8_r8(self, Reg8::C, Reg8::D),
            0x4B => opcodes::ld_r8_r8(self, Reg8::C, Reg8::E),
            0x4C => opcodes::ld_r8_r8(self, Reg8::C, Reg8::H),
            0x4D => opcodes::ld_r8_r8(self, Reg8::C, Reg8::L),
            0x4E => opcodes::ld_r8_mem_r16(self, Reg8::C, Reg16::HL),
            0x4F => opcodes::ld_r8_r8(self, Reg8::C, Reg8::A),
            0x50 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::B),
            0x51 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::C),
            0x52 => 4, // LD D, D
            0x53 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::E),
            0x54 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::H),
            0x55 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::L),
            0x56 => opcodes::ld_r8_mem_r16(self, Reg8::D, Reg16::HL),
            0x57 => opcodes::ld_r8_r8(self, Reg8::D, Reg8::A),
            0x58 => opcodes::ld_r8_r8(self, Reg8::E, Reg8::B),
            0x59 => opcodes::ld_r8_r8(self, Reg8::E, Reg8::C),
            0x5A => opcodes::ld_r8_r8(self, Reg8::E, Reg8::D),
            0x5B => 4, // LD E, E
            0x5C => opcodes::ld_r8_r8(self, Reg8::E, Reg8::H),
            0x5D => opcodes::ld_r8_r8(self, Reg8::E, Reg8::L),
            0x5E => opcodes::ld_r8_mem_r16(self, Reg8::E, Reg16::HL),
            0x5F => opcodes::ld_r8_r8(self, Reg8::E, Reg8::A),
            0x60 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::B),
            0x61 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::C),
            0x62 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::D),
            0x63 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::E),
            0x64 => 4, // LD H, H
            0x65 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::L),
            0x66 => opcodes::ld_r8_mem_r16(self, Reg8::H, Reg16::HL),
            0x67 => opcodes::ld_r8_r8(self, Reg8::H, Reg8::A),
            0x68 => opcodes::ld_r8_r8(self, Reg8::L, Reg8::B),
            0x69 => opcodes::ld_r8_r8(self, Reg8::L, Reg8::C),
            0x6A => opcodes::ld_r8_r8(self, Reg8::L, Reg8::D),
            0x6B => opcodes::ld_r8_r8(self, Reg8::L, Reg8::E),
            0x6C => opcodes::ld_r8_r8(self, Reg8::L, Reg8::H),
            0x6D => 4, // LD L, L
            0x6E => opcodes::ld_r8_mem_r16(self, Reg8::L, Reg16::HL),
            0x6F => opcodes::ld_r8_r8(self, Reg8::L, Reg8::A),
            0x70 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::B),
            0x71 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::C),
            0x72 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::D),
            0x73 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::E),
            0x74 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::H),
            0x75 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::L),
            0x77 => opcodes::ld_mem_r16_r8(self, Reg16::HL, Reg8::A),
            0x78 => opcodes::ld_r8_r8(self, Reg8::A, Reg8::B),
            0x79 => opcodes::ld_r8_r8(self, Reg8::A, Reg8::C),
            0x7A => opcodes::ld_r8_r8(self, Reg8::A, Reg8::D),
            0x7B => opcodes::ld_r8_r8(self, Reg8::A, Reg8::E),
            0x7C => opcodes::ld_r8_r8(self, Reg8::A, Reg8::H),
            0x7D => opcodes::ld_r8_r8(self, Reg8::A, Reg8::L),
            0x76 => opcodes::ld_r8_mem_r16(self, Reg8::A, Reg16::HL),
            0x7F => 4, // LD A, A
            0x80 => opcodes::add_a_r8(self, Reg8::B),

            _ => {
                panic!("Unimplemented opcode: {:02X}", opcode);
            }
        }
    }
}