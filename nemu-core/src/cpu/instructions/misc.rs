use crate::cpu::{Cpu, InterruptMode};
use crate::traits::Bus;

/// STOP - Enter low power mode (halts CPU until an interrupt occurs)
pub(in crate::cpu) fn stop<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    // stub implementation until i add it properly
    cpu.inc_pc(1);
    4
}

/// HALT - Halt CPU until an interrupt occurs
pub(in crate::cpu) fn halt<B: Bus>(_cpu: &mut Cpu<B>) -> u8 {
    // again, stub implementation for now
    4
}

/// DI - Disable interrupts
pub(in crate::cpu) fn di<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.ime = InterruptMode::Disabled;
    4
}

/// EI - Enable interrupts (actually delayed until next instruction, so set to Pending)
pub(in crate::cpu) fn ei<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.ime = InterruptMode::Pending;
    4
}