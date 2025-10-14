pub(super) trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);

    fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;
        (high << 8) | low
    }

    fn write_u16(&mut self, addr: u16, data: u16) {
        let low = (data & 0x00FF) as u8;
        let high = (data >> 8) as u8;
        self.write(addr, low);
        self.write(addr + 1, high);
    }
}

pub struct TestBus {
    memory: [u8; 65536],
}

impl TestBus {
    pub fn new() -> Self {
        let mut memory = [0; 65536];
        // Test program starting at 0x0100
        memory[0x0100] = 0x3E; // LD A, 0x42
        memory[0x0101] = 0x42;

        memory[0x0102] = 0x06; // LD B, 0x10
        memory[0x0103] = 0x10;

        memory[0x0104] = 0x01; // LD BC, 0xC050
        memory[0x0105] = 0x50;
        memory[0x0106] = 0xC0;

        memory[0x0107] = 0x02; // LD (BC), A  -> writes 0x42 to 0xC050

        memory[0x0108] = 0x03; // INC BC  -> BC becomes 0xC051

        memory[0x0109] = 0x04; // INC B  -> B becomes 0xC1

        memory[0x010A] = 0x05; // DEC B  -> B becomes 0xC0
        memory[0x010B] = 0x05; // DEC B  -> B becomes 0xBF

        memory[0x010C] = 0x3E; // LD A, 0x80
        memory[0x010D] = 0x80;

        memory[0x010E] = 0x07; // RLCA  -> A becomes 0x01, Carry set

        memory[0x010F] = 0x3E; // LD A, 0x0F
        memory[0x0110] = 0x0F;

        memory[0x0111] = 0x06; // LD B, 0x01
        memory[0x0112] = 0x01;

        memory[0x0113] = 0x80; // ADD A, B  -> A = 0x10, Half-carry set

        memory[0x0114] = 0x00; // NOP

        Self { memory }
    }

    pub(crate) fn reset(&mut self) {
        self.memory = [0; 65536];
    }
}

impl Bus for TestBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;

    #[test]
    fn test_all_opcodes() {
        let bus = TestBus::new();
        let mut cpu = CPU::new(bus);

        // Step 1: LD A, 0x42
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.a(), 0x42);
        assert_eq!(cpu.regs.pc(), 0x0102);

        // Step 2: LD B, 0x10
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.b(), 0x10);
        assert_eq!(cpu.regs.pc(), 0x0104);

        // Step 3: LD BC, 0xC050
        let cycles = cpu.step();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.regs.bc(), 0xC050);
        assert_eq!(cpu.regs.b(), 0xC0);
        assert_eq!(cpu.regs.c(), 0x50);
        assert_eq!(cpu.regs.pc(), 0x0107);

        // Step 4: LD (BC), A
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.memory.read(0xC050), 0x42);
        assert_eq!(cpu.regs.pc(), 0x0108);

        // Step 5: INC BC
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.bc(), 0xC051);
        assert_eq!(cpu.regs.pc(), 0x0109);

        // Step 6: INC B
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.b(), 0xC1);
        assert_eq!(cpu.regs.zero_flag(), false);
        assert_eq!(cpu.regs.subtract_flag(), false);
        assert_eq!(cpu.regs.half_carry_flag(), false);
        assert_eq!(cpu.regs.pc(), 0x010A);

        // Step 7: DEC B
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.b(), 0xC0);
        assert_eq!(cpu.regs.zero_flag(), false);
        assert_eq!(cpu.regs.subtract_flag(), true);
        assert_eq!(cpu.regs.half_carry_flag(), false);
        assert_eq!(cpu.regs.pc(), 0x010B);

        // Step 8: DEC B again
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.b(), 0xBF);
        assert_eq!(cpu.regs.zero_flag(), false);
        assert_eq!(cpu.regs.subtract_flag(), true);
        assert_eq!(cpu.regs.half_carry_flag(), true);
        assert_eq!(cpu.regs.pc(), 0x010C);

        // Step 9: LD A, 0x80
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.a(), 0x80);
        assert_eq!(cpu.regs.pc(), 0x010E);

        // Step 10: RLCA
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.a(), 0x01);
        assert_eq!(cpu.regs.zero_flag(), false);
        assert_eq!(cpu.regs.subtract_flag(), false);
        assert_eq!(cpu.regs.half_carry_flag(), false);
        assert_eq!(cpu.regs.carry_flag(), true);
        assert_eq!(cpu.regs.pc(), 0x010F);

        // Step 11: LD A, 0x0F
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.a(), 0x0F);
        assert_eq!(cpu.regs.pc(), 0x0111);

        // Step 12: LD B, 0x01
        let cycles = cpu.step();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.regs.b(), 0x01);
        assert_eq!(cpu.regs.pc(), 0x0113);

        // Step 13: ADD A, B
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.a(), 0x10); // 0x0F + 0x01 = 0x10
        assert_eq!(cpu.regs.zero_flag(), false);
        assert_eq!(cpu.regs.subtract_flag(), false);
        assert_eq!(cpu.regs.half_carry_flag(), true); // 0x0F + 0x01 causes half-carry
        assert_eq!(cpu.regs.carry_flag(), false);
        assert_eq!(cpu.regs.pc(), 0x0114);

        // Step 14: NOP
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.regs.pc(), 0x0115);
    }
}