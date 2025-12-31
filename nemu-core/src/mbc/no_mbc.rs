pub(crate) struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub(crate) fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        unsafe { *self.rom.get_unchecked(addr as usize) }
    }
}