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
