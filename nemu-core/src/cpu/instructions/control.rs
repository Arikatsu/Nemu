use crate::cpu::Cpu;
use crate::traits::Bus;

pub(in crate::cpu) enum JumpCond {
    Z,  // Zero flag set
    NZ, // Zero flag not set
    C,  // Carry flag set
    NC, // Carry flag not set
}

impl JumpCond {
    #[inline]
    fn check<B: Bus>(&self, cpu: &Cpu<B>) -> bool {
        match self {
            JumpCond::Z => cpu.regs.zero_flag(),
            JumpCond::NZ => !cpu.regs.zero_flag(),
            JumpCond::C => cpu.regs.carry_flag(),
            JumpCond::NC => !cpu.regs.carry_flag(),
        }
    }
}

/// JR imm8 - Jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_imm8<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc) as i8;
    cpu.inc_pc(1);
    let pc = cpu.pc.wrapping_add(offset as u16);
    cpu.set_pc(pc);
    12
}

/// JR cc, imm8 - Conditional jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_cond_imm8<B: Bus>(cpu: &mut Cpu<B>, cond: JumpCond) -> u8 {
    let offset = cpu.memory.borrow().read(cpu.pc) as i8;
    cpu.inc_pc(1);

    if cond.check(cpu) {
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

/// JP HL - Jump to address in HL
pub(in crate::cpu) fn jp_hl<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let addr = cpu.regs.hl();
    cpu.set_pc(addr);
    4
}

/// JP cc, imm16 - Conditional jump to immediate 16-bit address
pub(in crate::cpu) fn jp_cond_imm16<B: Bus>(cpu: &mut Cpu<B>, cond: JumpCond) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);

    if cond.check(cpu) {
        cpu.set_pc(addr);
        16
    } else {
        12
    }
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

/// CALL cc, imm16 - Conditional call to subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_cond_imm16<B: Bus>(cpu: &mut Cpu<B>, cond: JumpCond) -> u8 {
    let addr = cpu.memory.borrow().read_u16(cpu.pc);
    cpu.inc_pc(2);

    if cond.check(cpu) {
        let ret_addr = cpu.pc;
        let sp = cpu.sp.wrapping_sub(2);
        cpu.memory.borrow_mut().write_u16(sp, ret_addr);
        cpu.set_sp(sp);
        cpu.set_pc(addr);
        24
    } else {
        12
    }
}

/// RET - Return from subroutine
pub(in crate::cpu) fn ret<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let ret_addr = cpu.memory.borrow().read_u16(cpu.sp);
    cpu.inc_sp(2);
    cpu.set_pc(ret_addr);
    16
}

/// RET cc - Conditional return from subroutine
pub(in crate::cpu) fn ret_cond<B: Bus>(cpu: &mut Cpu<B>, cond: JumpCond) -> u8 {
    if cond.check(cpu) {
        let ret_addr = cpu.memory.borrow().read_u16(cpu.sp);
        cpu.inc_sp(2);
        cpu.set_pc(ret_addr);
        20
    } else {
        8
    }
}

/// RETI - Return from interrupt (enable interrupts after return)
pub(in crate::cpu) fn reti<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    let cycles = ret(cpu);
    cpu.ime = true;
    cycles
}

/// RST vec - Call subroutine at fixed address (vector)
pub(in crate::cpu) fn rst<B: Bus>(cpu: &mut Cpu<B>, vec: u8) -> u8 {
    let ret_addr = cpu.pc;
    let sp = cpu.sp.wrapping_sub(2);
    cpu.memory.borrow_mut().write_u16(sp, ret_addr);
    cpu.set_sp(sp);
    cpu.set_pc(vec as u16);
    16
}