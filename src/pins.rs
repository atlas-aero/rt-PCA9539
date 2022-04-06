use crate::expander::{Bank, Mode, PinID};
use crate::guard::RefGuard;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Read, Write};

/// Container for fetching individual pins
pub struct Pins<B: Write + Read, R: RefGuard<B>> {
    guard: R,
    bus: PhantomData<fn(B) -> B>,
}

impl<B: Write + Read, R: RefGuard<B>> Pins<B, R> {
    pub fn new(guard: R) -> Self {
        Self {
            guard,
            bus: PhantomData,
        }
    }

    /// Returns an individual pin, which state gets updated synchronously
    /// **The library does not prevent multiple parallel instances of the same pin.**
    pub fn get_pin(&self, bank: Bank, id: PinID) -> Pin<B, R, Input, RegularAccessMode> {
        Pin::regular(&self.guard, bank, id)
    }

    /// Returns an individual pin, which is using a cached state
    /// The status is explicitly updated. This allows a more efficient status query and assignment,
    /// as the status is only updated once for all pins.
    /// **The library does not prevent multiple parallel instances of the same pin.**
    pub fn get_refreshable_pin(&self, bank: Bank, id: PinID) -> Pin<B, R, Input, RefreshMode> {
        Pin::refreshable(&self.guard, bank, id)
    }
}

/// Marker trait defining how the state of pins is handled. Currently there are two modes supported:
/// * Regular: State of the pin is synchronously fetched from I2C bus when calling functions like `is_high()`
/// * Refreshable: State of all pins is refreshed explicitly and functions like `is_high()` are working on a cached state.
/// This reducing the I2C overhead
pub trait AccessMode {}

/// State of the pin is synchronously fetched from I2C bus
pub struct RegularAccessMode {}
impl AccessMode for RegularAccessMode {}

/// Working on cached register state. State of all pins is refreshed explicitly.
pub struct RefreshMode {}
impl AccessMode for RefreshMode {}

/// Current I/O mode. Either Input or Output.
pub trait PinMode {}

/// Input mode
pub struct Input {}
impl PinMode for Input {}

/// Output mode
pub struct Output {}
impl PinMode for Output {}

/// Individual GPIO pin
pub struct Pin<'a, B, R, M, A>
where
    B: Write + Read,
    R: RefGuard<B>,
    M: PinMode,
    A: AccessMode,
{
    pub(crate) expander: &'a R,
    pub(crate) bank: Bank,
    pub(crate) id: PinID,

    pub(crate) bus: PhantomData<fn(B) -> B>,
    pub(crate) mode: PhantomData<M>,
    pub(crate) access_mode: PhantomData<A>,
}

impl<'a, B, R, A> Pin<'a, B, R, Input, A>
where
    B: Write + Read,
    R: RefGuard<B>,
    A: AccessMode,
{
    /// Reverses/Resets the input polarity
    pub fn invert_polarity(&self, invert: bool) -> Result<(), <B as Write>::Error> {
        let mut result = Ok(());

        self.expander.access(|expander| {
            result = expander.reverse_polarity(self.bank, self.id, invert);
        });

        result
    }
}

impl<'a, B, R, A> Pin<'a, B, R, Output, A>
where
    B: Write + Read,
    R: RefGuard<B>,
    A: AccessMode,
{
    /// Returns the current output state, this logic is independent from access mode, as it acts in both
    /// cases on cached register state
    pub(crate) fn is_pin_output_high(&self) -> bool {
        let mut is_high = false;
        self.expander
            .access(|expander| is_high = expander.is_pin_output_high(self.bank, self.id));

        is_high
    }
}

impl<'a, B, M, R, A> Pin<'a, B, R, M, A>
where
    B: Write + Read,
    R: RefGuard<B>,
    M: PinMode,
    A: AccessMode,
{
    /// Switches the pin to the given mode
    pub(crate) fn change_mode(&self, mode: Mode) -> Result<(), <B as Write>::Error> {
        let mut result = Ok(());

        self.expander.access(|expander| {
            result = expander.set_mode(self.bank, self.id, mode);
        });

        result
    }
}
