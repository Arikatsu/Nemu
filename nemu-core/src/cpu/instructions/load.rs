use crate::context::NemuContext;
use crate::cpu::Cpu;
use crate::cpu::registers::{Reg16, Reg8};

/// LD r8, r8 - Load 8-bit register value into another 8-bit register
pub(in crate::cpu) fn ld_r8_r8(cpu: &mut Cpu, dest: Reg8, src: Reg8) {
    let value = cpu.regs.read_reg8(src);
    cpu.regs.write_reg8(dest, value);
}

/// LD r8, imm8 - Load immediate 8-bit value into 8-bit register
pub(in crate::cpu) fn ld_r8_imm8(cpu: &mut Cpu, ctx: &mut NemuContext, reg: Reg8)  {
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    cpu.regs.write_reg8(reg, value);
}

/// LD r8, (r16) - Load 8-bit value from memory address in 16-bit register into 8-bit register
pub(in crate::cpu) fn ld_r8_mem_r16(cpu: &mut Cpu, ctx: &mut NemuContext, dest: Reg8, addr_reg: Reg16) {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = ctx.mem_read(addr);
    cpu.regs.write_reg8(dest, value);
}

/// LD r16, imm16 - Load immediate 16-bit value into 16-bit register
pub(in crate::cpu) fn ld_r16_imm16(cpu: &mut Cpu, ctx: &mut NemuContext, reg: Reg16) {
    let value = ctx.mem_read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    cpu.regs.write_reg16(reg, value);
}

/// LD (r16), r8 - Store 8-bit register value at memory address in 16-bit register
pub(in crate::cpu) fn ld_mem_r16_r8(cpu: &mut Cpu, ctx: &mut NemuContext, addr_reg: Reg16, src: Reg8) {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.regs.read_reg8(src);
    ctx.mem_write(addr, value);
}

/// LD (r16), imm8 - Store immediate 8-bit value at memory address in 16-bit register
pub(in crate::cpu) fn ld_mem_r16_imm8(cpu: &mut Cpu, ctx: &mut NemuContext, addr_reg: Reg16) {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    ctx.mem_write(addr, value);
}

/// LD (HL+), A - Store A at address in HL, then increment HL
pub(in crate::cpu) fn ld_mem_hli_a(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a();
    ctx.mem_write(addr, a);
    cpu.regs.set_hl(addr.wrapping_add(1));
}

/// LD (HL-), A - Store A at address in HL, then decrement HL
pub(in crate::cpu) fn ld_mem_hld_a(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a();
    ctx.mem_write(addr, a);
    cpu.regs.set_hl(addr.wrapping_sub(1));
}

/// LD A, (HL+) - Load A from address in HL, then increment HL
pub(in crate::cpu) fn ld_a_mem_hli(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_add(1));
}

/// LD A, (HL-) - Load A from address in HL, then decrement HL
pub(in crate::cpu) fn ld_a_mem_hld(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = cpu.regs.hl();
    let value = ctx.mem_read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_sub(1));
}

/// LD (imm16), SP - Store SP at immediate 16-bit address
pub(in crate::cpu) fn ld_mem_imm16_sp(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = ctx.mem_read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    ctx.mem_write_u16(addr, cpu.regs.sp());
}

/// LD SP. imm16 - Load immediate 16-bit value into SP
pub(in crate::cpu) fn ld_sp_imm16(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let value = ctx.mem_read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    cpu.regs.set_sp(value);
}

/// LD HL, SP+imm8 - Load SP plus immediate 8-bit signed value into HL
pub(in crate::cpu) fn ld_hl_sp_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let offset = ctx.mem_read(cpu.regs.pc()) as i8;
    cpu.regs.inc_pc(1);
    ctx.tick(1);
    let sp = cpu.regs.sp();
    let result = sp.wrapping_add(offset as u16);
    cpu.regs.set_hl(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag((sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);
}

/// LD SP, HL - Load HL into SP
pub(in crate::cpu) fn ld_sp_hl(cpu: &mut Cpu, ctx: &mut NemuContext) {
    ctx.tick(1);
    let hl = cpu.regs.hl();
    cpu.regs.set_sp(hl);
}

/// LDH (imm8), A - Store A at address 0xFF00 + immediate 8-bit value
pub(in crate::cpu) fn ldh_mem_imm8_a(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let offset = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = cpu.regs.a();
    ctx.mem_write(addr, a);
}

/// LDH A, (imm8) - Load A from address 0xFF00 + immediate 8-bit value
pub(in crate::cpu) fn ldh_a_mem_imm8(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let offset = ctx.mem_read(cpu.regs.pc());
    cpu.regs.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = ctx.mem_read(addr);
    cpu.regs.set_a(value);
}

/// LDH (C), A - Store A at address 0xFF00 + C
pub(in crate::cpu) fn ldh_mem_c_a(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let offset = cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = cpu.regs.a();
    ctx.mem_write(addr, a);
}

/// LDH A, (C) - Load A from address 0xFF00 + C
pub(in crate::cpu) fn ldh_a_mem_c(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let offset = cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = ctx.mem_read(addr);
    cpu.regs.set_a(value);
}

/// LD (imm16), A - Store A at immediate 16-bit address
pub(in crate::cpu) fn ld_mem_imm16_a(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = ctx.mem_read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    let a = cpu.regs.a();
    ctx.mem_write(addr, a);
}

/// LD A, (imm16) - Load A from immediate 16-bit address
pub(in crate::cpu) fn ld_a_mem_imm16(cpu: &mut Cpu, ctx: &mut NemuContext) {
    let addr = ctx.mem_read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    let value = ctx.mem_read(addr);
    cpu.regs.set_a(value);
}