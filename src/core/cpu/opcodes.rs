use super::registers::{Reg8, Reg16, Registers};
use crate::core::bus::Bus;

/// LD r8, imm - Load immediate 8-bit value into 8-bit register
pub(super) fn ld_r8_imm(regs: &mut Registers, reg: Reg8, memory: &mut dyn Bus) -> u8 {
    let value = memory.read(regs.pc());
    regs.inc_pc(1);
    regs.write_reg8(reg, value);
    8
}

/// LD r16, imm - Load immediate 16-bit value into 16-bit register
pub(super) fn ld_r16_imm(regs: &mut Registers, reg: Reg16, memory: &mut dyn Bus) -> u8 {
    let low = memory.read(regs.pc()) as u16;
    let high = memory.read(regs.pc() + 1) as u16;
    let value = (high << 8) | low;
    regs.inc_pc(2);
    regs.write_reg16(reg, value);
    12
}

/// LD (r16), r8 - Store 8-bit register value at memory address in 16-bit register
pub(super) fn ld_mem_r16_r8(regs: &mut Registers, addr_reg: Reg16, src: Reg8, memory: &mut dyn Bus) -> u8 {
    let addr = regs.read_reg16(addr_reg);
    let value = regs.read_reg8(src);
    memory.write(addr, value);
    8
}

/// INC r16 - Increment 16-bit register
pub(super) fn inc_r16(regs: &mut Registers, reg: Reg16) -> u8 {
    let value = regs.read_reg16(reg);
    regs.write_reg16(reg, value.wrapping_add(1));
    8
}

/// INC r8 - Increment 8-bit register
pub(super) fn inc_r8(regs: &mut Registers, reg: Reg8) -> u8 {
    let value = regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    regs.write_reg8(reg, result);
    regs.set_zero_flag(result == 0);
    regs.set_subtract_flag(false);
    regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    4
}

/// DEC r8 - Decrement 8-bit register
pub(super) fn dec_r8(regs: &mut Registers, reg: Reg8) -> u8 {
    let value = regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    regs.write_reg8(reg, result);
    regs.set_zero_flag(result == 0);
    regs.set_subtract_flag(true);
    regs.set_half_carry_flag((value & 0x0F) == 0);
    4
}

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(super) fn rlca(regs: &mut Registers) -> u8 {
    let a = regs.a();
    let carry = (a & 0x80) != 0;
    let result = (a << 1) | if carry { 1 } else { 0 };

    regs.set_a(result);
    regs.set_zero_flag(false);
    regs.set_subtract_flag(false);
    regs.set_half_carry_flag(false);
    regs.set_carry_flag(carry);
    4
}

pub(super) fn add_a_r8(regs: &mut Registers, reg: Reg8) -> u8 {
    let a = regs.a();
    let value = regs.read_reg8(reg);
    let (result, carry) = a.overflowing_add(value);
    regs.set_a(result);

    regs.set_zero_flag(result == 0);
    regs.set_subtract_flag(false);
    regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    regs.set_carry_flag(carry);
    4
}
