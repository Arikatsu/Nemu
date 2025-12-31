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
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    dots: u16,
    mode: Mode,
    wline_counter: u8,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],

    pub framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            dots: 0,
            mode: Mode::OAMSearch,
            wline_counter: 0,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
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
        self.wy = 0;
        self.wx = 0;
        self.dots = 0;
        self.mode = Mode::OAMSearch;
        self.wline_counter = 0;
        self.vram = [0; 0x2000];
        self.oam = [0; 0xA0];
        self.framebuffer = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
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
            0x8000..=0x9FFF => unsafe { *self.vram.get_unchecked((addr - 0x8000) as usize) },
            0xFE00..=0xFE9F => unsafe { *self.oam.get_unchecked((addr - 0xFE00) as usize) },
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            _ => panic!("PPU read from invalid address: {:#06X}", addr),
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => unsafe { *self.vram.get_unchecked_mut((addr - 0x8000) as usize) = value },
            0xFE00..=0xFE9F => unsafe { *self.oam.get_unchecked_mut((addr - 0xFE00) as usize) = value },
            0xFF40 => self.set_lcdc(value),
            0xFF41 => self.stat = (value & 0xF8) | (self.stat & 0x07),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => (), // LY is read-only
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

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
                self.draw_scanline();
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
                    self.wline_counter = 0;

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
                eprintln!(
                    "LCD turned off outside of VBlank (mode {})",
                    self.mode as u8
                );
            }

            self.ly = 0;
            self.dots = 0;
            self.mode = Mode::HBlank;
            self.wline_counter = 0;
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

    fn draw_scanline(&mut self) {
        let start = (self.ly as usize) * SCREEN_WIDTH;
        let end = start + SCREEN_WIDTH;
        let scanline = &mut self.framebuffer[start..end];
        let mut bg_priority: [bool; SCREEN_WIDTH] = [false; SCREEN_WIDTH];

        // Draw background
        if (self.lcdc & 0x01) != 0 {
            let y_pos = self.ly.wrapping_add(self.scy);
            let tile_row = (y_pos / 8) as usize;
            let tile_line = (y_pos % 8) as usize;

            let tilemap_base = if (self.lcdc & 0x08) != 0 { 0x1C00 } else { 0x1800 };
            let tile_data_unsigned: bool = (self.lcdc & 0x10) != 0;

            for x in 0..SCREEN_WIDTH {
                let x_pos = x.wrapping_add(self.scx as usize) % 256;
                let tile_num = self.vram[tilemap_base + (tile_row * 32) + (x_pos / 8)];

                let tile_addr = if tile_data_unsigned {
                    tile_num as usize * 16
                } else {
                    0x1000_i16.wrapping_add((tile_num as i8 as i16) * 16) as usize
                };

                let line_addr = tile_addr + (tile_line * 2);
                let byte1 = self.vram[line_addr];
                let byte2 = self.vram[line_addr + 1];

                let bit_index = 7 - (x_pos % 8);
                let color_bit0 = (byte1 >> bit_index) & 0x01;
                let color_bit1 = (byte2 >> bit_index) & 0x01;
                let color_raw = (color_bit1 << 1) | color_bit0;
                let color = (self.bgp >> (color_raw * 2)) & 0x03;

                bg_priority[x] = color_raw != 0;
                scanline[x] = color;
            }
        }

        // Draw window
        if (self.lcdc & 0x20) != 0 && self.wy <= self.ly && self.wx < 167 {
            let tile_row = (self.wline_counter / 8) as usize;
            let tile_line = (self.wline_counter % 8) as usize;

            let tilemap_base = if (self.lcdc & 0x40) != 0 { 0x1C00 } else { 0x1800 };
            let loop_start = (self.wx as usize).saturating_sub(7);

            for x in loop_start..SCREEN_WIDTH {
                let window_x = x + 7 - (self.wx as usize);
                let tile_num = self.vram[tilemap_base + (tile_row * 32) + (window_x / 8)];

                let tile_addr = if (self.lcdc & 0x10) != 0 {
                    tile_num as usize * 16
                } else {
                    0x1000_i16.wrapping_add((tile_num as i8 as i16) * 16) as usize
                };

                let line_addr = tile_addr + (tile_line * 2);
                let byte1 = self.vram[line_addr];
                let byte2 = self.vram[line_addr + 1];

                let bit_index = 7 - (window_x % 8);
                let color_bit0 = (byte1 >> bit_index) & 0x01;
                let color_bit1 = (byte2 >> bit_index) & 0x01;
                let color_raw = (color_bit1 << 1) | color_bit0;
                let color = (self.bgp >> (color_raw * 2)) & 0x03;

                bg_priority[x] = color_raw != 0;
                scanline[x] = color;
            }

            self.wline_counter = self.wline_counter.wrapping_add(1);
        }

        // Draw sprites
        if (self.lcdc & 0x02) != 0 {
            let sprite_height = if (self.lcdc & 0x04) != 0 { 16 } else { 8 };
            let ly = self.ly as i16;

            let mut sprites = [(0i16, 0i16, 0u8, 0u8); 10];
            let mut sprite_count = 0;

            for sprite in self.oam.chunks_exact(4) {
                let sprite_y = (sprite[0] as i16) - 16;
                let sprite_x = (sprite[1] as i16) - 8;

                if ly >= sprite_y && ly < sprite_y + (sprite_height as i16) {
                    let mut i = sprite_count;

                    while i > 0 && sprites[i - 1].0 <= sprite_x {
                        sprites[i] = sprites[i - 1];
                        i -= 1;
                    }

                    sprites[i] = (sprite_x, sprite_y, sprite[2], sprite[3]);
                    sprite_count += 1;

                    if sprite_count >= 10 {
                        break;
                    }
                }
            }

            for &(sprite_x, sprite_y, tile_num, attributes) in &sprites[..sprite_count] {
                let y_offset = (ly - sprite_y) as u8;
                let y_flip = (attributes & 0x40) != 0;
                let x_flip = (attributes & 0x20) != 0;
                let palette = if (attributes & 0x10) != 0 { self.obp1 } else { self.obp0 };

                let tile_line = if y_flip {
                    sprite_height - 1 - y_offset
                } else {
                    y_offset
                } as usize;

                let tile_addr = if sprite_height == 16 {
                    (tile_num & 0xFE) as usize * 16
                } else {
                    tile_num as usize * 16
                };

                let line_addr = tile_addr + (tile_line * 2);
                let byte1 = self.vram[line_addr];
                let byte2 = self.vram[line_addr + 1];

                for x in 0..8 {
                    let pixel_x = sprite_x + (x as i16);

                    if pixel_x < 0 || (pixel_x as usize) >= SCREEN_WIDTH ||
                        bg_priority[pixel_x as usize] && (attributes & 0x80) != 0
                    {
                        continue;
                    }

                    let bit_index = if x_flip { x } else { 7 - x };
                    let color_bit0 = (byte1 >> bit_index) & 0x01;
                    let color_bit1 = (byte2 >> bit_index) & 0x01;
                    let color_raw = (color_bit1 << 1) | color_bit0;

                    if color_raw == 0 {
                        continue;
                    }

                    let color = (palette >> (color_raw * 2)) & 0x03;
                    scanline[pixel_x as usize] = color;
                }
            }
        }
    }
}
