//! Digital I/O.

use core::{convert::From, ops::Not};
pub use embedded_hal::digital::{Error, ErrorKind, ErrorType};

#[cfg(feature = "defmt")]
use crate::defmt;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Polarity {
    /// Normal polarity (On is high voltage level, Off is low/zero volts).
    Normal = 1,
    /// Inverted polarity (On is low/zero volts, Off is high voltage level).
    Inverted = 0,
}

impl From<bool> for Polarity {
    #[inline]
    fn from(value: bool) -> Self {
        match value {
            false => Polarity::Inverted,
            true => Polarity::Normal,
        }
    }
}

impl Not for Polarity {
    type Output = Polarity;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Polarity::Normal => Polarity::Inverted,
            Polarity::Inverted => Polarity::Normal,
        }
    }
}

impl From<Polarity> for bool {
    #[inline]
    fn from(value: Polarity) -> bool {
        match value {
            Polarity::Inverted => false,
            Polarity::Normal => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DriveMode {
    /// Push-pull output mode, drives pin to High or Low level
    PushPull,
    /// Open drain/open collector output mode. High impedance when not driven, sinks current from line when driven. Effectively inverted (ON drives low).
    OpenDrain,
    /// Open source/open emmitter output mode. High impedance when not driven, sources current from pin when drien. Non-inverted (ON drives high).
    OpenSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(i8)]
pub enum Bias {
    /// Internal pull-up resistor to MCU positive voltage rail enabled
    PullUp = 1,
    /// Internal pull-down resistor to MCU zero voltage/ground rail enabled
    PullDown = -1,
    /// Input allowed to float
    Floating = 0,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PinMode {
    Input,
    Output,
    //Analog,
}

/// Configurable generic GPIO pin
/// Implement one or both of the ConfigurableInput and ConfigurableOutput sub-traits for each hardware GPIO pin
/// Capabilities can be checked at runtime as well
pub trait Configurable: ErrorType {
    fn get_capabilites() -> [PinMode];

    fn set_polarity(&mut self, polarity: Polarity) -> &mut Self;
    
    fn set_bias(&mut self, direction: Bias) -> &mut Self;
}

/// Pin can be configured as an input
pub trait ConfigurableInput: Configurable {
    fn into_input(&mut self) -> &mut Self;
}

/// Pin can be configured as an output
pub trait ConfigurableOutput: Configurable {

    fn into_output(&mut self) -> &mut Self;

    fn set_drive_mode(&mut self, mode: DriveMode) -> &mut Self;
}