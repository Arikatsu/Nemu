pub struct Registers {
    a: u8,
    f: u8,      // Flags
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,    // Stack Pointer
    pc: u16,    // Program Counter
}

#[derive(Clone, Copy)]
pub enum Reg8 {
    A,
    // F not needed
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    // I use separate methods for SP and PC. Why? Made it that way and lazy to change now.
}

impl Registers {
    pub(super) fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
    
    pub(super) fn reset(&mut self) {
        self.a = 0x01;
        self.f = 0xB0;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    #[inline(always)]
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    #[inline(always)]
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8 & 0xF0; // Lower nibble of F is always 0
    }

    #[inline(always)]
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    #[inline(always)]
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    #[inline(always)]
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    #[inline(always)]
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    #[inline(always)]
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    #[inline(always)]
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    #[inline(always)]
    pub fn a(&self) -> u8 {
        self.a
    }

    #[inline(always)]
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    #[inline(always)]
    pub fn f(&self) -> u8 {
        self.f
    }

    #[inline(always)]
    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0;
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.b
    }

    #[inline(always)]
    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    #[inline(always)]
    pub fn c(&self) -> u8 {
        self.c
    }

    #[inline(always)]
    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    #[inline(always)]
    pub fn d(&self) -> u8 {
        self.d
    }

    #[inline(always)]
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    #[inline(always)]
    pub fn e(&self) -> u8 {
        self.e
    }

    #[inline(always)]
    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    #[inline(always)]
    pub fn h(&self) -> u8 {
        self.h
    }

    #[inline(always)]
    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    #[inline(always)]
    pub fn l(&self) -> u8 {
        self.l
    }

    #[inline(always)]
    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    // FLAGS

    #[inline(always)]
    pub fn zero_flag(&self) -> bool {
        (self.f() & 0x80) != 0
    }

    #[inline(always)]
    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x80);
        } else {
            self.set_f(self.f() & !0x80);
        }
    }

    #[inline(always)]
    pub fn subtract_flag(&self) -> bool {
        (self.f() & 0x40) != 0
    }

    #[inline(always)]
    pub fn set_subtract_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x40);
        } else {
            self.set_f(self.f() & !0x40);
        }
    }

    #[inline(always)]
    pub fn half_carry_flag(&self) -> bool {
        (self.f() & 0x20) != 0
    }

    #[inline(always)]
    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x20);
        } else {
            self.set_f(self.f() & !0x20);
        }
    }

    #[inline(always)]
    pub fn carry_flag(&self) -> bool {
        (self.f() & 0x10) != 0
    }

    #[inline(always)]
    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | 0x10);
        } else {
            self.set_f(self.f() & !0x10);
        }
    }
    
    pub fn read_reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a(),
            Reg8::B => self.b(),
            Reg8::C => self.c(),
            Reg8::D => self.d(),
            Reg8::E => self.e(),
            Reg8::H => self.h(),
            Reg8::L => self.l(),
        }
    }
    
    pub fn write_reg8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.set_a(value),
            Reg8::B => self.set_b(value),
            Reg8::C => self.set_c(value),
            Reg8::D => self.set_d(value),
            Reg8::E => self.set_e(value),
            Reg8::H => self.set_h(value),
            Reg8::L => self.set_l(value),
        }
    }
    
    pub fn read_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => self.af(),
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
        }
    }
    
    pub fn write_reg16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => self.set_af(value),
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
        }
    }

    pub fn get_snapshot(&self) -> String {
        format!(
            "A: {:02X} F: {:02X}\nB: {:02X} C: {:02X}\nD: {:02X} E: {:02X}\nH: {:02X} L: {:02X}",
            self.a(), self.f(), self.b(), self.c(), self.d(), self.e(), self.h(), self.l(),
        )
    }

    #[inline(always)]
    pub fn sp(&self) -> u16 {
        self.sp
    }

    #[inline(always)]
    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    #[inline(always)]
    pub fn inc_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_add(value);
    }

    #[inline(always)]
    pub fn dec_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(value);
    }

    #[inline(always)]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline(always)]
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    #[inline(always)]
    pub fn inc_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }
}