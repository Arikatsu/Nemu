use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::joypad::Joypad;
use crate::mbc::MbcType;

const BOOT_ROM: &[u8; 0x100] = include_bytes!("../bootrom/build/dmg_boot.bin");

pub(crate) struct Bus {
    pub(crate) mbc: MbcType,
    pub(crate) eram: [u8; 0x2000],      // 8KB External RAM
    pub(crate) wram: [u8; 0x2000],      // 8KB Work RAM
    pub(crate) io: [u8; 0x80],          // I/O Registers
    pub(crate) hram: [u8; 0x7F],        // High RAM
    pub(crate) ie: u8,                  // Interrupt Enable Register
    pub(crate) timer: Timer,
    pub(crate) ppu: Ppu,
    pub(crate) joypad: Joypad,
    pub(crate) boot_rom_enabled: bool,

    #[cfg(test)]
    pub(crate) serial_output: String,
}

impl Bus {
    pub(crate) fn new() -> Self {
        Self {
            mbc: MbcType::default(),
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
            timer: Timer::new(),
            ppu: Ppu::new(),
            joypad: Joypad::new(),
            boot_rom_enabled: true,

            #[cfg(test)]
            serial_output: String::new(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.eram = [0; 0x2000];
        self.wram = [0; 0x2000];
        self.io = [0; 0x80];
        self.hram = [0; 0x7F];
        self.ie = 0;
        self.boot_rom_enabled = true;

        self.timer.reset();
        self.ppu.reset();

        #[cfg(test)]
        self.serial_output.clear();
    }

    pub(crate) fn tick(&mut self, cycles: u8) {
        let ppu_irq_mask = self.ppu.update(cycles);
        let timer_irq_mask = self.timer.update(cycles);
        let joypad_irq_mask = self.joypad.poll_interrupt();

        self.io[0x0F] |= ppu_irq_mask | timer_irq_mask | joypad_irq_mask;
    }

    #[inline(always)]
    pub(crate) fn get_ie_if(&self) -> (u8, u8) {
        (self.ie, self.io[0x0F])
    }

    #[inline(always)]
    pub(crate) fn peek(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF if self.boot_rom_enabled => unsafe { *BOOT_ROM.get_unchecked(addr as usize) },
            0x0000..=0x7FFF => self.mbc.read(addr),
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xA000..=0xBFFF => self.mbc.read(addr),
            0xC000..=0xDFFF => unsafe { *self.wram.get_unchecked((addr - 0xC000) as usize) },
            0xE000..=0xFDFF => unsafe { *self.wram.get_unchecked((addr - 0xE000) as usize) }, // Echo RAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFEA0..=0xFEFF => 0, // unusable
            0xFF00 => self.joypad.read(),
            0xFF04..=0xFF07 => self.timer.read(addr),
            0xFF40..=0xFF45 => self.ppu.read(addr),
            0xFF47..=0xFF4B => self.ppu.read(addr),
            0xFF80..=0xFFFE => unsafe { *self.hram.get_unchecked((addr - 0xFF80) as usize) },
            0xFFFF => self.ie,
            _ => unsafe { *self.io.get_unchecked((addr - 0xFF00) as usize) }, // Fallback for unimplemented I/O
        }
    }

    #[inline(always)]
    pub(crate) fn read(&mut self, addr: u16) -> u8 {
        self.tick(1);
        self.peek(addr)
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, data: u8) {
        self.tick(1);

        match addr {
            0x0000..=0x7FFF => self.mbc.write(addr, data),
            0x8000..=0x9FFF => self.ppu.write(addr, data),
            0xA000..=0xBFFF => unsafe { *self.eram.get_unchecked_mut((addr - 0xA000) as usize) = data },
            0xC000..=0xDFFF => unsafe { *self.wram.get_unchecked_mut((addr - 0xC000) as usize) = data },
            0xE000..=0xFDFF => unsafe { *self.wram.get_unchecked_mut((addr - 0xE000) as usize) = data }, // Echo RAM
            0xFE00..=0xFE9F => self.ppu.write(addr, data),
            0xFEA0..=0xFEFF => { /* unusable */ }
            0xFF00 => self.joypad.write(data),
            0xFF02 => {
                unsafe { *self.io.get_unchecked_mut((addr - 0xFF00) as usize) = data };
                if data == 0x81 {
                    #[cfg(test)]
                    {
                        self.serial_output.push(self.io[0x01] as char);
                    }
                    self.io[0x02] = 0;
                }
            }
            0xFF04..=0xFF07 => self.timer.write(addr, data),
            0xFF40..=0xFF45 => self.ppu.write(addr, data),
            0xFF46 => self.transfer_dma(data),
            0xFF47..=0xFF4B => self.ppu.write(addr, data),
            0xFF50 => self.boot_rom_enabled = false,
            0xFF80..=0xFFFE => unsafe { *self.hram.get_unchecked_mut((addr - 0xFF80) as usize) = data },
            0xFFFF => self.ie = data,
            _ => unsafe { *self.io.get_unchecked_mut((addr - 0xFF00) as usize) = data } // Fallback for unimplemented I/O
        };
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

    fn transfer_dma(&mut self, start_addr: u8) {
        let base_addr = (start_addr as u16) << 8;
        for i in 0..0xA0 {
            let data = self.peek(base_addr + i);
            self.ppu.write(0xFE00 + i, data);
        }
    }
}
