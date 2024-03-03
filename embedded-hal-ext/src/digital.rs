//! Digital I/O.
use core::{
    convert::{From, Infallible},
    ops::Not,
};
pub use embedded_hal::digital::{Error, ErrorKind, ErrorType};

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Digital GPIO pin polarity.
///
/// Conversion from `bool` and logical negation are also implemented
/// for this type.
/// ```rust
/// # use embedded_hal_ext::digital::Polarity;
/// let polarity = Polarity::from(true);
/// assert_eq!(polarity, Polarity::Normal);
/// assert_eq!(!polarity, Polarity::Inverted);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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

/// GPIO Output drive mode.
///
/// This represents a typical set of output modes
/// Implementations are free to define additional ones
///     e.g. different drive strengths, sometimes used to shorten rise/fall time and enable faster switching and higher frequencies
/// TODO: Add compile-time checks for this and other similar features, since not every platform has e.g. open emitter capabilites on any or all pins
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
#[non_exhaustive]
pub enum DriveMode {
    /// Push-pull output mode, drives pin to High or Low level
    PushPull,
    /// Open drain/open collector output mode. High impedance when not driven, sinks current from line when driven. Effectively inverted (ON drives low).
    OpenDrain,
    /// Open source/open emmitter output mode. High impedance when not driven, sources current from pin when drien. Non-inverted (ON drives high).
    OpenSource,
}

/// GPIO Pin bias.
///
/// This represents a typical set of possible bias resistor configurations
/// Implementations are free to define additional ones
///     e.g. different resistances, simultaneous pull-up and pull-down
/// Lower resistances (stronger biasing) should have a larger absolute value than the default, and vice versa, to aid comparisons
/// TODO: Add compile-time checks for this and other similar features, since not every platform has e.g. internal pulldowns that are configurable
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(i8)]
#[non_exhaustive]
pub enum Bias {
    /// Internal pull-up resistor to MCU positive voltage rail enabled
    PullUp = 3,
    /// Internal pull-down resistor to MCU zero voltage/ground rail enabled
    PullDown = -3,
    /// Input allowed to float
    /// Should be the default
    Floating = 0,
}

/// GPIO Pin IO mode.
///
/// This represents a typical set of possible bias resistor configurations
/// Implementations are free to define additional ones
///     e.g. different resistances, simultaneous pull-up and pull-down
/// TODO: Add compile-time checks for this and other similar features, since not every platform has e.g. internal pulldowns that are configurable
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
#[non_exhaustive]
pub enum PinMode {
    /// Input-only mode
    Input,
    /// Output-only mode
    Output,
    /// Bi-directional mode
    /// Implements `embedded-hal` `Input` and `Output` traits regardless of mode
    IO,
    /// Supports interrupts/events
    /// Can be implemented on all pins with software implementation where needed, or restricted to specific pins that support it in hardware
    /// Platforms should aim to provide per-pin interrupt granulatity even if not directly supported in hardware
    ///     (e.g. EXTI pin mode on STM32, which often has one interrupt for multiple pins)
    Events,
    //Analog,
}

/// GPIO Pin events.
///
/// This represents a typical set of possible GPIO event triggers.
/// Implementations are free to define additional ones.
/// If hardware triggers for some of the default set are not present on a platform (commonly High and Low levels are not, only edge triggers), they should be implemented in software.
/// A suggested method would be checking the pin state following the receipt of an appropriate edge trigger, to verify that it has remained at the specified level.
/// This can run into issues on platforms without strong timing guarantees. Document these limitations in HAL implementations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
#[non_exhaustive]
pub enum PinEvent {
    /// High logic level
    High,
    /// Low logic level
    Low,
    /// Rising edge ( `_¦¯` )
    RisingEdge,
    /// Falling edge ( `¯¦_` )
    FallingEdge,
    /// Any edge ( `¯¦_` or `_¦¯` )
    AnyEdge,
}

/// Runtime identifier for a GPIO pin. Platform-specific, but should provide both u16 and string representations that uniquely identify the pin.
/// A convenient `id()` implementation for e.g. STM32 or Atmel MCUs could be as follows:
/// ```rust
/// # use embedded_hal_ext::digital::*;
/// # use heapless::String;
///
/// # struct PortPin {
/// #     port: char,
/// #     pin: u8,
/// # }
/// impl PinID for PortPin {
///     fn id(&self) -> u16 {
///         (((self.port as u8) as u16) << 8 | (self.pin as u16)) as u16
///     }
///
/// #    fn name(&self) -> heapless::String<8> {
/// #        let mut s: String<8> = String::new();
/// #        s.push(self.port);
/// #        s.push(char::from_digit(self.pin as u32, 10).unwrap());
/// #        s
/// #    }
/// }
/// ```
/// This would encode pin 17 on port A as `0x4111` i.e. "A" in utf-8 encoding, followed by the number 17.
/// This is suggested instead of the pin mask and port mask because this identifier is intended for
pub trait PinID {
    /// A unique identifier for the pin, either the canonical pin number if pins are only identified numerically, or some encoding of the pin id if not.
    fn id(&self) -> u16;

    /// The standard human-readable name, similar to what's printed on a dev board silkscreen
    fn name(&self) -> heapless::String<8>;
}

/// Configurable generic GPIO pin
/// Implement one or both of the ConfigurableInput and ConfigurableOutput sub-traits for each hardware GPIO pin
/// Capabilities can be checked at runtime as well
pub trait Configurable: ErrorType {
    /// Returns a list of the pin's supported modes
    fn capabilities(self: &Self) -> &[PinMode];

    /// Returns the GPIO Pin ID, usually a number, but platforms differ in their canonical representations.
    /// See [PinID] trait documentation for details.
    fn pin(self: &Self) -> impl PinID;

    /// Returns currently active IO mode
    fn mode(&self) -> PinMode;

    /// Sets pin polarity, Normal or Inverted
    fn set_polarity(self: &mut Self, polarity: Polarity) -> Result<Polarity, Self::Error>;

    /// Sets the bias of the pin, enabling or disabling internal pull-up or pull-down resistors
    /// TODO: Add compile-time checks for this and other similar features, since not every platform has e.g. internal pulldowns that are configurable
    fn set_bias(self: &mut Self, direction: Bias) -> Result<Bias, Self::Error>;
}

impl<T: Configurable + ?Sized> Configurable for &mut T {
    #[inline]
    fn capabilities(self: &Self) -> &[PinMode] {
        T::capabilities(self)
    }

    #[inline]
    fn pin(self: &Self) -> impl PinID {
        T::pin(self)
    }

    #[inline]
    fn mode(self: &Self) -> PinMode {
        T::mode(self)
    }

    #[inline]
    fn set_polarity(self: &mut Self, polarity: Polarity) -> Result<Polarity, Self::Error> {
        T::set_polarity(self, polarity)
    }

    #[inline]
    fn set_bias(self: &mut Self, direction: Bias) -> Result<Bias, Self::Error> {
        T::set_bias(self, direction)
    }
}

/// GPIO pin that can be configured as an input
pub trait ConfigurableInput: Configurable + embedded_hal::digital::InputPin {
    /// Converts pin into input mode
    fn into_input(self: &mut Self) -> Result<(), Self::Error>;
}

impl<T: ConfigurableInput + ?Sized> ConfigurableInput for &mut T {
    #[inline]
    fn into_input(self: &mut Self) -> Result<(), Self::Error> {
        T::into_input(self)
    }
}

/// GPIO Pin can be listened to for events in a non-blocking manner
/// If `async` feature is enabled, pins implementing this trait should also implement [`embedded_hal_async::digital::Wait`]
pub trait Event: ConfigurableInput {
    /// Listen for events
    /// Default options. Platform should provide a sane (i.e. power-efficient, reponsive) default that this calls.
    /// Enforces no simultaneous event listening
    fn listen(self: &mut Self, event: PinEvent) {
        self.stop_listening();
        self.listen_for(event)
    }

    /// Listen for events
    /// Typical implementation is interrupt-based
    /// Platforms should generally only allow listening for one event at a time on a pin via trait functions.
    fn listen_for(self: &mut Self, event: PinEvent);

    /// Stop listening for events. Clears interrupt status flag or similar.
    fn stop_listening(self: &mut Self);

    /// Check if pin is already listening for an event
    fn is_listening(&self) -> bool;

    /// Polls if the listened event has occurred. Clears interrupt status flag or similar if it has.
    fn has_event(&self) -> Option<PinEvent>;

    /// Gets the latest event. Returns `nb::Error::WouldBlock``
    fn get_event(&mut self) -> nb::Result<PinEvent, Infallible>;

    #[cfg(any(feature = "async", doc))]
    // Base function for `embedded-hal-async` "Wait" trait implementation
    async fn wait_for(self: &mut Self, event: PinEvent) -> Result<PinEvent, Self::Error>;
}

impl<T: Event + ?Sized> Event for &mut T {
    #[inline]
    fn listen_for(self: &mut Self, event: PinEvent) {
        T::listen_for(self, event)
    }

    #[inline]
    fn stop_listening(self: &mut Self) {
        T::stop_listening(self)
    }

    #[inline]
    fn is_listening(&self) -> bool {
        T::is_listening(self)
    }

    #[inline]
    fn has_event(&self) -> Option<PinEvent> {
        T::has_event(self)
    }

    #[inline]
    fn get_event(&mut self) -> nb::Result<PinEvent, Infallible> {
        T::get_event(self)
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn wait_for(self: &mut Self, event: PinEvent) -> Result<PinEvent, Self::Error> {
        T::wait_for(self, event).await
    }
}

/// GPIO Pin can be configured as an output
pub trait ConfigurableOutput: Configurable + embedded_hal::digital::OutputPin {
    /// Converts pin into output mode
    fn into_output(self: &mut Self) -> Result<(), Self::Error>;

    /// Sets drive mode of pin
    /// Should also set pin polarity to Inverted
    /// Platform implementation should return an error if a specific mode is unsupported on a specific pin.
    /// Cross-platform crates should perform runtime checks currently
    /// TODO: Add compile-time checks for this and other similar features
    fn set_drive_mode(self: &mut Self, mode: DriveMode) -> Result<DriveMode, Self::Error>;
}

impl<T: ConfigurableOutput + ?Sized> ConfigurableOutput for &mut T {
    #[inline]
    fn into_output(self: &mut Self) -> Result<(), Self::Error> {
        T::into_output(self)
    }

    #[inline]
    fn set_drive_mode(self: &mut Self, mode: DriveMode) -> Result<DriveMode, Self::Error> {
        T::set_drive_mode(self, mode)
    }
}

/// Configurable GPIO Pin that implements both Input and Output traits
pub trait ConfigurableIO: ConfigurableInput + ConfigurableOutput {}
