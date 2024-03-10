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

use cgpio::LineInfo;
use embedded_hal_ext::digital::ConfigurablePin;
use embedded_hal_ext::digital::{Bias, DriveMode, PinID, Polarity, PinMode};
use thiserror::Error;

use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum::{EnumCount as EnumCountMacro, EnumIter};


use crate::chip;
use crate::cgpio;

#[derive(Error, Debug)]
pub enum GpioError {
    #[error("GPIO chip for this platform not found")]
    ChipNotFound,
    #[error(transparent)]
    CDevError(#[from] cgpio::Error),
    #[error("Compiled for a different platform than the one running")]
    PlatformMismatch,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Permission Denied: {}", .0)]
    PermissionDenied(String),
    
}

impl embedded_hal::digital::Error for GpioError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

pub struct GpioPin<const GPIOID: u8> {
    reset_on_drop: bool,
    mode: PinMode,
    line: cgpio::Line,
    rq: Option<cgpio::LineHandle>,
    state: Option<Arc<GpioState>>,
}

impl<const GPIOID: u8> embedded_hal::digital::ErrorType for GpioPin<GPIOID>
{
    type Error = GpioError;
}

impl<const GPIOID: u8> ConfigurablePin for GpioPin<GPIOID>  {
    
    fn capabilities(self: &Self) -> &[PinMode] {
        &[PinMode::Input, PinMode::Output, PinMode::IOMode, PinMode::Alt(0)]
    }

    fn pin(self: &Self) -> impl PinID {
        chip::PiHeader::from_repr(GPIOID).unwrap()
    }

    fn mode(&self) -> PinMode {
        self.mode
    }

    fn set_polarity(self: &mut Self, polarity: Polarity) -> Result<Polarity, Self::Error> {
        let mut value = 0;
        let mut flags = cgpio::LineRequestFlags::empty();
        if self.rq.is_some() {
            value = self.rq.as_ref().unwrap().get_value()?;
            flags = self.rq.as_ref().unwrap().flags();
        }
        self.rq = if polarity == Polarity::Inverted {
            if value == 0 {
                value = 1;
            } else {
                value = 0;
            }
            Some(self.line.request(cgpio::LineRequestFlags::union(flags, cgpio::LineRequestFlags::ACTIVE_LOW), value, self.pin().name().as_str())?)
        } else {
            Some(self.line.request(flags, value, self.pin().name().as_str())?)
        };
        Ok(polarity)
    }

    fn set_bias(self: &mut Self, direction: Bias) -> Result<Bias, Self::Error> {
        match self.state {
            Some(ref state) => {
                state.gpio_mem.set_bias(GPIOID, direction);
                Ok(direction)
            }
            None => Err(GpioError::PlatformMismatch),
        }
    }
}

impl<const GPIOID: u8> Drop for GpioPin<GPIOID> {
    fn drop(&mut self) {
        match &self.state {
            Some(gpio_state) => gpio_state.pins_taken[GPIOID as usize].store(false, Ordering::SeqCst),
            None => {},
        }
    }
}


// Store Gpio's state separately, so we can conveniently share it through
// a cloned Arc.
#[derive(Debug)]
pub(crate) struct GpioState {
    gpio_mem: Box<dyn GpioRegisters>,
    //cdev: std::fs::File,
    //sync_interrupts: Mutex<interrupt::EventLoop>,
    pins_taken: [AtomicBool; u8::MAX as usize],
    gpio_lines: u8,
}

#[derive(Debug)]
pub struct GpioChip {
    chip: cgpio::Chip,
    state: Option<Arc<GpioState>>,

}

impl GpioChip {
    pub fn get(chip_label: &str) -> Result<Self, GpioError> {
        let chips: cgpio::ChipIterator = cgpio::chips()?;
        let chip = chips
            .filter(|c| {
                c.as_ref().is_ok_and(|chip| (chip.label() == chip_label) || (chip.name() == chip_label))
            })
            .next()
            .ok_or(GpioError::ChipNotFound)??;
        let mut state  = None;
        if chip_label == chip::GPIO_CHIP.as_ref() {
            // Replace this when std::sync::SyncLazy is stabilized. https://github.com/rust-lang/rust/issues/74465

            // Shared state between Gpio and Pin instances. GpioState is dropped after
            // all Gpio and Pin instances go out of scope, guaranteeing we won't have
            // any pins simultaneously using different EventLoop or GpioMem instances.
            static mut GPIO_STATE: MaybeUninit<Mutex<Weak<GpioState>>> = MaybeUninit::uninit();
            static ONCE: Once = Once::new();

            // call_once is thread-safe, guaranteed to be called only once, and memory writes performed
            // by the closure can be observed by other threads after execution completes.
            let mut weak_state = unsafe {
                ONCE.call_once(|| {
                    GPIO_STATE.write(Mutex::new(Weak::new()));
                });

                // GPIO_STATE will always be initialized at this point.
                GPIO_STATE.assume_init_ref().lock().unwrap()
            };
             // Clone a strong reference if a GpioState instance already exists, otherwise
            // initialize it here so we can return any relevant errors.
            if let Some(ref s) = weak_state.upgrade() {
                state = Some(s.clone());
            } else {
                //let device_info = DeviceInfo::new().map_err(|_| Error::UnknownModel)?;

                let gpio_mem: Box<dyn GpioRegisters> = Box::new(chip::gpiomem::GpioMem::open()?);

                //let cdev = ioctl::find_gpiochip()?;
                //let sync_interrupts = Mutex::new(interrupt::EventLoop::new(
                //    cdev.as_raw_fd(),
                //    u8::MAX as usize,
                //)?);
                let pins_taken = init_array!(AtomicBool::new(false), u8::MAX as usize);
                let gpio_lines = chip::BCMHeader::COUNT as u8;

                state = Some(Arc::new(GpioState {
                    gpio_mem,
                    //cdev,
                    //sync_interrupts,
                    pins_taken,
                    gpio_lines,
                }));

                // Store a weak reference to our state. This gets dropped when
                // all Gpio and Pin instances go out of scope.
                *weak_state = Arc::downgrade(state.as_ref().unwrap());
            }
        }
        Ok(Self { chip, state })
    }

    pub fn get_default() -> Result<Self, GpioError> {
        Self::get(chip::GPIO_CHIP.as_ref())
    }

    pub fn pin(&self, pin: impl PinID) {

    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PinProp {
    Mode(PinMode),
    Bias(Bias),
    Drive(DriveMode),
    Polarity(Polarity),
    RstOnDrop(bool),
}

pub trait Signal {}

impl Signal for PinProp {}

#[doc(hidden)]
pub trait PinType {}

#[doc(hidden)]
pub trait IsOutputPin: PinType {}

#[doc(hidden)]
pub trait IsInputPin: PinType {}


#[doc(hidden)]
pub struct InputOutputPinType;

#[doc(hidden)]
pub struct InputOnlyPinType;


impl PinType for InputOutputPinType {}
impl IsOutputPin for InputOutputPinType {}
impl IsInputPin for InputOutputPinType {}

impl PinType for InputOnlyPinType {}
impl IsInputPin for InputOnlyPinType {}

pub trait GpioRegisters: std::fmt::Debug + Sync + Send {
    fn set_high(&self, pin: u8);
    fn set(&self, pin: u8, state: embedded_hal::digital::PinState);
    fn set_low(&self, pin: u8);
    fn level(&self, pin: u8) -> embedded_hal::digital::PinState;
    fn mode(&self, pin: u8) -> chip::PinMode;
    fn set_mode(&self, pin: u8, mode: chip::PinMode);
    fn set_bias(&self, pin: u8, bias: Bias);
}


pub trait GpioProperties {
    type Chip: chip::Soc;
    type Registers: GpioRegisters;
    type Signal: Signal;
    type PinType: PinType;
}


#[doc(hidden)]
#[macro_export]
macro_rules! gpio {
    (
        $(
            ($gpionum:literal, $chip:literal, $type:ident
                $(
                    ( $( $af_input_num:literal => $af_input_signal:ident )* )
                    ( $( $af_output_num:literal => $af_output_signal:ident )* )
                )?
            )
        )+
    ) => {
        
    };
}
