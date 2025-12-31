pub(crate) struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: bool,

    rom_offset: usize,
    ram_offset: usize,
}

impl Mbc1 {
    pub(crate) fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x8000],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: false,
            rom_offset: 0,
            ram_offset: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        // will make these unchecked after running a bunch of tests and roms
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],

            0x4000..=0x7FFF => {
                let index = self.rom_offset.wrapping_add(addr as usize);
                self.rom[index]
            }

            0xA000..=0xBFFF => {
                if !self.ram_enabled { return 0xFF; }
                let index = self.ram_offset.wrapping_add(addr as usize);
                self.ram[index]
            }

            _ => 0xFF,
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }

            0x2000..=0x3FFF => {
                let mut bank = value & 0x1F;
                bank |= (bank == 0) as u8;
                self.rom_bank = (self.rom_bank & 0x60) | bank;
                self.update_offsets();
            }

            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
                self.update_offsets();
            }

            0x6000..=0x7FFF => {
                self.banking_mode = (value & 0x01) != 0;
                self.update_offsets();
            }

            _ => {}
        }
    }

    fn update_offsets(&mut self) {
        let ram_bank_or_upper = if self.banking_mode { self.ram_bank } else { 0 };
        let rom_bank = (ram_bank_or_upper << 5) | (self.rom_bank & 0x1F);
        self.rom_offset = (rom_bank as usize * 0x4000).wrapping_sub(0x4000);
        self.ram_offset = (ram_bank_or_upper as usize * 0x2000).wrapping_sub(0xA000);
    }
}

