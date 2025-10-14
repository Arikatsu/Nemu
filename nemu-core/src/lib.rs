mod bus;
mod cpu;

pub struct Nemu {
    cpu: cpu::CPU<bus::TestBus>
}

impl Nemu {
    pub fn new() -> Self {
        let bus = bus::TestBus::new();
        let cpu = cpu::CPU::new(bus);
        Self { cpu }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.memory.reset();
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step()
    }

    pub fn get_regs_snapshot(&self) -> String {
        self.cpu.regs.get_snapshot()
    }
}