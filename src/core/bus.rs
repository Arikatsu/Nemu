pub(super) trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub struct TestBus {
    memory: [u8; 65536],
}

impl TestBus {
    pub fn new() -> Self {
        let mut memory = [0; 65536];
        memory[0x0100] = 0x3E; // LD A, 0x42
        memory[0x0101] = 0x42;
        
        memory[0x0102] = 0x06; // LD B, 0x37
        memory[0x0103] = 0x37;
        
        memory[0x0104] = 0x0E; // LD C, 0x18
        memory[0x0105] = 0x18;

        memory[0x0106] = 0x80; // ADD A, B

        memory[0x0107] = 0x00; // NOP
        
        Self {
            memory
        }
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