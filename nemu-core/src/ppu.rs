#[derive(Debug)]
enum Mode {
    HBlank = 0x00,
    VBlank = 0x01,
    OAMSearch = 0x02,
    PixelTransfer = 0x03,
}

pub struct Ppu {
    dots: u16,
    scanlines: u16,
    mode: Mode,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            dots: 0,
            scanlines: 0,
            mode: Mode::OAMSearch,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
        }
    }

    pub(crate) fn reset(&mut self) {
        self.dots = 0;
        self.scanlines = 0;
        self.mode = Mode::OAMSearch;
    }

    pub(crate) fn tick(&mut self, cycles: u8) {
        let mut ticks = (cycles * 4) as u16;
        while ticks > 0 {
            let current_mode_dots = self.current_mode_dots();
            if ticks >= current_mode_dots {
                self.dots += current_mode_dots;
                ticks -= current_mode_dots;
                self.switch_modes();
                self.dots = 0;
            } else {
                self.dots += ticks;
                ticks = 0;
            }
        }
    }

    fn current_mode_dots(&self) -> u16 {
        match self.mode {
            Mode::OAMSearch => 80 - self.dots,
            Mode::PixelTransfer => 252 - self.dots,
            Mode::HBlank => 456 - self.dots,
            Mode::VBlank => 456 - self.dots,
        }
    }

    fn switch_modes(&mut self) {
        match self.mode {
            Mode::OAMSearch => {
                self.mode = Mode::PixelTransfer;
            }
            Mode::PixelTransfer => {
                self.mode = Mode::HBlank;
            }
            Mode::HBlank => {
                self.scanlines += 1;
                if self.scanlines < 144 {
                    self.mode = Mode::OAMSearch;
                } else {
                    self.mode = Mode::VBlank;
                }
            }
            Mode::VBlank => {
                self.scanlines += 1;
                if self.scanlines > 153 {
                    self.scanlines = 0;
                    self.mode = Mode::OAMSearch;
                }
            }
        }
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            _ => 0xFF,
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            _ => {}
        }
    }
}