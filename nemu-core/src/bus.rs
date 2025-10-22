use crate::timer::Timer;
use crate::ppu::Ppu;

pub(crate) struct Bus {
    cartridge: [u8; 0x8000], // 32KB Cartridge ROM
    eram: [u8; 0x2000],      // 8KB External RAM
    wram: [u8; 0x2000],      // 8KB Work RAM
    io: [u8; 0x80],          // I/O Registers
    hram: [u8; 0x7F],        // High RAM
    ie: u8,                  // Interrupt Enable Register
    timer: Timer,
    ppu: Ppu,

    #[cfg(test)]
    pub(crate) serial_output: String,
}

impl Bus {
    pub(crate) fn new() -> Self {
        Self {
            cartridge: [0; 0x8000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
            timer: Timer::new(),
            ppu: Ppu::new(),
            
            #[cfg(test)]
            serial_output: String::new(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.cartridge = [0; 0x8000];
        self.eram = [0; 0x2000];
        self.wram = [0; 0x2000];
        self.io = [0; 0x80];
        self.hram = [0; 0x7F];
        self.ie = 0;

        self.timer.reset();
        self.ppu.reset();

        #[cfg(test)]
        {
            self.serial_output.clear();
        }
    }

    pub(crate) fn load_cartridge_bytes(&mut self, data: &[u8]) {
        let len = data.len().min(self.cartridge.len());
        self.cartridge[..len].copy_from_slice(&data[..len]);
    }

    pub(crate) fn tick(&mut self, cycles: u8) {
        self.ppu.tick(cycles);
        let interrupt = self.timer.update(cycles);
        if interrupt {
            self.io[0x0F] |= 0x04;
        }
    }

    #[inline(always)]
    pub(crate) fn get_ie_if(&self) -> (u8, u8) {
        (self.ie, self.io[0x0F])
    }

    #[inline(always)]
    pub(crate) fn read(&mut self, addr: u16) -> u8 {
        let data = match addr {
            0x0000..=0x7FFF => self.cartridge[addr as usize],
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], // Echo RAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFEA0..=0xFEFF => 0, // unusable
            0xFF04..=0xFF07 => self.timer.read(addr),
            0xFF44 => 0x90, // LY register (stubbed)
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        };

        self.tick(1);
        data
    }
    
    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => { /* ROM area (no write) */ }
            0x8000..=0x9FFF => self.ppu.write(addr, data),
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize] = data,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = data,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = data, // Echo RAM
            0xFE00..=0xFE9F => self.ppu.write(addr, data),
            0xFEA0..=0xFEFF => { /* unusable */ }
            0xFF02 => {
                self.io[(addr - 0xFF00) as usize] = data;
                if data == 0x81 {
                    #[cfg(test)]
                    {
                        self.serial_output.push(self.io[0x01] as char);
                    }
                    self.io[0x02] = 0;
                }
            }
            0xFF04..=0xFF07 => self.timer.write(addr, data),
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = data,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.ie = data,
        };

        self.tick(1);
    }

    #[inline(always)]
    pub(crate) fn read_u16(&mut self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;
        (high << 8) | low
    }

    #[inline(always)]
    pub(crate) fn write_u16(&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.write(addr, lo);
        self.write(addr + 1, hi);
    }
}