use std::rc::Rc;
use std::cell::RefCell;

mod memory;
mod cpu;
mod traits;

pub struct Nemu {
    cpu: cpu::Cpu<memory::Memory>,
    memory: Rc<RefCell<memory::Memory>>
}

impl Default for Nemu {
    fn default() -> Self {
        let memory = Rc::new(RefCell::new(memory::Memory::new()));
        let cpu = cpu::Cpu::new(Rc::clone(&memory));
        Self { memory, cpu }
    }
}

impl Nemu {
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.borrow_mut().reset();
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step()
    }

    pub fn get_regs_snapshot(&self) -> String {
        self.cpu.regs.get_snapshot()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Bus;
    use super::Nemu;

    #[test]
    fn test_all_opcodes() {
        let mut nemu = Nemu::default();

        // Step 1: LD A, 0x42
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.a(), 0x42);
        assert_eq!(nemu.cpu.regs.pc(), 0x0102);

        // Step 2: LD B, 0x10
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.b(), 0x10);
        assert_eq!(nemu.cpu.regs.pc(), 0x0104);

        // Step 3: LD BC, 0xC050
        let cycles = nemu.step();
        assert_eq!(cycles, 12);
        assert_eq!(nemu.cpu.regs.bc(), 0xC050);
        assert_eq!(nemu.cpu.regs.b(), 0xC0);
        assert_eq!(nemu.cpu.regs.c(), 0x50);
        assert_eq!(nemu.cpu.regs.pc(), 0x0107);

        // Step 4: LD (BC), A
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.memory.borrow().read(0xC050), 0x42);
        assert_eq!(nemu.cpu.regs.pc(), 0x0108);

        // Step 5: INC BC
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.bc(), 0xC051);
        assert_eq!(nemu.cpu.regs.pc(), 0x0109);

        // Step 6: INC B
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.b(), 0xC1);
        assert_eq!(nemu.cpu.regs.zero_flag(), false);
        assert_eq!(nemu.cpu.regs.subtract_flag(), false);
        assert_eq!(nemu.cpu.regs.half_carry_flag(), false);
        assert_eq!(nemu.cpu.regs.pc(), 0x010A);

        // Step 7: DEC B
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.b(), 0xC0);
        assert_eq!(nemu.cpu.regs.zero_flag(), false);
        assert_eq!(nemu.cpu.regs.subtract_flag(), true);
        assert_eq!(nemu.cpu.regs.half_carry_flag(), false);
        assert_eq!(nemu.cpu.regs.pc(), 0x010B);

        // Step 8: DEC B again
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.b(), 0xBF);
        assert_eq!(nemu.cpu.regs.zero_flag(), false);
        assert_eq!(nemu.cpu.regs.subtract_flag(), true);
        assert_eq!(nemu.cpu.regs.half_carry_flag(), true);
        assert_eq!(nemu.cpu.regs.pc(), 0x010C);

        // Step 9: LD A, 0x80
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.a(), 0x80);
        assert_eq!(nemu.cpu.regs.pc(), 0x010E);

        // Step 10: RLCA
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.a(), 0x01);
        assert_eq!(nemu.cpu.regs.zero_flag(), false);
        assert_eq!(nemu.cpu.regs.subtract_flag(), false);
        assert_eq!(nemu.cpu.regs.half_carry_flag(), false);
        assert_eq!(nemu.cpu.regs.carry_flag(), true);
        assert_eq!(nemu.cpu.regs.pc(), 0x010F);

        // Step 11: LD A, 0x0F
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.a(), 0x0F);
        assert_eq!(nemu.cpu.regs.pc(), 0x0111);

        // Step 12: LD B, 0x01
        let cycles = nemu.step();
        assert_eq!(cycles, 8);
        assert_eq!(nemu.cpu.regs.b(), 0x01);
        assert_eq!(nemu.cpu.regs.pc(), 0x0113);

        // Step 13: ADD A, B
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.a(), 0x10); // 0x0F + 0x01 = 0x10
        assert_eq!(nemu.cpu.regs.zero_flag(), false);
        assert_eq!(nemu.cpu.regs.subtract_flag(), false);
        assert_eq!(nemu.cpu.regs.half_carry_flag(), true); // 0x0F + 0x01 causes half-carry
        assert_eq!(nemu.cpu.regs.carry_flag(), false);
        assert_eq!(nemu.cpu.regs.pc(), 0x0114);

        // Step 14: NOP
        let cycles = nemu.step();
        assert_eq!(cycles, 4);
        assert_eq!(nemu.cpu.regs.pc(), 0x0115);
    }
}