use crate::cpu::{Cpu, InterruptMode};

/// STOP - Enter low power mode (halts CPU until an interrupt occurs)
pub(in crate::cpu) fn stop(cpu: &mut Cpu) {
    // stub implementation until i add it properly
    cpu.regs.inc_pc(1);
}

/// HALT - Halt CPU until an interrupt occurs
pub(in crate::cpu) fn halt(cpu: &mut Cpu) {
    cpu.halted = true;
}

/// DI - Disable interrupts
pub(in crate::cpu) fn di(cpu: &mut Cpu) {
    cpu.ime = InterruptMode::Disabled;
}

/// EI - Enable interrupts (actually delayed until next instruction, so set to Pending)
pub(in crate::cpu) fn ei(cpu: &mut Cpu) {
    cpu.ime = InterruptMode::Pending;
}