use super::registers::{Reg8, Reg16, Registers};
use super::opcodes;
use crate::core::bus::Bus;

pub struct CPU {
    pub(crate) regs: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
    }

    pub fn step<B: Bus>(&mut self, memory: &mut B) -> u8 {
        let opcode = memory.read(self.regs.pc());
        self.regs.inc_pc(1);
        self.execute(opcode, memory)
    }

    fn execute<B: Bus>(&mut self, opcode: u8, memory: &mut B) -> u8 {
        match opcode {
            0x00 => 4, // NOP
            0x01 => opcodes::ld_r16_imm(&mut self.regs, Reg16::BC, memory),
            0x02 => opcodes::ld_mem_r16_r8(&mut self.regs, Reg16::BC, Reg8::A, memory),
            0x03 => opcodes::inc_r16(&mut self.regs, Reg16::BC),
            0x04 => opcodes::inc_r8(&mut self.regs, Reg8::B),
            0x05 => opcodes::dec_r8(&mut self.regs, Reg8::B),
            0x06 => opcodes::ld_r8_imm(&mut self.regs, Reg8::B, memory),
            0x07 => opcodes::rlca(&mut self.regs),
            0x3E => opcodes::ld_r8_imm(&mut self.regs, Reg8::A, memory),
            0x80 => opcodes::add_a_r8(&mut self.regs, Reg8::B),

            _ => {
                println!("Unimplemented opcode: {:02X}", opcode);
                0
            }
        }
    }
}