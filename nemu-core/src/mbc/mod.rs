mod no_mbc;

pub(crate) enum MbcType {
    NoMbc(no_mbc::NoMbc),
}

impl Default for MbcType {
    fn default() -> Self {
        Self::NoMbc(no_mbc::NoMbc::new(vec![0; 0x8000]))
    }
}

impl MbcType {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        let mbc_type = data[0x147];

        match mbc_type {
            0x00 => Self::NoMbc(no_mbc::NoMbc::new(data)),
            _ => panic!("Unsupported MBC type: {:02X}", mbc_type),
        }
    }
    
    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match self {
            MbcType::NoMbc(mbc) => mbc.read(addr),
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match self {
            MbcType::NoMbc(_) => {}
        }
    }
}