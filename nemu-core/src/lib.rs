mod memory;
mod cpu;
mod traits;

use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn with_rom<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let mut nemu = Self::default();
        nemu.load_cartridge(path)?;
        Ok(nemu)
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.borrow_mut().reset();
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step()
    }

    pub fn load_cartridge<P: AsRef<std::path::Path>>(&mut self, path: P) -> std::io::Result<()> {
        let data = std::fs::read(path)?;
        self.memory.borrow_mut().load_cartridge_bytes(&data);
        Ok(())
    }

    pub fn get_regs_snapshot(&self) -> String {
        self.cpu.regs.get_snapshot()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Bus;
    use super::*;

    fn run_test_rom(path: &str, max_cycles: u64) -> String {
        let mut nemu = Nemu::with_rom(path).expect("Failed to load ROM");

        let mut serial_output = String::new();
        let mut cycles = 0;

        while cycles < max_cycles {
            let c = nemu.step() as u64;
            cycles += c;

            if nemu.cpu.memory.borrow().read(0xFF02) == 0x81 {
                let byte = nemu.cpu.memory.borrow().read(0xFF01);
                serial_output.push(byte as char);

                nemu.cpu.memory.borrow_mut().write(0xFF02, 0x00);

                if serial_output.contains("Passed") ||
                    serial_output.contains("Failed") {
                    break;
                }
            }
        }

        serial_output
    }

    #[test]
    fn test_cpu_instrs_01() {
        let output = run_test_rom(
            "../tests/cpu_instrs/individual/01-special.gb",
            1_000_000
        );

        println!("Output:\n{}", output);
        assert!(output.contains("Passed"));
    }
}