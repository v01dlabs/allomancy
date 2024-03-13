
pub use self::implementation::*;
//pub use self::implementation::gpio::*;
//pub use self::implementation::gpiomem::*;
use std::convert::AsRef;
use std::str::FromStr;
use std::fmt;
use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum::{EnumCount as EnumCountMacro, EnumIter, FromRepr, AsRefStr, EnumString};
use embedded_hal_ext::digital::{Bias, Polarity, PinID, PinEvent, DriveMode};

#[cfg_attr(feature = "pi5", path = "pi5/mod.rs")]
#[cfg_attr(feature = "pi4", path = "pi4/mod.rs")]
#[cfg_attr(feature = "pi3", path = "pi4/mod.rs")]
#[cfg_attr(feature = "pi_zero", path = "pi_zero/mod.rs")]
mod implementation;

pub(crate)mod ioctl;
/// Broadcom GPIO numbers for the header pins
#[derive(Debug, Clone, EnumCountMacro, EnumIter, FromRepr, AsRefStr, Copy)]
#[repr(u8)]
pub enum BCMHeader {
    GP0 = 0,
    GP1 = 1,
    GP2 = 2,
    GP3 = 3,
    GP4 = 4,
    GP5 = 5,
    GP6 = 6,
    GP7 = 7,
    GP8 = 8,
    GP9 = 9,
    GP10 = 10,
    GP11 = 11,
    GP12 = 12,
    GP13 = 13,
    GP14 = 14,
    GP15 = 15,
    GP16 = 16,
    GP17 = 17,
    GP18 = 18,
    GP19 = 19,
    GP20 = 20,
    GP21 = 21,
    GP22 = 22,
    GP23 = 23,
    GP24 = 24,
    GP25 = 25,
    GP26 = 26,
    GP27 = 27,
}

impl PinID for BCMHeader {

    fn id(&self) -> u16 {
        *self as u16
    }
    
    fn name(&self) -> heapless::String<16> {
        heapless::String::from_str(self.as_ref()).unwrap()
    }
}
/// 40-pin header physical pin numbers
#[derive(Debug, Clone, EnumCountMacro, EnumIter, FromRepr, AsRefStr, Copy)]
#[repr(u8)]
pub enum PiHeader {
    P27 = 0,
    P28 = 1,
    P3 = 2,
    P5 = 3,
    P7 = 4,
    P29 = 5,
    P31 = 6,
    P26 = 7,
    P24 = 8,
    P21 = 9,
    P19 = 10,
    P23 = 11,
    P32 = 12,
    P33 = 13,
    P8  = 14,
    P10 = 15,
    P36 = 16,
    P11 = 17,
    P12 = 18,
    P35 = 19,
    P38 = 20,
    P40 = 21,
    P15 = 22,
    P16 = 23,
    P18 = 24,
    P22 = 25,
    P37 = 26,
    P13 = 27,
}

impl PinID for PiHeader {

    fn id(&self) -> u16 {
        *self as u16
    }
    
    fn name(&self) -> heapless::String<16> {
        heapless::String::from_str(self.as_ref()).unwrap()
    }
}


/// Pin modes.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum PinMode {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
    Alt6,
    Alt7,
    Alt8,
}

impl fmt::Display for PinMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PinMode::Input => write!(f, "In"),
            PinMode::Output => write!(f, "Out"),
            PinMode::Alt0 => write!(f, "Alt0"),
            PinMode::Alt1 => write!(f, "Alt1"),
            PinMode::Alt2 => write!(f, "Alt2"),
            PinMode::Alt3 => write!(f, "Alt3"),
            PinMode::Alt4 => write!(f, "Alt4"),
            PinMode::Alt5 => write!(f, "Alt5"),
            PinMode::Alt6 => write!(f, "Alt6"),
            PinMode::Alt7 => write!(f, "Alt7"),
            PinMode::Alt8 => write!(f, "Alt8"),
        }
    }
}


