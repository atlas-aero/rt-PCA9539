//! Dummy I2C bus for examples
use core::convert::Infallible;
use embedded_hal::i2c::{ErrorType, I2c, Operation, SevenBitAddress};

#[derive(Default)]
pub struct DummyI2CBus {
    /// Command byte of last write operation
    previous_register: u8,
}

impl ErrorType for DummyI2CBus {
    type Error = Infallible;
}

impl I2c<SevenBitAddress> for DummyI2CBus {
    fn transaction(
        &mut self,
        _address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(data) => {
                    match self.previous_register {
                        0x00 => data[0] = 0b0010_0110,
                        0x01 => data[0] = 0b1110_0101,
                        _ => {}
                    };
                }
                Operation::Write(data) => {
                    self.previous_register = data[0];
                }
            }
        }

        Ok(())
    }
}
