use super::CPU;
use super::registers::{Reg8, Reg16};
use crate::core::bus::Bus;

/// LD r8, r8 - Load 8-bit register value into another 8-bit register
pub(super) fn ld_r8_r8<B: Bus>(cpu: &mut CPU<B>, dest: Reg8, src: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(src);
    cpu.regs.write_reg8(dest, value);
    4
}

/// LD r8, imm8 - Load immediate 8-bit value into 8-bit register
pub(super) fn ld_r8_imm8<B: Bus>(cpu: &mut CPU<B>, reg: Reg8) -> u8 {
    let value = cpu.memory.read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    cpu.regs.write_reg8(reg, value);
    8
}

/// LD r8, (r16) - Load 8-bit value from memory address in 16-bit register into 8-bit register
pub(super) fn ld_r8_mem_r16<B: Bus>(cpu: &mut CPU<B>, dest: Reg8, addr_reg: Reg16) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.memory.read(addr);
    cpu.regs.write_reg8(dest, value);
    8
}

/// LD r16, imm16 - Load immediate 16-bit value into 16-bit register
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

/// LD (r16), imm8 - Store immediate 8-bit value at memory address in 16-bit register
pub(super) fn ld_mem_r16_imm8<B: Bus>(cpu: &mut CPU<B>, addr_reg: Reg16) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.memory.read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    cpu.memory.write(addr, value);
    12
}

/// LD (HL+), A - Store A at address in HL, then increment HL
pub(super) fn ld_mem_hli_a<B: Bus>(cpu: &mut CPU<B>) -> u8 {
    let addr = cpu.regs.hl();
    cpu.memory.write(addr, cpu.regs.a());
    cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD (HL-), A - Store A at address in HL, then decrement HL
pub(super) fn ld_mem_hld_a<B: Bus>(cpu: &mut CPU<B>) -> u8 {
    let addr = cpu.regs.hl();
    cpu.memory.write(addr, cpu.regs.a());
    cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD A, (HL+) - Load A from address in HL, then increment HL
pub(super) fn ld_a_mem_hli<B: Bus>(cpu: &mut CPU<B>) -> u8 {
    let addr = cpu.regs.hl();
    let value = cpu.memory.read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD A, (HL-) - Load A from address in HL, then decrement HL
pub(super) fn ld_a_mem_hld<B: Bus>(cpu: &mut CPU<B>) -> u8 {
    let addr = cpu.regs.hl();
    let value = cpu.memory.read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD (imm16), SP - Store SP at immediate 16-bit address
pub(super) fn ld_mem_imm16_sp<B: Bus>(cpu: &mut CPU<B>) -> u8 {
    let addr = cpu.memory.read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    cpu.memory.write_u16(addr, cpu.regs.sp());
    20
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
