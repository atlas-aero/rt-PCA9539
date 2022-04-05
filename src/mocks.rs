use embedded_hal::blocking::i2c::{SevenBitAddress, Write};
use embedded_hal::serial::Read;
use mockall::mock;
use nb;

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum WriteError {
    Error1,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ReadError {
    Error1,
}

mock! {
    #[derive(Debug)]
    pub I2CBus{}

    impl Write<SevenBitAddress> for I2CBus {
        type Error = WriteError;
        fn write(&mut self, address: SevenBitAddress, bytes: &[u8]) -> Result<(), WriteError>;
    }

    impl Read<u8> for I2CBus {
        type Error = ReadError;
        fn read(&mut self) -> nb::Result<u8, ReadError>;
    }
}
