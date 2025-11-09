#[derive(Debug, Copy, Clone)]
pub(super) enum Mode {
    HBlank = 0x00,
    VBlank = 0x01,
    OAMSearch = 0x02,
    PixelTransfer = 0x03,
}

pub(super) const STAT_LYC_EQ_LY: u8 = 0b0000_0100;
pub(super) const STAT_HBLANK_IRQ: u8 = 0b0000_1000;
pub(super) const STAT_VBLANK_IRQ: u8 = 0b0001_0000;
pub(super) const STAT_OAM_IRQ: u8 = 0b0010_0000;
pub(super) const STAT_LYC_IRQ: u8 = 0b0100_0000;