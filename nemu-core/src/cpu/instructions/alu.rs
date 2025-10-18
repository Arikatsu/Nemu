use super::InstructionContext;
use crate::cpu::registers::{Reg16, Reg8};

/// INC r8 - Increment 8-bit register
pub(in crate::cpu) fn inc_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let value = ctx.cpu.regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    ctx.cpu.regs.write_reg8(reg, result);
    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    4
}

/// INC r16 - Increment 16-bit register
pub(in crate::cpu) fn inc_r16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let value = ctx.cpu.regs.read_reg16(reg);
    ctx.cpu.regs.write_reg16(reg, value.wrapping_add(1));
    8
}

/// INC SP - Increment Stack Pointer
pub(in crate::cpu) fn inc_sp(ctx: &mut InstructionContext) -> u8 {
    ctx.cpu.regs.inc_sp(1);
    8
}

/// INC (HL) - Increment value at address in HL
pub(in crate::cpu) fn inc_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let result = value.wrapping_add(1);
    ctx.memory.write(addr, result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    12
}

/// DEC r8 - Decrement 8-bit register
pub(in crate::cpu) fn dec_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let value = ctx.cpu.regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    ctx.cpu.regs.write_reg8(reg, result);
    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
    4
}

/// DEC r16 - Decrement 16-bit register
pub(in crate::cpu) fn dec_r16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let value = ctx.cpu.regs.read_reg16(reg);
    ctx.cpu.regs.write_reg16(reg, value.wrapping_sub(1));
    8
}

/// DEC SP - Decrement Stack Pointer
pub(in crate::cpu) fn dec_sp(ctx: &mut InstructionContext) -> u8 {
    ctx.cpu.regs.dec_sp(1);
    8
}

/// DEC (HL) - Decrement value at address in HL
pub(in crate::cpu) fn dec_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let result = value.wrapping_sub(1);
    ctx.memory.write(addr, result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
    12
}

/// ADD A, r8 - Add 8-bit register value to A
pub(in crate::cpu) fn add_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let (result, carry) = a.overflowing_add(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry);
    4
}

/// ADD HL, r16 - Add 16-bit register value to HL
pub(in crate::cpu) fn add_hl_r16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let hl = ctx.cpu.regs.hl();
    let value = ctx.cpu.regs.read_reg16(reg);
    let (result, carry) = hl.overflowing_add(value);
    ctx.cpu.regs.set_hl(result);

    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
    ctx.cpu.regs.set_carry_flag(carry);
    8
}

/// ADD A, imm8 - Add immediate 8-bit value to A
pub(in crate::cpu) fn add_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let (result, carry) = a.overflowing_add(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry);
    8
}

/// ADD A, (HL) - Add value at address in HL to A
pub(in crate::cpu) fn add_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let (result, carry) = a.overflowing_add(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry);
    8
}

/// ADD HL, SP - Add Stack Pointer to HL
pub(in crate::cpu) fn add_hl_sp(ctx: &mut InstructionContext) -> u8 {
    let hl = ctx.cpu.regs.hl();
    let sp = ctx.cpu.regs.sp();
    let (result, carry) = hl.overflowing_add(sp);
    ctx.cpu.regs.set_hl(result);

    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((hl & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF);
    ctx.cpu.regs.set_carry_flag(carry);
    8
}

/// ADD SP, imm8 - Add immediate 8-bit signed value to Stack Pointer
pub(in crate::cpu) fn add_sp_imm8(ctx: &mut InstructionContext) -> u8 {
    let sp = ctx.cpu.regs.sp();
    let offset = ctx.memory.read(ctx.cpu.regs.pc()) as i8;
    ctx.cpu.regs.inc_pc(1);
    let result = sp.wrapping_add(offset as u16);
    ctx.cpu.regs.set_sp(result);

    ctx.cpu.regs.set_zero_flag(false);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F);
    ctx.cpu.regs.set_carry_flag((sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);
    16
}

/// ADC A, r8 - Add 8-bit register value + Carry flag to A
pub(in crate::cpu) fn adc_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry1 || carry2);
    4
}

/// ADC A, imm8 - Add immediate 8-bit value + Carry flag to A
pub(in crate::cpu) fn adc_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry1 || carry2);
    8
}

/// ADC A, (HL) - Add value at address in HL + Carry flag to A
pub(in crate::cpu) fn adc_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    ctx.cpu.regs.set_carry_flag(carry1 || carry2);
    8
}

/// SUB r8 - Subtract 8-bit register value from A
pub(in crate::cpu) fn sub_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    4
}

/// SUB imm8 - Subtract immediate 8-bit value from A
pub(in crate::cpu) fn sub_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let (result, borrow) = a.overflowing_sub(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    8
}

/// SUB (HL) - Subtract value at address in HL from A
pub(in crate::cpu) fn sub_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let (result, borrow) = a.overflowing_sub(value);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    8
}

/// SBC A, r8 - Subtract 8-bit register value + Carry flag from A
pub(in crate::cpu) fn sbc_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    ctx.cpu.regs.set_carry_flag(borrow1 || borrow2);
    4
}

/// SBC A, imm8 - Subtract immediate 8-bit value + Carry flag from A
pub(in crate::cpu) fn sbc_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    ctx.cpu.regs.set_carry_flag(borrow1 || borrow2);
    8
}

/// SBC A, (HL) - Subtract value at address in HL + Carry flag from A
pub(in crate::cpu) fn sbc_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let carry_in = ctx.cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    ctx.cpu.regs.set_carry_flag(borrow1 || borrow2);
    8
}

/// AND A, r8 - Logical AND 8-bit register value with A
pub(in crate::cpu) fn and_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let result = a & value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(true);
    ctx.cpu.regs.set_carry_flag(false);
    4
}

/// AND A, imm8 - Logical AND immediate 8-bit value with A
pub(in crate::cpu) fn and_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let result = a & value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(true);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// AND A, (HL) - Logical AND value at address in HL with A
pub(in crate::cpu) fn and_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let result = a & value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(true);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// XOR A, r8 - Logical XOR 8-bit register value with A
pub(in crate::cpu) fn xor_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let result = a ^ value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    4
}

/// XOR A, imm8 - Logical XOR immediate 8-bit value with A
pub(in crate::cpu) fn xor_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let result = a ^ value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// XOR A, (HL) - Logical XOR value at address in HL with A
pub(in crate::cpu) fn xor_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let result = a ^ value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// OR A, r8 - Logical OR 8-bit register value with A
pub(in crate::cpu) fn or_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let result = a | value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    4
}

/// OR A, imm8 - Logical OR immediate 8-bit value with A
pub(in crate::cpu) fn or_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let result = a | value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// OR A, (HL) - Logical OR value at address in HL with A
pub(in crate::cpu) fn or_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let result = a | value;
    ctx.cpu.regs.set_a(result);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(false);
    8
}

/// CP A, r8 - Compare 8-bit register value with A
pub(in crate::cpu) fn cp_r8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    4
}

/// CP A, imm8 - Compare immediate 8-bit value with A
pub(in crate::cpu) fn cp_imm8(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let (result, borrow) = a.overflowing_sub(value);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    8
}

/// CP A, (HL) - Compare value at address in HL with A
pub(in crate::cpu) fn cp_mem_hl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    let (result, borrow) = a.overflowing_sub(value);

    ctx.cpu.regs.set_zero_flag(result == 0);
    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    ctx.cpu.regs.set_carry_flag(borrow);
    8
}

/// DAA - Decimal Adjust for Addition (BCD)
pub(in crate::cpu) fn daa(ctx: &mut InstructionContext) -> u8 {
    let mut a = ctx.cpu.regs.a();
    let mut adjust = 0;

    if !ctx.cpu.regs.subtract_flag() {
        if ctx.cpu.regs.half_carry_flag() || (a & 0x0F) > 9 { // fallback cuz quirks
            adjust |= 0x06;
        }
        if ctx.cpu.regs.carry_flag() || a > 0x99 {
            adjust |= 0x60;
            ctx.cpu.regs.set_carry_flag(true);
        }
        a = a.wrapping_add(adjust);
    } else {
        if ctx.cpu.regs.half_carry_flag() {
            adjust |= 0x06;
        }
        if ctx.cpu.regs.carry_flag() {
            adjust |= 0x60;
        }
        a = a.wrapping_sub(adjust);
    }

    ctx.cpu.regs.set_a(a);
    ctx.cpu.regs.set_zero_flag(a == 0);
    ctx.cpu.regs.set_half_carry_flag(false);
    4
}

/// CPL - Complement A (bitwise NOT)
pub(in crate::cpu) fn cpl(ctx: &mut InstructionContext) -> u8 {
    let a = ctx.cpu.regs.a();
    ctx.cpu.regs.set_a(!a);

    ctx.cpu.regs.set_subtract_flag(true);
    ctx.cpu.regs.set_half_carry_flag(true);
    4
}

/// SCF - Set Carry Flag
pub(in crate::cpu) fn scf(ctx: &mut InstructionContext) -> u8 {
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    ctx.cpu.regs.set_carry_flag(true);
    4
}

/// CCF - Complement Carry Flag
pub(in crate::cpu) fn ccf(ctx: &mut InstructionContext) -> u8 {
    let carry = ctx.cpu.regs.carry_flag();
    ctx.cpu.regs.set_carry_flag(!carry);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag(false);
    4
}