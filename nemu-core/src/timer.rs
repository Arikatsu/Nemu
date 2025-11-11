use crate::interrupts::INT_TIMER;

pub struct Timer {
    tima: u8,
    tma: u8,
    tac: u8,
    div: u16,
    overflow_cycles: u8,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            tima: 0,
            tma: 0,
            tac: 0,
            div: 0,
            overflow_cycles: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.tima = 0;
        self.tma = 0;
        self.tac = 0;
        self.div = 0;
        self.overflow_cycles = 0;
    }

    pub(crate) fn update(&mut self, cycles: u8) -> u8 {
        let mut irq_mask = 0;

        if self.overflow_cycles > 0 {
            self.overflow_cycles = self.overflow_cycles.saturating_sub(cycles);
            if self.overflow_cycles == 0 {
                self.tima = self.tma;
                irq_mask = INT_TIMER;
            }
        }

        let old_div = self.div;
        self.div = self.div.wrapping_add((cycles * 4) as u16);

        if (self.tac & 0x04) != 0 {
            let div_bit = self.get_bit_position();
            let old_bit = (old_div >> div_bit) & 1;
            let new_bit = (self.div >> div_bit) & 1;

            if old_bit == 1 && new_bit == 0 {
                self.increment_tima();
            }
        }

        irq_mask
    }

    fn increment_tima(&mut self) {
        if self.tima == 0xFF {
            self.tima = 0;
            self.overflow_cycles = 1;
        } else {
            self.tima = self.tima.wrapping_add(1);
        }
    }

    fn get_bit_position(&self) -> u8 {
        match self.tac & 0x03 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac | 0xF8,
            _ => 0xFF,
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF04 => self.div = 0,
            0xFF05 => {
                self.tima = data;
                self.overflow_cycles = 0;
            }
            0xFF06 => self.tma = data,
            0xFF07 => self.tac = data & 0x07,
            _ => {}
        }
    }
}