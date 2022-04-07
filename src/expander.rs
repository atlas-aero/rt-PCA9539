//! # Abstraction of PCA9539
//!
//! Central part of this crate is the struct [PCA9539], which either allows central I/O control or
//! or alternatively offers a breakdown into individual pins.
//!
//! The following examples demonstrates central I/O control. For getting separate pin instances,
//! see the [pins module](crate::pins).
//!
//! ## Setup
//! [PCA9539] instance is created using a I2CBus implementing the I2C traits of
//! [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/blocking/i2c/index.html).
//! ```
//! use pca9539::example::DummyI2CBus;
//! use pca9539::expander::PCA9539;
//!
//! let i2c_bus = DummyI2CBus::new();
//! let expander = PCA9539::new(i2c_bus);
//! ```
//! ## Changing mode
//! ```
//!# use pca9539::example::DummyI2CBus;
//!# use pca9539::expander::Bank::{Bank0, Bank1};
//!# use pca9539::expander::Mode::{Input, Output};
//!# use pca9539::expander::PCA9539;
//!# use pca9539::expander::PinID::{Pin2, Pin4};
//!#
//!# let i2c_bus = DummyI2CBus::new();
//!# let mut  expander = PCA9539::new(i2c_bus);
//!#
//! // Switch Pin02 to input mode
//! expander.set_mode(Bank0, Pin2, Input).unwrap();
//!
//! // Switch Pin14 to output mode
//! expander.set_mode(Bank1, Pin4, Output).unwrap();
//! ```
//! ## Reading input state
//! ```
//!# use pca9539::example::DummyI2CBus;
//!# use pca9539::expander::Bank::Bank0;
//!# use pca9539::expander::PCA9539;
//!# use pca9539::expander::PinID::Pin1;
//!#
//!# let i2c_bus = DummyI2CBus::new();
//!# let mut  expander = PCA9539::new(i2c_bus);
//!#
//! expander.refresh_input_state(Bank0).unwrap();
//! let is_high = expander.is_pin_input_high(Bank0, Pin1);
//!
//! assert!(is_high);
//! ```
//! ## Setting output state
//! ```
//!# use pca9539::example::DummyI2CBus;
//!# use pca9539::expander::Bank::Bank0;
//!# use pca9539::expander::Mode::Output;
//!# use pca9539::expander::PCA9539;
//!# use pca9539::expander::PinID::Pin1;
//!#
//!# let i2c_bus = DummyI2CBus::new();
//!# let mut  expander = PCA9539::new(i2c_bus);
//!#
//! expander.set_mode(Bank0, Pin1, Output);
//!
//! expander.set_state(Bank0, Pin1, true);
//! expander.write_output_state(Bank0).unwrap();
//!
//! let is_high = expander.is_pin_output_high(Bank0, Pin1);
//! assert!(is_high);
//! ```
//! ## Invert input polarity
//! PCA9539 has built-in hardware support for inverting input state. See [datasheet](<https://www.ti.com/lit/ds/symlink/pca9539.pdf?ts=1649342250975>)
//! for more details.
//! ```
//!# use pca9539::example::DummyI2CBus;
//!# use pca9539::expander::Bank::Bank0;
//!# use pca9539::expander::PCA9539;
//!# use pca9539::expander::PinID::{Pin1, Pin3};
//!#
//!# let i2c_bus = DummyI2CBus::new();
//!# let mut  expander = PCA9539::new(i2c_bus);
//!#
//! expander.reverse_polarity(Bank0, Pin3, true).unwrap();
//! ```

#[cfg(feature = "cortex-m")]
use crate::guard::CsMutexGuard;
use crate::guard::LockFreeGuard;
#[cfg(feature = "spin")]
use crate::guard::SpinGuard;
use crate::pins::Pins;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use bitmaps::Bitmap;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
#[cfg(feature = "cortex-m")]
use cortex_m::interrupt::Mutex as CsMutex;
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};
#[cfg(feature = "spin")]
use spin::Mutex as SpinMutex;

/// GPIO bank. PCA9539 has two with 7 pins each
#[derive(Copy, Clone)]
pub enum Bank {
    Bank0,
    Bank1,
}

/// GPIO pin ID. Builds together with bank an unique pin identification.
#[derive(Copy, Clone)]
pub enum PinID {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
}

/// GPIO mode
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    Output,
    Input,
}

/// Abstraction of [PCA9539](<https://www.ti.com/lit/ds/symlink/pca9539.pdf?ts=1649342250975>) I/O expander
pub struct PCA9539<B>
where
    B: Write<SevenBitAddress> + Read<SevenBitAddress>,
{
    bus: B,

    /// First input register
    #[allow(unused)]
    input_0: Bitmap<8>,
    /// Second input register
    #[allow(unused)]
    input_1: Bitmap<8>,

    /// First output register
    output_0: Bitmap<8>,
    /// Second output register
    output_1: Bitmap<8>,

    /// First polarity inversion register
    #[allow(unused)]
    polarity_0: Bitmap<8>,
    /// Second polarity inversion register
    #[allow(unused)]
    polarity_1: Bitmap<8>,

    /// First configuration register
    configuration_0: Bitmap<8>,
    /// Second configuration register
    configuration_1: Bitmap<8>,
}

/// Wrapped I2C error when refreshing input state
/// Reading input state consists of one write, followed by a read operation
pub enum RefreshInputError<B: Write + Read<u8>> {
    WriteError(<B as Write>::Error),
    ReadError(<B as Read>::Error),
}

const COMMAND_INPUT_0: u8 = 0x00;
const COMMAND_INPUT_1: u8 = 0x01;

const COMMAND_OUTPUT_0: u8 = 0x02;
const COMMAND_OUTPUT_1: u8 = 0x03;

const COMMAND_POLARITY_0: u8 = 0x04;
const COMMAND_POLARITY_1: u8 = 0x05;

const COMMAND_CONF_0: u8 = 0x06;
const COMMAND_CONF_1: u8 = 0x07;

impl<B> PCA9539<B>
where
    B: Write<SevenBitAddress> + Read<SevenBitAddress>,
{
    pub fn new(bus: B) -> Self {
        let mut expander = Self {
            bus,
            input_0: Bitmap::<8>::new(),
            input_1: Bitmap::<8>::new(),
            output_0: Bitmap::<8>::new(),
            output_1: Bitmap::<8>::new(),
            polarity_0: Bitmap::<8>::new(),
            polarity_1: Bitmap::<8>::new(),
            configuration_0: Bitmap::<8>::new(),
            configuration_1: Bitmap::<8>::new(),
        };

        expander.output_0.invert();
        expander.output_1.invert();
        expander.configuration_0.invert();
        expander.configuration_1.invert();

        expander
    }

    /// Returns a pins container without using any locks
    /// This is the most efficient way of using individual pins
    /// The downside is, that these pins are neither Send or Sync, so can only be used in single-threaded
    /// and interrupt-free applications
    pub fn pins(&mut self) -> Pins<B, LockFreeGuard<B>> {
        Pins::new(LockFreeGuard::new(RefCell::new(self)))
    }

    /// Returns a pins container using Mutex based on critical sections
    /// Individual pins can be used across threads and interrupts, as long just running on a single core
    #[cfg(feature = "cortex-m")]
    pub fn pins_cs_mutex(&mut self) -> Pins<B, CsMutexGuard<B>> {
        Pins::new(CsMutexGuard::new(CsMutex::new(RefCell::new(self))))
    }

    /// Returns a pins container using a spin mutex
    /// This is safe to use across threads and on multi-core applications
    /// However, this requires a system supporting spin mutexes, which are generally only
    /// available on systems with Atomic CAS
    #[cfg(feature = "spin")]
    pub fn pins_spin_mutex(&mut self) -> Pins<B, SpinGuard<B>> {
        Pins::new(SpinGuard::new(SpinMutex::new(RefCell::new(self))))
    }

    /// Switches the given pin to the input/output mode by adjusting the configuration register
    pub fn set_mode(&mut self, bank: Bank, id: PinID, mode: Mode) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.configuration_0.set(id as usize, mode.into()),
            Bank::Bank1 => self.configuration_1.set(id as usize, mode.into()),
        };
        self.write_conf(bank)
    }

    /// Switches all pins of the given bank to output/input mode1
    pub fn set_mode_all(&mut self, bank: Bank, mode: Mode) -> Result<(), <B as Write>::Error> {
        let mut bitset = Bitmap::<8>::new();

        if mode == Mode::Input {
            bitset.invert();
        }

        match bank {
            Bank::Bank0 => self.configuration_0 = bitset,
            Bank::Bank1 => self.configuration_1 = bitset,
        };
        self.write_conf(bank)
    }

    /// Sets the given output state by adjusting the output register
    /// Pin needs to be in OUTPUT mode for correct electrical state
    /// Note: This just updates the internal register, to make the changes effective,
    /// an additional call to `write_output_state()` is needed.
    pub fn set_state(&mut self, bank: Bank, id: PinID, is_high: bool) {
        match bank {
            Bank::Bank0 => self.output_0.set(id as usize, is_high),
            Bank::Bank1 => self.output_1.set(id as usize, is_high),
        };
    }

    /// Sets output state for all pins of a bank
    pub fn set_state_all(&mut self, bank: Bank, is_high: bool) -> Result<(), <B as Write>::Error> {
        let mut bitset = Bitmap::<8>::new();

        if is_high {
            bitset.invert();
        }

        match bank {
            Bank::Bank0 => self.output_0 = bitset,
            Bank::Bank1 => self.output_1 = bitset,
        };
        self.write_output_state(bank)
    }

    /// Reveres/Resets the input polarity of the given pin
    pub fn reverse_polarity(&mut self, bank: Bank, id: PinID, reversed: bool) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.polarity_0.set(id as usize, reversed),
            Bank::Bank1 => self.polarity_1.set(id as usize, reversed),
        };
        self.write_polarity(bank)
    }

    /// Refreshes the input state of the given bank
    pub fn refresh_input_state(&mut self, bank: Bank) -> Result<(), RefreshInputError<B>> {
        match bank {
            Bank::Bank0 => self.input_0 = Bitmap::from_value(self.read_input_register(COMMAND_INPUT_0)?),
            Bank::Bank1 => self.input_1 = Bitmap::from_value(self.read_input_register(COMMAND_INPUT_1)?),
        };

        Ok(())
    }

    /// Returns true if the given pin input is high
    /// Pin needs to be in INPUT mode
    /// This method is using the cached register, for a updated result `refresh_input_state()` needs
    /// to be called beforehand
    pub fn is_pin_input_high(&self, bank: Bank, id: PinID) -> bool {
        match bank {
            Bank::Bank0 => self.input_0.get(id as usize),
            Bank::Bank1 => self.input_1.get(id as usize),
        }
    }

    /// Returns true if the pins output state is set high
    pub fn is_pin_output_high(&self, bank: Bank, id: PinID) -> bool {
        match bank {
            Bank::Bank0 => self.output_0.get(id as usize),
            Bank::Bank1 => self.output_1.get(id as usize),
        }
    }

    /// Reads and returns the given input register
    fn read_input_register(&mut self, command: u8) -> Result<u8, RefreshInputError<B>> {
        let result = self.bus.write(command, &[0x0]);
        if result.is_err() {
            return Err(RefreshInputError::WriteError(result.unwrap_err()));
        }

        let mut buffer: [u8; 1] = [0x0; 1];
        let result = self.bus.read(command, &mut buffer);

        if result.is_err() {
            return Err(RefreshInputError::ReadError(result.unwrap_err()));
        }

        Ok(buffer[0])
    }

    /// Writes the configuration register of the given bank
    fn write_conf(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.bus.write(COMMAND_CONF_0, &[self.configuration_0.as_value().to_owned()]),
            Bank::Bank1 => self.bus.write(COMMAND_CONF_1, &[self.configuration_1.as_value().to_owned()]),
        }
    }

    /// Writes the output register of the given bank
    pub fn write_output_state(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.bus.write(COMMAND_OUTPUT_0, &[self.output_0.as_value().to_owned()]),
            Bank::Bank1 => self.bus.write(COMMAND_OUTPUT_1, &[self.output_1.as_value().to_owned()]),
        }
    }

    /// Writes the polarity register of the given bank
    fn write_polarity(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.bus.write(COMMAND_POLARITY_0, &[self.polarity_0.as_value().to_owned()]),
            Bank::Bank1 => self.bus.write(COMMAND_POLARITY_1, &[self.polarity_1.as_value().to_owned()]),
        }
    }
}

impl From<Mode> for bool {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Output => false,
            Mode::Input => true,
        }
    }
}

impl<B: Read<u8> + Write> Debug for RefreshInputError<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            RefreshInputError::WriteError(_) => f.write_str("RefreshInputError::WriteError"),
            RefreshInputError::ReadError(_) => f.write_str("RefreshInputError::ReadError"),
        }
    }
}

impl<B: Read<u8> + Write> ToString for RefreshInputError<B> {
    fn to_string(&self) -> String {
        match self {
            RefreshInputError::WriteError(_) => "WriteError".to_string(),
            RefreshInputError::ReadError(_) => "ReadError".to_string(),
        }
    }
}
