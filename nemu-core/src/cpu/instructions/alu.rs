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

/// INC SP - Increment Stack Pointer
pub(in crate::cpu) fn inc_sp<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.inc_sp(1);
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

/// DEC SP - Decrement Stack Pointer
pub(in crate::cpu) fn dec_sp<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.dec_sp(1);
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

/// ADC A, r8 - Add 8-bit register value + Carry flag to A
pub(in crate::cpu) fn adc_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    cpu.regs.set_carry_flag(carry1 || carry2);
    4
}

/// ADC A, (HL) - Add value at address in HL + Carry flag to A
pub(in crate::cpu) fn adc_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    cpu.regs.set_carry_flag(carry1 || carry2);
    8
}

/// SUB r8 - Subtract 8-bit register value from A
pub(in crate::cpu) fn sub_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
    4
}

/// SUB (HL) - Subtract value at address in HL from A
pub(in crate::cpu) fn sub_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let (result, borrow) = a.overflowing_sub(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
    8
}

/// SBC A, r8 - Subtract 8-bit register value + Carry flag from A
pub(in crate::cpu) fn sbc_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    cpu.regs.set_carry_flag(borrow1 || borrow2);
    4
}

/// SBC A, (HL) - Subtract value at address in HL + Carry flag from A
pub(in crate::cpu) fn sbc_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    cpu.regs.set_carry_flag(borrow1 || borrow2);
    8
}

/// AND A, r8 - Logical AND 8-bit register value with A
pub(in crate::cpu) fn and_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a & value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);
    4
}

/// AND A, (HL) - Logical AND value at address in HL with A
pub(in crate::cpu) fn and_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let result = a & value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);
    8
}

/// XOR A, r8 - Logical XOR 8-bit register value with A
pub(in crate::cpu) fn xor_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a ^ value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    4
}

/// XOR A, (HL) - Logical XOR value at address in HL with A
pub(in crate::cpu) fn xor_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let result = a ^ value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    8
}

/// OR A, r8 - Logical OR 8-bit register value with A
pub(in crate::cpu) fn or_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a | value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    4
}

/// OR A, (HL) - Logical OR value at address in HL with A
pub(in crate::cpu) fn or_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let result = a | value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    8
}

/// CP A, r8 - Compare 8-bit register value with A
pub(in crate::cpu) fn cp_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
    4
}

/// CP A, (HL) - Compare value at address in HL with A
pub(in crate::cpu) fn cp_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    let (result, borrow) = a.overflowing_sub(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
    8
}