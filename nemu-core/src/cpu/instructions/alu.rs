use crate::cpu::Cpu;
use crate::cpu::registers::{Reg16, Reg8};
use crate::traits::Bus;

/// INC r8 - Increment 8-bit register
pub(in crate::cpu) fn inc_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    4
}

/// INC r16 - Increment 16-bit register
pub(in crate::cpu) fn inc_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.regs.read_reg16(reg);
    cpu.regs.write_reg16(reg, value.wrapping_add(1));
    8
}


/// DEC r8 - Decrement 8-bit register
pub(in crate::cpu) fn dec_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
    4
}

/// DEC r16 - Decrement 16-bit register
pub(in crate::cpu) fn dec_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.regs.read_reg16(reg);
    cpu.regs.write_reg16(reg, value.wrapping_sub(1));
    8
}

/// ADD A, r8 - Add 8-bit register value to A
pub(in crate::cpu) fn add_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, carry) = a.overflowing_add(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag(carry);
    4
}

/// ADD A, (HL) - Add value at address in HL to A
pub(in crate::cpu) fn add_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let (result, carry) = a.overflowing_add(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag(carry);
    8
}