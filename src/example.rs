//! Dummy I2C bus for examples
use core::convert::Infallible;
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};

pub struct DummyI2CBus {}

impl DummyI2CBus {
    pub fn new() -> Self {
        DummyI2CBus {}
    }
}

impl Write for DummyI2CBus {
    type Error = Infallible;

    fn write(&mut self, _address: SevenBitAddress, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Read for DummyI2CBus {
    type Error = Infallible;

    fn read(&mut self, address: SevenBitAddress, buffer: &mut [u8]) -> Result<(), Self::Error> {
        match address {
            0x00 => buffer[0] = 0b0010_0110,
            0x01 => buffer[0] = 0b1110_0101,
            _ => {}
        };

        Ok(())
    }
}
