#[derive(PartialEq, Eq)]
pub(crate) enum InterruptMode {
    Enabled,
    Disabled,
    Pending,
}

macro_rules! reg_cb {
    ($code:expr) => {
        {
            match $code & 0x07 {
                0 => Some(Reg8::B),
                1 => Some(Reg8::C),
                2 => Some(Reg8::D),
                3 => Some(Reg8::E),
                4 => Some(Reg8::H),
                5 => Some(Reg8::L),
                6 => None, // (HL)
                7 => Some(Reg8::A),
                _ => unreachable!(),
            }
        }
    };
}

pub(in super) use reg_cb;