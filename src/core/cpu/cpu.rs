use super::registers::{Reg8, Reg16, Registers};
use super::opcodes;
use crate::core::bus::Bus;

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
            0x3E => opcodes::ld_r8_imm8(self, Reg8::A),
            0x80 => opcodes::add_a_r8(self, Reg8::B),

            _ => {
                println!("Unimplemented opcode: {:02X}", opcode);
                0
            }
        }
    }
}