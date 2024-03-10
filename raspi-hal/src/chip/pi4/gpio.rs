#![allow(dead_code)]
use std::convert::AsRef;
use std::str::FromStr;
use embedded_hal::digital::PinState;
use strum::{AsRefStr, EnumString};
use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum::{EnumCount as EnumCountMacro, EnumIter, FromRepr};
use embedded_hal_ext::digital::{Bias, Polarity, PinMode, PinID, PinEvent, DriveMode};



/// All pins in the Raspberry Pi 5 RP1 chip
#[derive(Debug, Clone, EnumCountMacro, EnumIter, FromRepr, AsRefStr, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PinNames {
    ID_SDA = 0,
    ID_SCL = 1,
    SDA = 2,
    SCL = 3,
    GPCLK0 = 4,
    GP5 = 5,
    GP6 = 6,
    CE1 = 7,
    CE0 = 8,
    MISO = 9,
    MOSI = 10,
    SCLK = 11,
    PWM0 = 12,
    PWM1 = 13,
    TXD = 14,
    RXD = 15,
    GP16 = 16,
    GP17 = 17,
    PCM_CLK = 18,
    PCM_FS = 19,
    PCM_DIN = 20,
    PCM_DOUT = 21,
    GP22 = 22,
    GP23 = 23,
    GP24 = 24,
    GP25 = 25,
    GP26 = 26,
    GP27 = 27,
    PCIE_RP1_WAKE = 28,
    FAN_TACH = 29,
    HOST_SDA = 30,
    HOST_SCL = 31,
    ETH_RST_N = 32, // used
    L33 = 33,
    CD0_IO0_MICCLK = 34, // used
    CD0_IO1_MICDAT0 = 35,
    RP1_PCIE_CLKREQ_N = 36,
    L37 = 37,
    CD0_SDA = 38,
    CD0_SCL = 39,
    CD1_SDA = 40,
    CD1_SCL = 41,
    USB_VBUS_EN = 42,
    USB_OC_N = 43,
    RP1_STAT_LED = 44,
    FAN_PWM = 45,
    CD1_IO0_MICCLK = 46, // used
    WAKE_2712 = 47,
    CD1_IO1_MICDAT1 = 48,
    EN_MAX_USB_CURRENT = 49,
    L50 = 50,
    L51 = 51,
    L52 = 52,
    L53 = 53,
}

impl PinID for PinNames {

    fn id(&self) -> u16 {
        *self as u16
    }
    
    fn name(&self) -> heapless::String<16> {
        heapless::String::from_str(self.as_ref()).unwrap()
    }
}

