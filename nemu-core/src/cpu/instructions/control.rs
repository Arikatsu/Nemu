use crate::cpu::Cpu;
use crate::traits::Bus;

/// JR imm8 - Jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc) as i8;
    cpu.inc_pc(1);
    let pc = cpu.pc.wrapping_add(offset as u16);
    cpu.set_pc(pc);
    12
}

/// JR NZ, imm8 - Jump relative by immediate 8-bit signed offset if Z flag is not set
pub(in crate::cpu) fn jr_nz_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
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
pub(in crate::cpu) fn jp_imm16<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.set_pc(addr);
    16
}

/// CALL imm16 - Call subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_imm16<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);
    let ret_addr = cpu.pc;
    let sp = cpu.sp.wrapping_sub(2);
    cpu.memory.borrow_mut().write_u16(sp, ret_addr);
    cpu.set_sp(sp);
    cpu.set_pc(addr);
    24
}

/// RET - Return from subroutine
pub(in crate::cpu) fn ret<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let ret_addr = cpu.memory.borrow().read_u16(cpu.sp);
    cpu.inc_sp(2);
    cpu.set_pc(ret_addr);
    16
}