pub(crate) struct Joypad {
    buttons: u8,
    directions: u8,
    select: u8,

    prev_buttons: u8,
    prev_directions: u8,
}

impl Joypad {
    pub(crate) fn new() -> Self {
        Self {
            buttons: 0x0F,
            directions: 0x0F,
            select: 0x30,

            prev_buttons: 0x0F,
            prev_directions: 0x0F,
        }
    }

    pub(crate) fn set_joypad(&mut self, input: JoypadButton, pressed: bool, is_direction: bool) {
        let target = if is_direction {
            &mut self.directions
        } else {
            &mut self.buttons
        };

        if pressed {
            *target &= !(input as u8);
        } else {
            *target |= input as u8;
        }
    }

    pub(crate) fn read(&self) -> u8 {
        let mut result = 0xC0 | self.select | 0x0F;

        if self.select & 0x20 == 0 {
            result &= 0xF0 | self.buttons;
        }

        if self.select & 0x10 == 0 {
            result &= 0xF0 | self.directions;
        }

        result
    }

    pub(crate) fn write(&mut self, value: u8) {
        self.select = value & 0x30;
    }

    pub(crate) fn poll_interrupt(&mut self) -> u8 {
        let mut irq = 0;

        if (self.select & 0x20) == 0 && (self.prev_buttons & !self.buttons) != 0 {
            irq |= 0x10;
        }

        if (self.select & 0x10) == 0 && (self.prev_directions & !self.directions) != 0 {
            irq |= 0x10;
        }

        self.prev_buttons = self.buttons;
        self.prev_directions = self.directions;

        irq
    }
}

#[repr(u8)]
pub enum JoypadButton {
    RightOrA = 0x01,
    LeftOrB = 0x02,
    UpOrSelect = 0x04,
    DownOrStart = 0x08,
}