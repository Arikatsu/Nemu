use super::InstructionContext;
use crate::cpu::registers::{Reg16};

/// POP r16 - Pop 16-bit value from stack into 16-bit register
pub(in crate::cpu) fn pop_r16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let value = ctx.memory.read_u16(ctx.cpu.sp);
    ctx.cpu.regs.write_reg16(reg, value);
    ctx.cpu.inc_sp(2);
    12
}

/// PUSH r16 - Push 16-bit register value onto stack
pub(in crate::cpu) fn push_r16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let sp = ctx.cpu.sp.wrapping_sub(2);
    let value = ctx.cpu.regs.read_reg16(reg);
    ctx.memory.write_u16(sp, value);
    ctx.cpu.set_sp(sp);
    16
}