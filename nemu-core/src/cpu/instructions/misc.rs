use crate::cpu::Cpu;
use crate::traits::Bus;

/// DI - Disable interrupts
pub(in crate::cpu) fn di<B: Bus>(cpu: &mut Cpu<B>) -> u8 {
    cpu.ime = false;
    4
}