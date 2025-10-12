use super::registers::Registers;
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

    pub fn step(&mut self, memory: &mut dyn Bus) -> u8 {
        let opcode = memory.read(self.regs.pc());
        self.regs.inc_pc(1);
        self.execute(opcode, memory)
    }

    /// Returns the number of cycles the instruction took
    pub fn execute(&mut self, opcode: u8, memory: &mut dyn Bus) -> u8 {
        match opcode {
            0x00 => 4, // NOP

            0x3E => { // LD A, u8
                let value = memory.read(self.regs.pc());
                self.regs.set_a(value);
                self.regs.inc_pc(1);
                8
            }

            0x06 => { // LD B, u8
                let value = memory.read(self.regs.pc());
                self.regs.set_b(value);
                self.regs.inc_pc(1);
                8
            }

            0x80 => { // ADD A, B
                let a = self.regs.a();
                let b = self.regs.b();
                let (result, carry) = a.overflowing_add(b);
                self.regs.set_a(result);

                self.regs.set_zero_flag(result == 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag((a & 0x0F) + (b & 0x0F) > 0x0F);
                self.regs.set_carry_flag(carry);
                4
            }

            _ => {
                println!("Unimplemented opcode: {:02X}", opcode);
                0
            }
        }
    }
}