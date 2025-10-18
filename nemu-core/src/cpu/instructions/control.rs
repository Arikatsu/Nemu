use super::InstructionContext;
use crate::cpu::{InterruptMode, Registers};

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
pub(in crate::cpu) fn jr_imm8(ctx: &mut InstructionContext) -> u8 {
    let offset = ctx.memory.read(ctx.cpu.pc) as i8;
    ctx.cpu.inc_pc(1);
    let pc = ctx.cpu.pc.wrapping_add(offset as u16);
    ctx.cpu.set_pc(pc);
    12
}

/// JR cc, imm8 - Conditional jump relative by immediate 8-bit signed offset
pub(in crate::cpu) fn jr_cond_imm8(ctx: &mut InstructionContext, cond: JumpCond) -> u8 {
    let offset = ctx.memory.read(ctx.cpu.pc) as i8;
    ctx.cpu.inc_pc(1);

    if cond.check(&ctx.cpu.regs) {
        let pc = ctx.cpu.pc.wrapping_add(offset as u16);
        ctx.cpu.set_pc(pc);
        12
    } else {
        8
    }
}

/// JP imm16 - Jump to immediate 16-bit address
pub(in crate::cpu) fn jp_imm16(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.pc);
    ctx.cpu.set_pc(addr);
    16
}

/// JP HL - Jump to address in HL
pub(in crate::cpu) fn jp_hl(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.cpu.regs.hl();
    ctx.cpu.set_pc(addr);
    4
}

/// JP cc, imm16 - Conditional jump to immediate 16-bit address
pub(in crate::cpu) fn jp_cond_imm16(ctx: &mut InstructionContext, cond: JumpCond) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.pc);
    ctx.cpu.inc_pc(2);

    if cond.check(&ctx.cpu.regs) {
        ctx.cpu.set_pc(addr);
        16
    } else {
        12
    }
}

/// CALL imm16 - Call subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_imm16(ctx: &mut InstructionContext) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.pc);
    ctx.cpu.inc_pc(2);
    let ret_addr = ctx.cpu.pc;
    let sp = ctx.cpu.sp.wrapping_sub(2);
    ctx.memory.write_u16(sp, ret_addr);
    ctx.cpu.set_sp(sp);
    ctx.cpu.set_pc(addr);
    24
}

/// CALL cc, imm16 - Conditional call to subroutine at immediate 16-bit address
pub(in crate::cpu) fn call_cond_imm16(ctx: &mut InstructionContext, cond: JumpCond) -> u8 {
    let addr = ctx.memory.read_u16(ctx.cpu.pc);
    ctx.cpu.inc_pc(2);

    if cond.check(&ctx.cpu.regs) {
        let ret_addr = ctx.cpu.pc;
        let sp = ctx.cpu.sp.wrapping_sub(2);
        ctx.memory.write_u16(sp, ret_addr);
        ctx.cpu.set_sp(sp);
        ctx.cpu.set_pc(addr);
        24
    } else {
        12
    }
}

/// RET - Return from subroutine
pub(in crate::cpu) fn ret(ctx: &mut InstructionContext) -> u8 {
    let ret_addr = ctx.memory.read_u16(ctx.cpu.sp);
    ctx.cpu.inc_sp(2);
    ctx.cpu.set_pc(ret_addr);
    16
}

/// RET cc - Conditional return from subroutine
pub(in crate::cpu) fn ret_cond(ctx: &mut InstructionContext, cond: JumpCond) -> u8 {
    if cond.check(&ctx.cpu.regs) {
        let ret_addr = ctx.memory.read_u16(ctx.cpu.sp);
        ctx.cpu.inc_sp(2);
        ctx.cpu.set_pc(ret_addr);
        20
    } else {
        8
    }
}

/// RETI - Return from interrupt (enable interrupts after return)
pub(in crate::cpu) fn reti(ctx: &mut InstructionContext) -> u8 {
    let cycles = ret(ctx);
    ctx.cpu.ime = InterruptMode::Enabled;
    cycles
}

/// RST vec - Call subroutine at fixed address (vector)
pub(in crate::cpu) fn rst(ctx: &mut InstructionContext, vec: u8) -> u8 {
    let ret_addr = ctx.cpu.pc;
    let sp = ctx.cpu.sp.wrapping_sub(2);
    ctx.memory.write_u16(sp, ret_addr);
    ctx.cpu.set_sp(sp);
    ctx.cpu.set_pc(vec as u16);
    16
}