mod bus;
mod cpu;

pub struct Nemu {
    cpu: cpu::CPU,
    bus: bus::TestBus,
}

impl Nemu {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::TestBus::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus = bus::TestBus::new();
    }
    
    pub fn step(&mut self) -> u8 {
        self.cpu.step(&mut self.bus)
    }
    
    pub fn get_regs_snapshot(&self) -> String {
        self.cpu.regs.get_snapshot()
    }
}