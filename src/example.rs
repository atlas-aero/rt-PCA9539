//! Dummy I2C bus for examples
use core::convert::Infallible;
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};

#[derive(Default)]
pub struct DummyI2CBus {
    /// Command byte of last write operation
    previous_register: u8,
}

impl Write for DummyI2CBus {
    type Error = Infallible;

    fn write(&mut self, _address: SevenBitAddress, _bytes: &[u8]) -> Result<(), Self::Error> {
        self.previous_register = _bytes[0];
        Ok(())
    }
}

impl Read for DummyI2CBus {
    type Error = Infallible;

    fn read(&mut self, _address: SevenBitAddress, buffer: &mut [u8]) -> Result<(), Self::Error> {
        match self.previous_register {
            0x00 => buffer[0] = 0b0010_0110,
            0x01 => buffer[0] = 0b1110_0101,
            _ => {}
        };

        Ok(())
    }
}
