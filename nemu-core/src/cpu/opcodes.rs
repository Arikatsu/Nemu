use super::Cpu;
use super::registers::{Reg8, Reg16};
use crate::traits::Bus;

/// LD r8, r8 - Load 8-bit register value into another 8-bit register
pub(super) fn ld_r8_r8<B: Bus>(cpu: &mut Cpu<B>, dest: Reg8, src: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(src);
    cpu.regs.write_reg8(dest, value);
    4
}

/// LD r8, imm8 - Load immediate 8-bit value into 8-bit register
pub(super) fn ld_r8_imm8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let value = cpu.memory.borrow().read(cpu.pc);
    cpu.inc_pc(1);
    cpu.regs.write_reg8(reg, value);
    8
}

/// LD r8, (r16) - Load 8-bit value from memory address in 16-bit register into 8-bit register
pub(super) fn ld_r8_mem_r16<B: Bus>(cpu: &mut Cpu<B>, dest: Reg8, addr_reg: Reg16) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.memory.borrow().read(addr);
    cpu.regs.write_reg8(dest, value);
    8
}

/// LD r16, imm16 - Load immediate 16-bit value into 16-bit register
pub(super) fn ld_r16_imm16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    cpu.regs.write_reg16(reg, value);
    12
}

/// LD (r16), r8 - Store 8-bit register value at memory address in 16-bit register
pub(super) fn ld_mem_r16_r8<B: Bus>(cpu: &mut Cpu<B>, addr_reg: Reg16, src: Reg8) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.regs.read_reg8(src);
    cpu.memory.borrow_mut().write(addr, value);
    8
}

/// LD (r16), imm8 - Store immediate 8-bit value at memory address in 16-bit register
pub(super) fn ld_mem_r16_imm8<B: Bus>(cpu: &mut Cpu<B>, addr_reg: Reg16) -> u8 {
    let addr = cpu.regs.read_reg16(addr_reg);
    let value = cpu.memory.borrow().read(cpu.pc);
    cpu.inc_pc(1);
    cpu.memory.borrow_mut().write(addr, value);
    12
}

/// LD (HL+), A - Store A at address in HL, then increment HL
pub(super) fn ld_mem_hli_a<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a();
    cpu.memory.borrow_mut().write(addr, a);
    cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD (HL-), A - Store A at address in HL, then decrement HL
pub(super) fn ld_mem_hld_a<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a();
    cpu.memory.borrow_mut().write(addr, a);
    cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD A, (HL+) - Load A from address in HL, then increment HL
pub(super) fn ld_a_mem_hli<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_add(1));
    8
}

/// LD A, (HL-) - Load A from address in HL, then decrement HL
pub(super) fn ld_a_mem_hld<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.regs.hl();
    let value = cpu.memory.borrow().read(addr);
    cpu.regs.set_a(value);
    cpu.regs.set_hl(addr.wrapping_sub(1));
    8
}

/// LD (imm16), SP - Store SP at immediate 16-bit address
pub(super) fn ld_mem_imm16_sp<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    cpu.memory.borrow_mut().write_u16(addr, cpu.sp);
    20
}

/// LD SP. imm16 - Load immediate 16-bit value into SP
pub(super) fn ld_sp_imm16<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let value = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    cpu.set_sp(value);
    12
}

/// LD HL, SP+imm8 - Load SP plus immediate 8-bit signed value into HL
pub(super) fn ld_hl_sp_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc) as i8;
    cpu.inc_pc(1);
    let sp = cpu.sp;
    let result = sp.wrapping_add(offset as u16);
    cpu.regs.set_hl(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F);
    cpu.regs.set_carry_flag((sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);
    12
}

/// LD SP, HL - Load HL into SP
pub(super) fn ld_sp_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let hl = cpu.regs.hl();
    cpu.set_sp(hl);
    8
}

/// LDH (imm8), A - Store A at address 0xFF00 + immediate 8-bit value
pub(super) fn ldh_mem_imm8_a<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc);
    cpu.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = cpu.regs.a();
    cpu.memory.borrow_mut().write(addr, a);
    12
}

/// LDH A, (imm8) - Load A from address 0xFF00 + immediate 8-bit value
pub(super) fn ldh_a_mem_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc);
    cpu.inc_pc(1);
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = cpu.memory.borrow().read(addr);
    cpu.regs.set_a(value);
    12
}

/// LDH (C), A - Store A at address 0xFF00 + C
pub(super) fn ldh_mem_c_a<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let a = cpu.regs.a();
    cpu.memory.borrow_mut().write(addr, a);
    8
}

/// LDH A, (C) - Load A from address 0xFF00 + C
pub(super) fn ldh_a_mem_c<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.regs.c();
    let addr = 0xFF00u16.wrapping_add(offset as u16);
    let value = cpu.memory.borrow().read(addr);
    cpu.regs.set_a(value);
    8
}

/// LD (imm16), A - Store A at immediate 16-bit address
pub(super) fn ld_mem_imm16_a<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    let a = cpu.regs.a();
    cpu.memory.borrow_mut().write(addr, a);
    16
}

/// INC r16 - Increment 16-bit register
pub(super) fn inc_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.regs.read_reg16(reg);
    cpu.regs.write_reg16(reg, value.wrapping_add(1));
    8
}

/// INC r8 - Increment 8-bit register
pub(super) fn inc_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_add(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag((value & 0x0F) + 1 > 0x0F);
    4
}

/// DEC r8 - Decrement 8-bit register
pub(super) fn dec_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = value.wrapping_sub(1);

    cpu.regs.write_reg8(reg, result);
    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag((value & 0x0F) == 0);
    4
}

/// DEC r16 - Decrement 16-bit register
pub(super) fn dec_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.regs.read_reg16(reg);
    cpu.regs.write_reg16(reg, value.wrapping_sub(1));
    8
}

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(super) fn rlca<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
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

/// ADD A, r8 - Add 8-bit register value to A
pub(super) fn add_a_r8<B: Bus>(cpu: &mut Cpu<B>, reg: Reg8) -> u8 {
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
pub(super) fn add_a_mem_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
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

/// POP r16 - Pop 16-bit value from stack into 16-bit register
pub(super) fn pop_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let value = cpu.memory.borrow().read_u16(cpu.sp);
    cpu.regs.write_reg16(reg, value);
    cpu.inc_sp(2);
    12
}

/// PUSH r16 - Push 16-bit register value onto stack
pub(super) fn push_r16<B: Bus>(cpu: &mut Cpu<B>, reg: Reg16) -> u8 {
    let sp = cpu.sp.wrapping_sub(2);
    let value = cpu.regs.read_reg16(reg);
    cpu.memory.borrow_mut().write_u16(sp, value);
    cpu.set_sp(sp);
    16
}

/// JR NZ, imm8 - Jump relative by immediate 8-bit signed offset if Z flag is not set
pub(super) fn jr_nz_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc) as i8;
    cpu.inc_pc(1);
    if !cpu.regs.zero_flag() {
        let pc = cpu.pc.wrapping_add(offset as u16);
        cpu.set_pc(pc);
        12
    } else {
        8
    }
}

/// JP imm16 - Jump to immediate 16-bit address
pub(super) fn jp_imm16<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.set_pc(addr);
    16
}

/// CALL imm16 - Call subroutine at immediate 16-bit address
pub(super) fn call_imm16<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    let ret_addr = cpu.pc;
    let sp = cpu.sp.wrapping_sub(2);
    cpu.memory.borrow_mut().write_u16(sp, ret_addr);
    cpu.set_sp(sp);
    cpu.set_pc(addr);
    24
}

/// DI - Disable interrupts
pub(super) fn di<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.ime = false;
    4
}