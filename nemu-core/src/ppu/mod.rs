mod utils;

use utils::{
    Mode,
    STAT_HBLANK_IRQ,
    STAT_VBLANK_IRQ,
    STAT_OAM_IRQ,
    STAT_LYC_IRQ,
    STAT_LYC_EQ_LY
};
use crate::interrupts::{INT_LCDSTAT, INT_VBLANK};

pub struct Ppu {
    scanline: u8,
    lyc: u8,
    stat: u8,
    dots: u16,
    mode: Mode,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            lyc: 0,
            stat: 0,
            scanline: 0,
            dots: 0,
            mode: Mode::OAMSearch,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
        }
    }

    pub(crate) fn reset(&mut self) {
        self.lyc = 0;
        self.stat = 0;
        self.scanline = 0;
        self.dots = 0;
        self.mode = Mode::OAMSearch;
        self.vram = [0; 0x2000];
        self.oam = [0; 0xA0];
    }

    pub(crate) fn tick(&mut self, cycles: u8) -> u8 {
        let mut ticks = (cycles * 4) as u16;
        let mut irq_mask: u8 = 0;

        while ticks > 0 {
            let current_mode_dots = self.current_mode_dots();
            if ticks >= current_mode_dots {
                self.dots += current_mode_dots;
                ticks -= current_mode_dots;
                irq_mask |= self.switch_modes();
                self.dots = 0;
            } else {
                self.dots += ticks;
                ticks = 0;
            }
        }

        irq_mask
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF41 => self.stat,
            0xFF44 => self.scanline, // LY register
            0xFF45 => self.lyc,
            _ => 0,
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF41 => {
                self.stat = (value & 0xF8) | (self.stat & 0x07);
            }
            0xFF45 => self.lyc = value,
            _ => {}
        }
    }

    fn current_mode_dots(&self) -> u16 {
        match self.mode {
            Mode::OAMSearch => 80 - self.dots,
            Mode::PixelTransfer => 172 - self.dots,
            Mode::HBlank => 204 - self.dots,
            Mode::VBlank => 456 - self.dots,
        }
    }

    fn switch_modes(&mut self) -> u8 {
        let mut irq_mask: u8 = 0;

        match self.mode {
            Mode::OAMSearch => {
                self.mode = Mode::PixelTransfer;
            }
            Mode::PixelTransfer => {
                self.mode = Mode::HBlank;

                if (self.stat & STAT_HBLANK_IRQ) != 0 {
                    irq_mask |= INT_LCDSTAT;
                }
            }
            Mode::HBlank => {
                self.scanline += 1;

                if self.compare_lyc() {
                    irq_mask |= INT_LCDSTAT;
                }

                if self.scanline < 144 {
                    self.mode = Mode::OAMSearch;

                    if (self.stat & STAT_OAM_IRQ) != 0 {
                        irq_mask |= INT_LCDSTAT;
                    }
                } else {
                    self.mode = Mode::VBlank;
                    irq_mask |= INT_VBLANK;

                    if (self.stat & STAT_VBLANK_IRQ) != 0 {
                        irq_mask |= INT_LCDSTAT;
                    }
                }
            }
            Mode::VBlank => {
                self.scanline += 1;

                if self.compare_lyc() {
                    irq_mask |= INT_LCDSTAT;
                }

                if self.scanline > 153 {
                    self.scanline = 0;
                    self.mode = Mode::OAMSearch;

                    if (self.stat & STAT_OAM_IRQ) != 0 {
                        irq_mask |= INT_LCDSTAT;
                    }
                }
            }
        }

        self.stat = (self.stat & 0xFC) | (self.mode as u8);

        irq_mask
    }

    #[inline(always)]
    fn compare_lyc(&mut self) -> bool {
        if self.scanline == self.lyc {
            self.stat |= STAT_LYC_EQ_LY;
            (self.stat & STAT_LYC_IRQ) != 0
        } else {
            self.stat &= !STAT_LYC_EQ_LY;
            false
        }
    }
}
