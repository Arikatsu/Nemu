use super::InstructionContext;
use crate::cpu::registers::{Reg16, Reg8};

/// LD r8, r8 - Load 8-bit register value into another 8-bit register
pub(in crate::cpu) fn ld_r8_r8(ctx: &mut InstructionContext, dest: Reg8, src: Reg8) -> u8 {
    let value = ctx.cpu.regs.read_reg8(src);
    ctx.cpu.regs.write_reg8(dest, value);
    4
}

/// LD r8, imm8 - Load immediate 8-bit value into 8-bit register
pub(in crate::cpu) fn ld_r8_imm8(ctx: &mut InstructionContext, reg: Reg8) -> u8 {
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    ctx.cpu.regs.write_reg8(reg, value);
    8
}

/// LD r8, (r16) - Load 8-bit value from memory address in 16-bit register into 8-bit register
pub(in crate::cpu) fn ld_r8_mem_r16(ctx: &mut InstructionContext, dest: Reg8, addr_reg: Reg16) -> u8 {
    let addr = ctx.cpu.regs.read_reg16(addr_reg);
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.write_reg8(dest, value);
    8
}

/// LD r16, imm16 - Load immediate 16-bit value into 16-bit register
pub(in crate::cpu) fn ld_r16_imm16(ctx: &mut InstructionContext, reg: Reg16) -> u8 {
    let value = ctx.memory.read_u16(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(2);
    ctx.cpu.regs.write_reg16(reg, value);
    12
}

/// LD (r16), r8 - Store 8-bit register value at memory address in 16-bit register
pub(in crate::cpu) fn ld_mem_r16_r8(ctx: &mut InstructionContext, addr_reg: Reg16, src: Reg8) -> u8 {
    let addr = ctx.cpu.regs.read_reg16(addr_reg);
    let value = ctx.cpu.regs.read_reg8(src);
    ctx.memory.write(addr, value);
    8
}

/// LD (r16), imm8 - Store immediate 8-bit value at memory address in 16-bit register
pub(in crate::cpu) fn ld_mem_r16_imm8(ctx: &mut InstructionContext, addr_reg: Reg16) -> u8 {
    let addr = ctx.cpu.regs.read_reg16(addr_reg);
    let value = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    ctx.memory.write(addr, value);
    12
}

/// LD (HL+), A - Store A at address in HL, then increment HL
pub(in crate::cpu) fn ld_mem_hli_a(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let a = ctx.cpu.regs.a();
    ctx.memory.write(addr, a);
    ctx.cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD (HL-), A - Store A at address in HL, then decrement HL
pub(in crate::cpu) fn ld_mem_hld_a(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let a = ctx.cpu.regs.a();
    ctx.memory.write(addr, a);
    ctx.cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD A, (HL+) - Load A from address in HL, then increment HL
pub(in crate::cpu) fn ld_a_mem_hli(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.set_a(value);
    ctx.cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD A, (HL-) - Load A from address in HL, then decrement HL
pub(in crate::cpu) fn ld_a_mem_hld(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.set_a(value);
    ctx.cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD (imm16), SP - Store SP at immediate 16-bit address
pub(in crate::cpu) fn ld_mem_imm16_sp(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(2);
    ctx.memory.write_u16(addr, ctx.cpu.regs.sp());
    20
}

/// LD SP. imm16 - Load immediate 16-bit value into SP
pub(in crate::cpu) fn ld_sp_imm16(ctx: &mut InstructionContext) -> u8 {
    let value = ctx.memory.read_u16(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(2);
    ctx.cpu.regs.set_sp(value);
    12
}

/// LD HL, SP+imm8 - Load SP plus immediate 8-bit signed value into HL
pub(in crate::cpu) fn ld_hl_sp_imm8(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.memory.read(ctx.cpu.regs.pc()) as i8;
    ctx.cpu.regs.inc_pc(1);
    let sp = ctx.cpu.regs.sp();
    let result = sp.wrapping_add(offset as u16);
    ctx.cpu.regs.set_hl(result);

    ctx.cpu.regs.set_zero_flag(false);
    ctx.cpu.regs.set_subtract_flag(false);
    ctx.cpu.regs.set_half_carry_flag((sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F);
    ctx.cpu.regs.set_carry_flag((sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);
    12
}

/// LD SP, HL - Load HL into SP
pub(in crate::cpu) fn ld_sp_hl(ctx: &mut InstructionContext) -> u8 {
    let hl = ctx.cpu.regs.hl();
    ctx.cpu.regs.set_sp(hl);
    8
}

/// LDH (imm8), A - Store A at address 0xFF00 + immediate 8-bit value
pub(in crate::cpu) fn ldh_mem_imm8_a(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = ctx.cpu.regs.a();
    ctx.memory.write(addr, a);
    12
}

/// LDH A, (imm8) - Load A from address 0xFF00 + immediate 8-bit value
pub(in crate::cpu) fn ldh_a_mem_imm8(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.memory.read(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.set_a(value);
    12
}

/// LDH (C), A - Store A at address 0xFF00 + C
pub(in crate::cpu) fn ldh_mem_c_a(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = ctx.cpu.regs.a();
    ctx.memory.write(addr, a);
    8
}

/// LDH A, (C) - Load A from address 0xFF00 + C
pub(in crate::cpu) fn ldh_a_mem_c(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.set_a(value);
    8
}

/// LD (imm16), A - Store A at immediate 16-bit address
pub(in crate::cpu) fn ld_mem_imm16_a(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(2);
    let a = ctx.cpu.regs.a();
    ctx.memory.write(addr, a);
    16
}

/// LD A, (imm16) - Load A from immediate 16-bit address
pub(in crate::cpu) fn ld_a_mem_imm16(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.regs.pc());
    ctx.cpu.regs.inc_pc(2);
    let value = ctx.memory.read(addr);
    ctx.cpu.regs.set_a(value);
    16
}