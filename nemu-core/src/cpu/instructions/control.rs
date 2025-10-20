use crate::bus::Bus;
use crate::cpu::{Cpu, InterruptMode, Registers};

pub(in crate::cpu) enum JumpCond {
    Z,  // Zero flag set
    NZ, // Zero flag not set
    C,  // Carry flag set
    NC, // Carry flag not set
}

impl JumpCond {
    #[inline]
    fn check(&self, regs: &Registers) -> bool {
        match self {
            JumpCond::Z => regs.zero_flag(),
            JumpCond::NZ => !regs.zero_flag(),
            JumpCond::C => regs.carry_flag(),
            JumpCond::NC => !regs.carry_flag(),
        }
    }
}

/// JR imm8 - Jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_imm8(cpu: &mut Cpu, bus: &mut Bus) {
    let offset = bus.read(cpu.regs.pc()) as i8;
    cpu.regs.inc_pc(1);
    let pc = cpu.regs.pc().wrapping_add(offset as u16);
    cpu.regs.set_pc(pc);
    bus.tick(1);
}

/// JR cc, imm8 - Conditional jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_cond_imm8(cpu: &mut Cpu, bus: &mut Bus, cond: JumpCond) {
    let offset = bus.read(cpu.regs.pc()) as i8;
    cpu.regs.inc_pc(1);

    if cond.check(&cpu.regs) {
        let pc = cpu.regs.pc().wrapping_add(offset as u16);
        cpu.regs.set_pc(pc);
        bus.tick(1);
    }
}

/// JP imm16 - Jump to immediate 16-bit address
pub(in crate::cpu) fn jp_imm16(cpu: &mut Cpu, bus: &mut Bus) {
    let addr = bus.read_u16(cpu.regs.pc());
    cpu.regs.set_pc(addr);
    bus.tick(1);
}

/// JP HL - Jump to address in HL
pub(in crate::cpu) fn jp_hl(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    cpu.regs.set_pc(addr);
}

/// JP cc, imm16 - Conditional jump to immediate 16-bit address
pub(in crate::cpu) fn jp_cond_imm16(cpu: &mut Cpu, bus: &mut Bus, cond: JumpCond) {
    let addr = bus.read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);

    if cond.check(&cpu.regs) {
        cpu.regs.set_pc(addr);
        bus.tick(1);
    }
}

/// CALL imm16 - Call subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_imm16(cpu: &mut Cpu, bus: &mut Bus) {
    let addr = bus.read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);
    let ret_addr = cpu.regs.pc();
    bus.tick(1);
    let sp = cpu.regs.sp().wrapping_sub(2);
    bus.write_u16(sp, ret_addr);
    cpu.regs.set_sp(sp);
    cpu.regs.set_pc(addr);
}

/// CALL cc, imm16 - Conditional call to subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_cond_imm16(cpu: &mut Cpu, bus: &mut Bus, cond: JumpCond) {
    let addr = bus.read_u16(cpu.regs.pc());
    cpu.regs.inc_pc(2);

    if cond.check(&cpu.regs) {
        let ret_addr = cpu.regs.pc();
        bus.tick(1);
        let sp = cpu.regs.sp().wrapping_sub(2);
        bus.write_u16(sp, ret_addr);
        cpu.regs.set_sp(sp);
        cpu.regs.set_pc(addr);
    }
}

/// RET - Return from subroutine
pub(in crate::cpu) fn ret(cpu: &mut Cpu, bus: &mut Bus) {
    let ret_addr = bus.read_u16(cpu.regs.sp());
    cpu.regs.inc_sp(2);
    cpu.regs.set_pc(ret_addr);
    bus.tick(1);
}

/// RET cc - Conditional return from subroutine
pub(in crate::cpu) fn ret_cond(cpu: &mut Cpu, bus: &mut Bus, cond: JumpCond) {
    if cond.check(&cpu.regs) {
        bus.tick(1);
        let ret_addr = bus.read_u16(cpu.regs.sp());
        cpu.regs.inc_sp(2);
        bus.tick(1);
        cpu.regs.set_pc(ret_addr);
    }
}

/// RETI - Return from interrupt (enable interrupts after return)
pub(in crate::cpu) fn reti(cpu: &mut Cpu, bus: &mut Bus) {
    ret(cpu, bus);
    cpu.ime = InterruptMode::Enabled;
}

/// RST vec - Call subroutine at fixed address (vector)
pub(in crate::cpu) fn rst(cpu: &mut Cpu, bus: &mut Bus, vec: u8) {
    let ret_addr = cpu.regs.pc();
    let sp = cpu.regs.sp().wrapping_sub(2);
    bus.tick(1);
    bus.write_u16(sp, ret_addr);
    cpu.regs.set_sp(sp);
    cpu.regs.set_pc(vec as u16);
}