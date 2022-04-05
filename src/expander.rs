use embedded_hal::blocking::i2c::{SevenBitAddress, Write};
use embedded_hal::serial::Read;

/// GPIO bank. PCA9539 has two with 7 pins each
pub enum Bank {
    Bank0,
    Bank1,
}

/// GPIO pin ID. Builds together with bank an unique pin identification.
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

pub struct PCA9539<B>
where
    B: Write<SevenBitAddress> + Read<SevenBitAddress>,
{
    bus: B,

    /// First input register
    #[allow(unused)]
    pub(crate) input_0: u8,
    /// Second input register
    #[allow(unused)]
    pub(crate) input_1: u8,

    /// First output register
    #[allow(unused)]
    pub(crate) output_0: u8,
    /// Second output register
    #[allow(unused)]
    pub(crate) output_1: u8,

    /// First polarity inversion register
    #[allow(unused)]
    pub(crate) polarity_0: u8,
    /// Second polarity inversion register
    #[allow(unused)]
    pub(crate) polarity_1: u8,

    /// First configuration register
    #[allow(unused)]
    pub(crate) configuration_0: u8,
    /// Second configuration register
    #[allow(unused)]
    pub(crate) configuration_1: u8,
}

const COMMAND_OUTPUT_0: u8 = 0x02;
const COMMAND_OUTPUT_1: u8 = 0x03;

const COMMAND_CONF_0: u8 = 0x06;
const COMMAND_CONF_1: u8 = 0x07;

impl<B> PCA9539<B>
where
    B: Write<SevenBitAddress> + Read<SevenBitAddress>,
{
    pub fn new(bus: B) -> Self {
        PCA9539 {
            bus,
            input_0: 0x0,
            input_1: 0x0,
            output_0: 0xff,
            output_1: 0xff,
            polarity_0: 0x0,
            polarity_1: 0x0,
            configuration_0: 0xff,
            configuration_1: 0xff,
        }
    }

    /// Switches the given pin to output mode by adjusting the configuration register
    pub fn output_mode(&mut self, bank: Bank, id: PinID) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_conf0(self.configuration_0 & !(1 << id as u8)),
            Bank::Bank1 => self.write_conf1(self.configuration_1 & !(1 << id as u8)),
        }
    }

    /// Switches all pins of the given bank to output mode1
    pub fn all_output(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_conf0(0x0),
            Bank::Bank1 => self.write_conf1(0x0),
        }
    }

    /// Switches the given pin to input mode by adjusting the configuration register
    pub fn input_mode(&mut self, bank: Bank, id: PinID) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_conf0(self.configuration_0 | 1 << id as u8),
            Bank::Bank1 => self.write_conf1(self.configuration_1 | 1 << id as u8),
        }
    }

    /// Switches all pins of the given bank to input mode
    pub fn all_input(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_conf0(0xff),
            Bank::Bank1 => self.write_conf1(0xff),
        }
    }

    /// Sets the given pin to HIGH
    /// The given pins needs to be in output note, otherwise the change has not electrical effect
    pub fn set_high(&mut self, bank: Bank, id: PinID) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_output0(self.output_0 | 1 << id as u8),
            Bank::Bank1 => self.write_output1(self.output_1 | 1 << id as u8),
        }
    }

    /// Sets the given pin to LOW
    /// The given pins needs to be in output note, otherwise the change has not electrical effect
    pub fn set_low(&mut self, bank: Bank, id: PinID) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_output0(self.output_0 & !(1 << id as u8)),
            Bank::Bank1 => self.write_output1(self.output_1 & !(1 << id as u8)),
        }
    }

    /// Sets all pins of the given bank to high state
    pub fn set_all_high(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_output0(0xff),
            Bank::Bank1 => self.write_output1(0xff),
        }
    }

    /// Sets all pins of the given bank to high state
    pub fn set_all_low(&mut self, bank: Bank) -> Result<(), <B as Write>::Error> {
        match bank {
            Bank::Bank0 => self.write_output0(0x0),
            Bank::Bank1 => self.write_output1(0x0),
        }
    }

    /// Writes the first output state byte
    fn write_output0(&mut self, conf: u8) -> Result<(), <B as Write>::Error> {
        self.output_0 = conf;
        self.bus.write(COMMAND_OUTPUT_0, &[self.output_0])
    }

    /// Writes the second output state byte
    fn write_output1(&mut self, conf: u8) -> Result<(), <B as Write>::Error> {
        self.output_1 = conf;
        self.bus.write(COMMAND_OUTPUT_1, &[self.output_1])
    }

    /// Writes the first configuration byte
    fn write_conf0(&mut self, conf: u8) -> Result<(), <B as Write>::Error> {
        self.configuration_0 = conf;
        self.bus.write(COMMAND_CONF_0, &[self.configuration_0])
    }

    /// Writes the second configuration byte
    fn write_conf1(&mut self, conf: u8) -> Result<(), <B as Write>::Error> {
        self.configuration_1 = conf;
        self.bus.write(COMMAND_CONF_1, &[self.configuration_1])
    }
}
