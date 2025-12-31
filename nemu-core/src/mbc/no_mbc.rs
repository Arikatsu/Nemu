pub(crate) struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub(crate) fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => unsafe { *self.rom.get_unchecked(addr as usize) },
            _ => 0xFF,
        }
    }
}