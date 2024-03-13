use std::convert::AsRef;
use std::str::FromStr;
use strum::AsRefStr;
use strum::EnumString;

pub mod gpio;

#[derive(Debug, Clone, EnumString, AsRefStr, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum GPIO {
    #[strum(serialize = "pinctrl-bcm2711")]
    PinCtrl,
    
}

pub const GPIO_CHIP: GPIO = GPIO::PinCtrl;
