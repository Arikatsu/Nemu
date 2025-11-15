mod utils;

use crate::interrupts::{INT_LCDSTAT, INT_VBLANK};
use utils::{Mode, STAT_HBLANK_IRQ, STAT_LYC_EQ_LY, STAT_LYC_IRQ, STAT_OAM_IRQ, STAT_VBLANK_IRQ};

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

pub struct Ppu {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dots: u16,
    mode: Mode,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],

    pub framebuffer: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    pub frame_ready: bool,
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dots: 0,
            mode: Mode::OAMSearch,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            framebuffer: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            frame_ready: false,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.lcdc = 0;
        self.stat = 0;
        self.scy = 0;
        self.scx = 0;
        self.ly = 0;
        self.lyc = 0;
        self.dots = 0;
        self.mode = Mode::OAMSearch;
        self.vram = [0; 0x2000];
        self.oam = [0; 0xA0];
        self.framebuffer = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.frame_ready = false;
    }

    pub(crate) fn update(&mut self, cycles: u8) -> u8 {
        if (self.lcdc & 0x80) == 0 {
            // LCD is off
            return 0;
        }

        let mut irq_mask: u8 = 0;
        self.dots += (cycles * 4) as u16;

        loop {
            let threshold = match self.mode {
                Mode::OAMSearch => 80,
                Mode::PixelTransfer => 252,
                Mode::HBlank => 456,
                Mode::VBlank => 456, // each line in vblank is 456 dots
            };

            if self.dots < threshold {
                break;
            }

            irq_mask |= self.switch_modes();
            while self.dots >= 456 {
                self.dots -= 456;
            }
        }

        irq_mask
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,

            _ => panic!("PPU read from invalid address: {:#06X}", addr),
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF40 => self.set_lcdc(value),
            0xFF41 => self.stat = (value & 0xF8) | (self.stat & 0x07),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => (), // LY is read-only
            0xFF45 => self.lyc = value,

            _ => panic!("PPU write to invalid address: {:#06X}", addr),
        }
    }

    #[inline(always)]
    fn switch_modes(&mut self) -> u8 {
        let mut irq_mask: u8 = 0;

        match self.mode {
            Mode::OAMSearch => {
                self.mode = Mode::PixelTransfer;
            }
            Mode::PixelTransfer => {
                self.render_scanline();
                self.mode = Mode::HBlank;

                if (self.stat & STAT_HBLANK_IRQ) != 0 {
                    irq_mask |= INT_LCDSTAT;
                }
            }
            Mode::HBlank => {
                self.ly += 1;

                if self.compare_lyc() {
                    irq_mask |= INT_LCDSTAT;
                }

                if self.ly < 144 {
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

                    self.frame_ready = true;
                }
            }
            Mode::VBlank => {
                self.ly += 1;

                if self.compare_lyc() {
                    irq_mask |= INT_LCDSTAT;
                }

                if self.ly > 153 {
                    self.ly = 0;
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
    fn set_lcdc(&mut self, value: u8) {
        let new_lcd_enabled = (value & 0x80) != 0;
        let old_lcd_enabled = (self.lcdc & 0x80) != 0;

        if !new_lcd_enabled && old_lcd_enabled {
            #[cfg(debug_assertions)]
            if self.mode != Mode::VBlank {
                eprintln!("LCD turned off outside of VBlank (mode {})", self.mode as u8);
            }

            self.ly = 0;
            self.dots = 0;
            self.mode = Mode::HBlank;
            self.stat = (self.stat & 0xFC) | (Mode::HBlank as u8);
        }

        if new_lcd_enabled && !old_lcd_enabled {
            self.ly = 0;
            self.dots = 0;
            self.mode = Mode::OAMSearch;
            self.stat = (self.stat & 0xFC) | (Mode::OAMSearch as u8);
            self.compare_lyc();
        }

        self.lcdc = value;
    }

    #[inline(always)]
    fn compare_lyc(&mut self) -> bool {
        if self.ly == self.lyc {
            self.stat |= STAT_LYC_EQ_LY;
            (self.stat & STAT_LYC_IRQ) != 0
        } else {
            self.stat &= !STAT_LYC_EQ_LY;
            false
        }
    }

    fn render_scanline(&mut self) {
        let bg_enabled = (self.lcdc & 0x01) != 0;

        if !bg_enabled {
            let y = self.ly as usize;
            for x in 0..SCREEN_WIDTH {
                self.framebuffer[y][x] = 0;
            }

            return;
        }

        let y = self.ly as usize;
        let y_pos = self.ly.wrapping_add(self.scy);

        let tile_row = (y_pos / 8) as usize;
        let tile_line = (y_pos % 8) as usize;

        let tilemap_base = if (self.lcdc & 0x08) != 0 { 0x1C00 } else { 0x1800 };

        let tile_data_unsigned: bool = (self.lcdc & 0x10) != 0;

        for x in 0..SCREEN_WIDTH {
            let x = x.wrapping_add(self.scx as usize);
            let tile_num = self.vram[tilemap_base + (tile_row * 32) + (x / 8)];

            let tile_addr = if tile_data_unsigned {
                tile_num as usize * 16
            } else {
                0x1000_i16.wrapping_add((tile_num as i8 as i16) * 16) as usize
            };

            let line_addr = tile_addr + (tile_line * 2);
            let byte1 = self.vram[line_addr];
            let byte2 = self.vram[line_addr + 1];

            let bit_index = 7 - (x % 8);
            let color_bit0 = (byte1 >> bit_index) & 0x01;
            let color_bit1 = (byte2 >> bit_index) & 0x01;
            let color = (color_bit1 << 1) | color_bit0;

            self.framebuffer[y][x] = color;
        }
    }
}
