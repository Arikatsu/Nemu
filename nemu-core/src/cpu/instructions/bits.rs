use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::registers::Reg8;

/// RLCA - Rotate A left, old bit 7 to Carry flag
pub(in crate::cpu) fn rlca(cpu: &mut Cpu) -> u8 {
    let a = cpu.regs.a();
    let carry = (a & 0x80) != 0;
    let result = (a << 1) | carry as u8;
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    4
}

/// RRCA - Rotate A right, old bit 0 to Carry flag
pub(in crate::cpu) fn rrca(cpu: &mut Cpu) -> u8 {
    let a = cpu.regs.a();
    let carry = (a & 0x01) != 0;
    let result = (a >> 1) | ((carry as u8) << 7);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    4
}

/// RLA - Rotate A left through Carry flag
pub(in crate::cpu) fn rla(cpu: &mut Cpu) -> u8 {
    let a = cpu.regs.a();
    let carry = cpu.regs.carry_flag();
    let new_carry = (a & 0x80) != 0;
    let result = (a << 1) | (carry as u8);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    4
}

/// RRA - Rotate A right through Carry flag
pub(in crate::cpu) fn rra(cpu: &mut Cpu) -> u8 {
    let a = cpu.regs.a();
    let carry = cpu.regs.carry_flag();
    let new_carry = (a & 0x01) != 0;
    let result = (a >> 1) | ((carry as u8) << 7);
    cpu.regs.set_a(result);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    4

}

/// RLC r8 - Rotate r8 left, old bit 7 to Carry flag
pub(in crate::cpu) fn rlc_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = (value & 0x80) != 0;
    let result = (value << 1) | carry as u8;
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    8
}

/// RLC (HL) - Rotate value at address in HL left, old bit 7 to Carry flag
pub(in crate::cpu) fn rlc_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = (value & 0x80) != 0;
    let result = (value << 1) | carry as u8;
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    16
}

/// RRC r8 - Rotate r8 right, old bit 0 to Carry flag
pub(in crate::cpu) fn rrc_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = (value & 0x01) != 0;
    let result = (value >> 1) | ((carry as u8) << 7);
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    8
}

/// RRC (HL) - Rotate value at address in HL right, old bit 0 to Carry flag
pub(in crate::cpu) fn rrc_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = (value & 0x01) != 0;
    let result = (value >> 1) | ((carry as u8) << 7);
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    16
}

/// RL r8 - Rotate r8 left through Carry flag
pub(in crate::cpu) fn rl_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = cpu.regs.carry_flag();
    let new_carry = (value & 0x80) != 0;
    let result = (value << 1) | (carry as u8);
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    8
}

/// RL (HL) - Rotate value at address in HL left through Carry flag
pub(in crate::cpu) fn rl_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = cpu.regs.carry_flag();
    let new_carry = (value & 0x80) != 0;
    let result = (value << 1) | (carry as u8);
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    16
}

/// RR r8 - Rotate r8 right through Carry flag
pub(in crate::cpu) fn rr_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = cpu.regs.carry_flag();
    let new_carry = (value & 0x01) != 0;
    let result = (value >> 1) | ((carry as u8) << 7);
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    8
}

/// RR (HL) - Rotate value at address in HL right through Carry flag
pub(in crate::cpu) fn rr_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = cpu.regs.carry_flag();
    let new_carry = (value & 0x01) != 0;
    let result = (value >> 1) | ((carry as u8) << 7);
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(new_carry);
    16
}

/// SLA r8 - Shift r8 left into Carry, LSB set to 0
pub(in crate::cpu) fn sla_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = (value & 0x80) != 0;
    let result = value << 1;
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    8
}

/// SLA (HL) - Shift value at address in HL left into Carry, LSB set to 0
pub(in crate::cpu) fn sla_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = (value & 0x80) != 0;
    let result = value << 1;
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    16
}

/// SRA r8 - Shift r8 right into Carry, MSB doesn't change
pub(in crate::cpu) fn sra_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = (value & 0x01) != 0;
    let msb = value & 0x80;
    let result = (value >> 1) | msb;
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    8
}

/// SRA (HL) - Shift value at address in HL right into Carry, MSB doesn't change
pub(in crate::cpu) fn sra_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = (value & 0x01) != 0;
    let msb = value & 0x80;
    let result = (value >> 1) | msb;
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    16
}

/// SRL r8 - Shift r8 right into Carry, MSB set to 0
pub(in crate::cpu) fn srl_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let carry = (value & 0x01) != 0;
    let result = value >> 1;
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    8
}

/// SRL (HL) - Shift value at address in HL right into Carry, MSB set to 0
pub(in crate::cpu) fn srl_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let carry = (value & 0x01) != 0;
    let result = value >> 1;
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(carry);
    16
}

/// SWAP r8 - Swap the upper 4 bits in r8 and the lower 4 ones
pub(in crate::cpu) fn swap_r8(cpu: &mut Cpu, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let result = (value << 4) | (value >> 4);
    cpu.regs.write_reg8(reg, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    8
}

/// SWAP (HL) - Swap the upper 4 bits and the lower 4 ones of the value at address in HL
pub(in crate::cpu) fn swap_mem_hl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let result = (value << 4) | (value >> 4);
    bus.write(hl, result);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);
    16
}

/// BIT imm3, r8 - Test bit imm3 with r8
pub(in crate::cpu) fn bit_imm3_r8(cpu: &mut Cpu, bit: u8, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let mask = 1 << bit;
    let result = (value & mask) == 0;

    cpu.regs.set_zero_flag(result);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    8
}

/// BIT imm3, (HL) - Test bit imm3 with value at address in HL
pub(in crate::cpu) fn bit_imm3_mem_hl(cpu: &mut Cpu, bus: &mut Bus, bit: u8) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let mask = 1 << bit;
    let result = (value & mask) == 0;

    cpu.regs.set_zero_flag(result);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    12
}

/// RES imm3, r8 - Set bit imm3 to 0 in r8
pub(in crate::cpu) fn res_imm3_r8(cpu: &mut Cpu, bit: u8, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let mask = !(1 << bit);
    let result = value & mask;
    cpu.regs.write_reg8(reg, result);
    8
}

/// RES imm3, (HL) - Set bit imm3 to 0 in value at address in HL
pub(in crate::cpu) fn res_imm3_mem_hl(cpu: &mut Cpu, bus: &mut Bus, bit: u8) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let mask = !(1 << bit);
    let result = value & mask;
    bus.write(hl, result);
    12
}

/// SET imm3, r8 - Set bit imm3 to 1 in r8
pub(in crate::cpu) fn set_imm3_r8(cpu: &mut Cpu, bit: u8, reg: Reg8) -> u8 {
    let value = cpu.regs.read_reg8(reg);
    let mask = 1 << bit;
    let result = value | mask;
    cpu.regs.write_reg8(reg, result);
    8
}

/// SET imm3, (HL) - Set bit imm3 to 1 in value at address in HL
pub(in crate::cpu) fn set_imm3_mem_hl(cpu: &mut Cpu, bus: &mut Bus, bit: u8) -> u8 {
    let hl = cpu.regs.hl();
    let value = bus.read(hl);
    let mask = 1 << bit;
    let result = value | mask;
    bus.write(hl, result);
    12
}