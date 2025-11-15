use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::registers::Reg16;

/// POP r16 - Pop 16-bit value from stack into 16-bit register
pub(in crate::cpu) fn pop_r16(cpu: &mut Cpu, bus: &mut Bus, reg: Reg16) -> u8 {
    let value = bus.read_u16(cpu.regs.sp());
    cpu.regs.write_reg16(reg, value);
    cpu.regs.inc_sp(2);
    12
}

/// PUSH r16 - Push 16-bit register value onto stack
pub(in crate::cpu) fn push_r16(cpu: &mut Cpu, bus: &mut Bus, reg: Reg16) -> u8 {
    let sp = cpu.regs.sp().wrapping_sub(2);
    bus.tick(1);
    let value = cpu.regs.read_reg16(reg);
    bus.write_u16(sp, value);
    cpu.regs.set_sp(sp);
    16
}