use super::CPU;
use super::registers::{Reg8, Reg16};
use crate::core::bus::Bus;

/// LD r8, r8 - Load 8-bit register value into another 8-bit register
pub(super) fn ld_r8_r8<B: Bus>(cpu: &mut CPU<B>, dest: Reg8, src: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(src);
    cpu.regs.write_reg8(dest, value);
    4
}

/// LD r8, imm - Load immediate 8-bit value into 8-bit register
pub(super) fn ld_r8_imm8<B: Bus>(cpu: &mut CPU<B>, reg: Reg8) -> u8 {
    let value = cpu.memory.read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    cpu.regs.write_reg8(reg, value);
    8
}

/// LD r16, imm - Load immediate 16-bit value into 16-bit register
pub(super) fn ld_r16_imm16<B: Bus>(cpu: &mut CPU<B>, reg: Reg16) -> u8 {
    let value = cpu.memory.read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    cpu.regs.write_reg16(reg, value);
    12
}

/// LD (r16), r8 - Store 8-bit register value at memory address in 16-bit register
pub(super) fn ld_mem_r16_r8<B: Bus>(cpu: &mut CPU<B>, addr_reg: Reg16, src: Reg8) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.regs.read_reg8(src);
    cpu.memory.write(addr, value);
    8
}

/// INC r16 - Increment 16-bit register
pub(super) fn inc_r16<B: Bus>(cpu: &mut CPU<B>, reg: Reg16) -> u8 {
    let value = cpu.regs.read_reg16(reg);
    cpu.regs.write_reg16(reg, value.wrapping_add(1));
    8
}

/// INC r8 - Increment 8-bit register
pub(super) fn inc_r8<B: Bus>(cpu: &mut CPU<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    4
}

/// DEC r8 - Decrement 8-bit register
pub(super) fn dec_r8<B: Bus>(cpu: &mut CPU<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
    4
}

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(super) fn rlca<B: Bus>(cpu: &mut CPU<B>) -> u8 {
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

pub(super) fn add_a_r8<B: Bus>(cpu: &mut CPU<B>, reg: Reg8) -> u8 {
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
