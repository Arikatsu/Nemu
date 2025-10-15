use crate::cpu::Cpu;
use crate::traits::Bus;

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(in crate::cpu) fn rlca<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let carry = (a & 0x80) != 0;
    let result = (a << 1) | if carry { 1 } else { 0 };

    cpu.regs.set_a(result);
    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    4
}