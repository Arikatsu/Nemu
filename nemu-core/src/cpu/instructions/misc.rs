use crate::cpu::{Cpu, InterruptMode};

/// STOP - Enter low power mode (halts CPU until an interrupt occurs)
pub(in crate::cpu) fn stop(cpu: &mut Cpu) -> u8 {
    // stub implementation until i add it properly
    cpu.regs.inc_pc(1);
    4
}

/// HALT - Halt CPU until an interrupt occurs
pub(in crate::cpu) fn halt(cpu: &mut Cpu) -> u8 {
    cpu.halted = true;
    4
}

/// DI - Disable interrupts
pub(in crate::cpu) fn di(cpu: &mut Cpu) -> u8 {
    cpu.ime = InterruptMode::Disabled;
    4
}

/// EI - Enable interrupts (actually delayed until next instruction, so set to Pending)
pub(in crate::cpu) fn ei(cpu: &mut Cpu) -> u8 {
    cpu.ime = InterruptMode::Pending;
    4
}