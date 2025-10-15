use crate::traits::Bus;

pub struct Memory {
    cartridge: [u8; 0x8000], // 32KB Cartridge ROM
    vram: [u8; 0x2000],      // 8KB Video RAM
    eram: [u8; 0x2000],      // 8KB External RAM
    wram: [u8; 0x2000],      // 8KB Work RAM
    oam: [u8; 0xA0],         // Sprite Attribute Table
    io: [u8; 0x80],          // I/O Registers
    hram: [u8; 0x7F],        // High RAM
    ie: u8,                  // Interrupt Enable Register
}

impl Memory {
    pub fn new() -> Self {
        Self {
            cartridge: [0; 0x8000],
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.cartridge = [0; 0x8000];
        self.vram = [0; 0x2000];
        self.eram = [0; 0x2000];
        self.wram = [0; 0x2000];
        self.oam = [0; 0xA0];
        self.io = [0; 0x80];
        self.hram = [0; 0x7F];
        self.ie = 0;
    }
}

impl Bus for Memory {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], // Echo RAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0, // unusable
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => { /* ROM area (no write) */ }
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = data,
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize] = data,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = data,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = data, // Echo RAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = data,
            0xFEA0..=0xFEFF => { /* unusable */ }
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = data,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.ie = data,
        }
    }
}