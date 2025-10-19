use crate::context::NemuContext;
use crate::cpu::Cpu;
use crate::cpu::registers::{Reg8, Reg16};

/// INC r8 - Increment 8-bit register
pub(in crate::cpu) fn inc_r8(cpu: &mut Cpu, reg: Reg8) {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
}

/// INC r16 - Increment 16-bit register
pub(in crate::cpu) fn inc_r16(cpu: &mut Cpu, ctx: &mut NemuContext, reg: Reg16) {
    let value = cpu.regs.read_reg16(reg);
    ctx.tick(1);
    cpu.regs.write_reg16(reg, value.wrapping_add(1));
}

/// INC SP - Increment Stack Pointer
pub(in crate::cpu) fn inc_sp(cpu: &mut Cpu, ctx: &mut NemuContext) {
    ctx.tick(1);
    cpu.regs.inc_sp(1);
}

/// INC (HL) - Increment value at address in HL
pub(in crate::cpu) fn inc_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let result = value.wrapping_add(1);
    ctx.mem_write(addr, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
}

/// DEC r8 - Decrement 8-bit register
pub(in crate::cpu) fn dec_r8(cpu: &mut Cpu, reg: Reg8) {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
}

/// DEC r16 - Decrement 16-bit register
pub(in crate::cpu) fn dec_r16(cpu: &mut Cpu, ctx: &mut NemuContext, reg: Reg16) {
    let value = cpu.regs.read_reg16(reg);
    ctx.tick(1);
    cpu.regs.write_reg16(reg, value.wrapping_sub(1));
}

/// DEC SP - Decrement Stack Pointer
pub(in crate::cpu) fn dec_sp(cpu: &mut Cpu, ctx: &mut NemuContext) {
    ctx.tick(1);
    cpu.regs.dec_sp(1);
}

/// DEC (HL) - Decrement value at address in HL
pub(in crate::cpu) fn dec_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let result = value.wrapping_sub(1);
    ctx.mem_write(addr, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
}

/// ADD A, r8 - Add 8-bit register value to A
pub(in crate::cpu) fn add_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, carry) = a.overflowing_add(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag(carry);
}

/// ADD HL, r16 - Add 16-bit register value to HL
pub(in crate::cpu) fn add_hl_r16(cpu: &mut Cpu, ctx: &mut NemuContext, reg: Reg16) {
    let hl = cpu.regs.hl();
    let value = cpu.regs.read_reg16(reg);
    let (result, carry) = hl.overflowing_add(value);
    ctx.tick(1);
    cpu.regs.set_hl(result);

    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
    cpu.regs.set_carry_flag(carry);
}

/// ADD A, imm8 - Add immediate 8-bit value to A
pub(in crate::cpu) fn add_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let (result, carry) = a.overflowing_add(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag(carry);
}

/// ADD A, (HL) - Add value at address in HL to A
pub(in crate::cpu) fn add_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let (result, carry) = a.overflowing_add(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag(carry);
}

/// ADD HL, SP - Add Stack Pointer to HL
pub(in crate::cpu) fn add_hl_sp(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let hl = cpu.regs.hl();
    let sp = cpu.regs.sp();
    let (result, carry) = hl.overflowing_add(sp);
    ctx.tick(1);
    cpu.regs.set_hl(result);

    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((hl & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF);
    cpu.regs.set_carry_flag(carry);
}

/// ADD SP, imm8 - Add immediate 8-bit signed value to Stack Pointer
pub(in crate::cpu) fn add_sp_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let sp = cpu.regs.sp();
    let offset = ctx.mem_read(cpu.regs.pc()) as i8;
    cpu.regs.inc_pc(1);
    let result = sp.wrapping_add(offset as u16);
    ctx.tick(2);
    cpu.regs.set_sp(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F);
    cpu.regs
        .set_carry_flag((sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);
}

/// ADC A, r8 - Add 8-bit register value + Carry flag to A
pub(in crate::cpu) fn adc_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    cpu.regs.set_carry_flag(carry1 || carry2);
}

/// ADC A, imm8 - Add immediate 8-bit value + Carry flag to A
pub(in crate::cpu) fn adc_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    cpu.regs.set_carry_flag(carry1 || carry2);
}

/// ADC A, (HL) - Add value at address in HL + Carry flag to A
pub(in crate::cpu) fn adc_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, carry1) = a.overflowing_add(value);
    let (result, carry2) = intermediate.overflowing_add(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry_in > 0x0F);
    cpu.regs.set_carry_flag(carry1 || carry2);
}

/// SUB r8 - Subtract 8-bit register value from A
pub(in crate::cpu) fn sub_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// SUB imm8 - Subtract immediate 8-bit value from A
pub(in crate::cpu) fn sub_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let (result, borrow) = a.overflowing_sub(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// SUB (HL) - Subtract value at address in HL from A
pub(in crate::cpu) fn sub_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let (result, borrow) = a.overflowing_sub(value);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// SBC A, r8 - Subtract 8-bit register value + Carry flag from A
pub(in crate::cpu) fn sbc_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    cpu.regs.set_carry_flag(borrow1 || borrow2);
}

/// SBC A, imm8 - Subtract immediate 8-bit value + Carry flag from A
pub(in crate::cpu) fn sbc_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    cpu.regs.set_carry_flag(borrow1 || borrow2);
}

/// SBC A, (HL) - Subtract value at address in HL + Carry flag from A
pub(in crate::cpu) fn sbc_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let carry_in = cpu.regs.carry_flag() as u8;
    let (intermediate, borrow1) = a.overflowing_sub(value);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs
        .set_half_carry_flag((a & 0x0F) < (value & 0x0F) + carry_in);
    cpu.regs.set_carry_flag(borrow1 || borrow2);
}

/// AND A, r8 - Logical AND 8-bit register value with A
pub(in crate::cpu) fn and_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a & value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);
}

/// AND A, imm8 - Logical AND immediate 8-bit value with A
pub(in crate::cpu) fn and_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let result = a & value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);
}

/// AND A, (HL) - Logical AND value at address in HL with A
pub(in crate::cpu) fn and_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let result = a & value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);
}

/// XOR A, r8 - Logical XOR 8-bit register value with A
pub(in crate::cpu) fn xor_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a ^ value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// XOR A, imm8 - Logical XOR immediate 8-bit value with A
pub(in crate::cpu) fn xor_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let result = a ^ value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// XOR A, (HL) - Logical XOR value at address in HL with A
pub(in crate::cpu) fn xor_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let result = a ^ value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// OR A, r8 - Logical OR 8-bit register value with A
pub(in crate::cpu) fn or_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let result = a | value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// OR A, imm8 - Logical OR immediate 8-bit value with A
pub(in crate::cpu) fn or_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let result = a | value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// OR A, (HL) - Logical OR value at address in HL with A
pub(in crate::cpu) fn or_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let result = a | value;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
}

/// CP A, r8 - Compare 8-bit register value with A
pub(in crate::cpu) fn cp_r8(cpu: &mut Cpu, reg: Reg8) {
    let a = cpu.regs.a();
    let value = cpu.regs.read_reg8(reg);
    let (result, borrow) = a.overflowing_sub(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// CP A, imm8 - Compare immediate 8-bit value with A
pub(in crate::cpu) fn cp_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let (result, borrow) = a.overflowing_sub(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// CP A, (HL) - Compare value at address in HL with A
pub(in crate::cpu) fn cp_mem_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let a = cpu.regs.a();
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    let (result, borrow) = a.overflowing_sub(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
    cpu.regs.set_carry_flag(borrow);
}

/// DAA - Decimal Adjust for Addition (BCD)
pub(in crate::cpu) fn daa(cpu: &mut Cpu) {
    let mut a = cpu.regs.a();
    let mut adjust = 0;

    if !cpu.regs.subtract_flag() {
        // fallback cuz quirks
        if cpu.regs.half_carry_flag() || (a & 0x0F) > 9 {
            adjust |= 0x06;
        }
        if cpu.regs.carry_flag() || a > 0x99 {
            adjust |= 0x60;
            cpu.regs.set_carry_flag(true);
        }
        a = a.wrapping_add(adjust);
    } else {
        if cpu.regs.half_carry_flag() {
            adjust |= 0x06;
        }
        if cpu.regs.carry_flag() {
            adjust |= 0x60;
        }
        a = a.wrapping_sub(adjust);
    }

    cpu.regs.set_a(a);
    cpu.regs.set_zero_flag(a == 0);
    cpu.regs.set_half_carry_flag(false);
}

/// CPL - Complement A (bitwise NOT)
pub(in crate::cpu) fn cpl(cpu: &mut Cpu) {
    let a = cpu.regs.a();
    cpu.regs.set_a(!a);

    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag(true);
}

/// SCF - Set Carry Flag
pub(in crate::cpu) fn scf(cpu: &mut Cpu) {
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(true);
}

/// CCF - Complement Carry Flag
pub(in crate::cpu) fn ccf(cpu: &mut Cpu) {
    let carry = cpu.regs.carry_flag();
    cpu.regs.set_carry_flag(!carry);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
}
