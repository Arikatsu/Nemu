use crate::NemuError;

mod no_mbc;
mod mbc1;

pub(crate) enum MbcType {
    NoMbc(no_mbc::NoMbc),
    Mbc1(mbc1::Mbc1),
}

impl Default for MbcType {
    fn default() -> Self {
        Self::NoMbc(no_mbc::NoMbc::new(vec![0; 0x8000]))
    }
}

impl MbcType {
    pub(crate) fn new(data: Vec<u8>) -> Result<Self, NemuError> {
        let mbc_type = data[0x147];

        match mbc_type {
            0x00 => Ok(Self::NoMbc(no_mbc::NoMbc::new(data))),
            0x01 | 0x02 => Ok(Self::Mbc1(mbc1::Mbc1::new(data))),

            _ => Err(NemuError::InvalidRom(format!(
                "Unsupported MBC type: {:#04X}",
                mbc_type
            ))),
        }
    }
    
    #[inline(always)]
    pub(crate) fn read(&self, addr: u16) -> u8 {
        match self {
            MbcType::NoMbc(mbc) => mbc.read(addr),
            MbcType::Mbc1(mbc) => mbc.read(addr),
        }
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match self {
            MbcType::NoMbc(_) => {},
            MbcType::Mbc1(mbc) => mbc.write(addr, value),
        }
    }
}