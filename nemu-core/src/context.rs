use crate::memory::Memory;

pub(crate) struct NemuContext<'a> {
    pub memory: &'a mut Memory,
}

impl<'a> NemuContext<'a> {
    #[inline(always)]
    pub(crate) fn tick(&mut self, ticks: u8) {
        let _ = ticks;
    }

    #[inline(always)]
    pub(crate) fn mem_read(&mut self, addr: u16) -> u8 {
        self.tick(1);
        self.memory.read(addr)
    }

    #[inline(always)]
    pub(crate) fn mem_write(&mut self, addr: u16, data: u8) {
        self.tick(1);
        self.memory.write(addr, data);
    }

    #[inline(always)]
    pub(crate) fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let low = self.mem_read(addr) as u16;
        let high = self.mem_read(addr + 1) as u16;
        (high << 8) | low
    }

    #[inline(always)]
    pub(crate) fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }
}