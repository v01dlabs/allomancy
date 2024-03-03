//! Digital I/O.

use core::{convert::From, ops::Not};
pub use embedded_hal::digital::{Error, ErrorKind, ErrorType};

#[cfg(feature = "defmt-03")]
use crate::defmt;

pub trait Configurable: ErrorType {

}