pub(crate) struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: bool,

    rom_offset: usize,
    bank0_offset: usize,
    ram_offset: usize,
    rom_mask: u8,
}

impl Mbc1 {
    pub(crate) fn new(rom: Vec<u8>) -> Self {
        let num_banks = rom.len() / 0x4000;
        let rom_mask = (num_banks.next_power_of_two() - 1) as u8;

        let mut mbc = Self {
            rom,
            ram: vec![0; 0x8000],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: false,
            rom_offset: 0,
            bank0_offset: 0,
            ram_offset: 0,
            rom_mask,
        };

        mbc.update_offsets();
        mbc
    }

    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let index = self.bank0_offset + addr as usize;
                self.rom[index]
            }

            0x4000..=0x7FFF => {
                let index = self.rom_offset + (addr as usize - 0x4000);
                self.rom[index]
            }

            0xA000..=0xBFFF => {
                if !self.ram_enabled { return 0xFF; }
                let index = self.ram_offset + (addr as usize - 0xA000);
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
                let bank = value & 0x1F;
                self.rom_bank = if bank == 0 { 1 } else { bank };
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

            0xA000..=0xBFFF => {
                if !self.ram_enabled { return; }
                let index = self.ram_offset + (addr as usize - 0xA000);
                self.ram[index] = value;
            }

            _ => {}
        }
    }

    fn update_offsets(&mut self) {
        let rom_bank = if self.banking_mode {
            ((self.ram_bank << 5) | self.rom_bank) & self.rom_mask
        } else {
            self.rom_bank & self.rom_mask
        } as usize;

        let bank0 = if self.banking_mode {
            ((self.ram_bank << 5) & self.rom_mask) as usize
        } else {
            0
        };

        self.rom_offset = rom_bank * 0x4000;
        self.bank0_offset = bank0 * 0x4000;

        let ram_bank = if self.banking_mode {
            self.ram_bank as usize
        } else {
            0
        };
        self.ram_offset = ram_bank * 0x2000;
    }
}