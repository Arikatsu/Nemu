use crate::cpu::Cpu;
use crate::cpu::registers::{Reg16};
use crate::traits::Bus;

/// POP r16 - Pop 16-bit value from stack into 16-bit register
pub(in crate::cpu) fn pop_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.memory.borrow().read_u16(cpu.sp);
    cpu.regs.write_reg16(reg, value);
    cpu.inc_sp(2);
    12
}

/// PUSH r16 - Push 16-bit register value onto stack
pub(in crate::cpu) fn push_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let sp = cpu.sp.wrapping_sub(2);
    let value = cpu.regs.read_reg16(reg);
    cpu.memory.borrow_mut().write_u16(sp, value);
    cpu.set_sp(sp);
    16
}