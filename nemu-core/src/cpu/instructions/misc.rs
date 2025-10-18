use super::InstructionContext;
use crate::cpu::InterruptMode;

/// STOP - Enter low power mode (halts CPU until an interrupt occurs)
pub(in crate::cpu) fn stop(ctx: &mut InstructionContext) -> u8 {
    // stub implementation until i add it properly
    ctx.cpu.inc_pc(1);
    4
}

/// HALT - Halt CPU until an interrupt occurs
pub(in crate::cpu) fn halt(_ctx: &mut InstructionContext) -> u8 {
    // again, stub implementation for now
    4
}

/// DI - Disable interrupts
pub(in crate::cpu) fn di(ctx: &mut InstructionContext) -> u8 {
    ctx.cpu.ime = InterruptMode::Disabled;
    4
}

/// EI - Enable interrupts (actually delayed until next instruction, so set to Pending)
pub(in crate::cpu) fn ei(ctx: &mut InstructionContext) -> u8 {
    ctx.cpu.ime = InterruptMode::Pending;
    4
}