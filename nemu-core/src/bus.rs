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