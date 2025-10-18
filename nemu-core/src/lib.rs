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
    use super::*;

    fn run_test_rom(path: &str) -> bool {
        let mut nemu = Nemu::with_rom(path).expect("Failed to load ROM");

        for i in 0..10_000_000 {
            nemu.step();

            if i % 10000 == 0 {
                let output = &nemu.memory.borrow().serial_output;
                if output.contains("Passed") {
                    return true;
                } else if output.contains("Failed") {
                    println!("\x1b[32mSerial Output:\x1b[0m\n{}", output);
                    return false;
                }
            }
        }

        false
    }

    #[test]
    fn test_cpu_instrs_01() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/01-special.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_02() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/02-interrupts.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_03() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/03-op sp,hl.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_04() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/04-op r,imm.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_05() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/05-op rp.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_06() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/06-ld r,r.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_07() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_08() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/08-misc instrs.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_09() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/09-op r,r.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_10() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/10-bit ops.gb",
        );
        assert!(result);
    }

    #[test]
    fn test_cpu_instrs_11() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/11-op a,(hl).gb",
        );
        assert!(result);
    }
}