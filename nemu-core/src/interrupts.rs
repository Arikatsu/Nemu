#![allow(dead_code)]

pub(crate) const INT_VBLANK: u8 = 0b0000_0001;
pub(crate) const INT_LCDSTAT: u8 = 0b0000_0010;
pub(crate) const INT_TIMER: u8 = 0b0000_0100;
pub(crate) const INT_SERIAL: u8 = 0b0000_1000;
pub(crate) const INT_JOYPAD: u8 = 0b0001_0000;

// in the future I will refactor this for all the interrupt stuff like an interrupt controller (not sure when)