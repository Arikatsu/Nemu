#![allow(dead_code)]

pub struct Registers {
    af: u16, // Accumulator & Flags
    bc: u16, // B & C
    de: u16, // D & E
    hl: u16, // H & L
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
}

impl Registers {
    pub(super) fn new() -> Self {
        Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
    
    pub(super) fn reset(&mut self) {
        self.af = 0x01B0;
        self.bc = 0x0013;
        self.de = 0x00D8;
        self.hl = 0x014D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    #[inline]
    pub fn af(&self) -> u16 {
        self.af
    }

    #[inline]
    pub fn set_af(&mut self, value: u16) {
        self.af = value & 0xFFF0; // Lower nibble of F is always 0
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        self.bc
    }

    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    #[inline]
    pub fn de(&self) -> u16 {
        self.de
    }

    #[inline]
    pub fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        self.hl
    }

    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    #[inline]
    pub fn sp(&self) -> u16 {
        self.sp
    }

    #[inline]
    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    #[inline]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    #[inline]
    pub fn inc_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }

    #[inline]
    pub fn dec_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_sub(value);
    }

    #[inline]
    pub fn a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    #[inline]
    pub fn set_a(&mut self, value: u8) {
        self.af = (self.af & 0x00FF) | ((value as u16) << 8);
    }

    #[inline]
    pub fn f(&self) -> u8 {
        (self.af & 0x00FF) as u8
    }

    #[inline]
    pub fn set_f(&mut self, value: u8) {
        self.af = (self.af & 0xFF00) | ((value as u16) & 0xF0); // Again, lower nibble of F is always 0
    }

    #[inline]
    pub fn b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    #[inline]
    pub fn set_b(&mut self, value: u8) {
        self.bc = (self.bc & 0x00FF) | ((value as u16) << 8);
    }

    #[inline]
    pub fn c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    #[inline]
    pub fn set_c(&mut self, value: u8) {
        self.bc = (self.bc & 0xFF00) | (value as u16);
    }

    #[inline]
    pub fn d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    #[inline]
    pub fn set_d(&mut self, value: u8) {
        self.de = (self.de & 0x00FF) | ((value as u16) << 8);
    }

    #[inline]
    pub fn e(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    #[inline]
    pub fn set_e(&mut self, value: u8) {
        self.de = (self.de & 0xFF00) | (value as u16);
    }

    #[inline]
    pub fn h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    #[inline]
    pub fn set_h(&mut self, value: u8) {
        self.hl = (self.hl & 0x00FF) | ((value as u16) << 8);
    }

    #[inline]
    pub fn l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    #[inline]
    pub fn set_l(&mut self, value: u8) {
        self.hl = (self.hl & 0xFF00) | (value as u16);
    }

    // FLAGS

    #[inline]
    pub fn zero_flag(&self) -> bool {
        (self.f() & 0x80) != 0
    }

    #[inline]
    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x80);
        } else {
            self.set_f(self.f() & !0x80);
        }
    }

    #[inline]
    pub fn subtract_flag(&self) -> bool {
        (self.f() & 0x40) != 0
    }

    #[inline]
    pub fn set_subtract_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x40);
        } else {
            self.set_f(self.f() & !0x40);
        }
    }

    #[inline]
    pub fn half_carry_flag(&self) -> bool {
        (self.f() & 0x20) != 0
    }

    #[inline]
    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x20);
        } else {
            self.set_f(self.f() & !0x20);
        }
    }

    #[inline]
    pub fn carry_flag(&self) -> bool {
        (self.f() & 0x10) != 0
    }

    #[inline]
    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x10);
        } else {
            self.set_f(self.f() & !0x10);
        }
    }

    pub fn get_snapshot(&self) -> String {
        format!(
            "A: {:02X} F: {:02X}\nB: {:02X} C: {:02X}\nD: {:02X} E: {:02X}\nH: {:02X} L: {:02X}\nSP: {:04X}\nPC: {:04X}",
            self.a(), self.f(), self.b(), self.c(), self.d(), self.e(), self.h(), self.l(), self.sp, self.pc
        )
    }
}