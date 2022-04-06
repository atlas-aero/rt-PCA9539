use crate::guard::LockFreeGuard;
use crate::pins::Pins;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use bitmaps::Bitmap;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};

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
#[derive(PartialEq)]
pub enum Mode {
    Output,
    Input,
}

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

    /// Returns a container for fetching individual pins
    pub fn pins(&mut self) -> Pins<B, LockFreeGuard<B>> {
        Pins::new(LockFreeGuard::new(RefCell::new(self)))
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
    pub fn set_state(&mut self, bank: Bank, id: PinID, is_high: bool) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.output_0.set(id as usize, is_high),
            Bank::Bank1 => self.output_1.set(id as usize, is_high),
        };
        self.write_output(bank)
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
        self.write_output(bank)
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
    fn write_output(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
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
