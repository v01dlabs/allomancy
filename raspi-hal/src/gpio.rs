use core::{convert::Infallible, marker::PhantomData};
use std::error;
use std::fmt;
use std::io;
use std::mem::MaybeUninit;
use std::ops::Not;
use std::os::unix::io::AsRawFd;
use std::result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once, Weak};
use std::time::Duration;


use embedded_hal_ext::digital::ConfigurablePin;
use embedded_hal_ext::digital::{Bias, DriveMode, PinID, Polarity, PinMode};
use thiserror::Error;

use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum::{EnumCount as EnumCountMacro, EnumIter};


use crate::chip;
use chip::ioctl;


pub struct AnyPin {
    line: ioctl::LineV2,
}