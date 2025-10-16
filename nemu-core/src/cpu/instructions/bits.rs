use crate::cpu::Cpu;
use crate::traits::Bus;

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(in crate::cpu) fn rlca<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let carry = (a & 0x80) != 0;
    let result = (a << 1) | carry as u8;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    4
}

/// RRCA - Rotate A right, old bit 0 to Carry flag
pub(in crate::cpu) fn rrca<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let carry = (a & 0x01) != 0;
    let result = (a >> 1) | ((carry as u8) << 7);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    4
}

/// RLA - Rotate A left through Carry flag
pub(in crate::cpu) fn rla<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let carry = cpu.regs.carry_flag();
    let new_carry = (a & 0x80) != 0;
    let result = (a << 1) | (carry as u8);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    4
}

/// RRA - Rotate A right through Carry flag
pub(in crate::cpu) fn rra<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let carry = cpu.regs.carry_flag();
    let new_carry = (a & 0x01) != 0;
    let result = (a >> 1) | ((carry as u8) << 7);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    4
}