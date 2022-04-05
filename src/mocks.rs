use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};
use mockall::mock;

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

    impl Read<SevenBitAddress> for I2CBus {
        type Error = ReadError;
        fn read(&mut self, address: SevenBitAddress, buffer: &mut [u8]) -> Result<(), ReadError>;
    }
}

pub struct BusMockBuilder {
    bus: MockI2CBus,
}

impl BusMockBuilder {
    pub fn new() -> Self {
        Self { bus: MockI2CBus::new() }
    }

    /// Expect the given number of write calls without any assertions
    pub fn mock_write(mut self, times: usize) -> Self {
        self.bus.expect_write().times(times).returning(move |_, _| Ok(()));
        self
    }

    pub fn expect_write(mut self, times: usize, address: u8, data: u8) -> Self {
        self.bus
            .expect_write()
            .times(times)
            .returning(move |actual_address, actual_data| {
                assert_eq!(address, actual_address);
                assert_eq!(1, actual_data.len());
                assert_eq!(data, actual_data[0]);
                Ok(())
            });

        self
    }

    pub fn expect_read(mut self, times: usize, address: u8, data: u8) -> Self {
        self.bus.expect_read().times(times).returning(move |actual_address, buffer| {
            assert_eq!(address, actual_address);
            assert_eq!(1, buffer.len());
            buffer[0] = data;

            Ok(())
        });

        self
    }

    pub fn write_error(mut self, address: u8) -> Self {
        self.bus.expect_write().times(1).returning(move |actual_address, _| {
            assert_eq!(actual_address, address);
            Err(WriteError::Error1)
        });

        self
    }

    pub fn read_error(mut self, address: u8) -> Self {
        self.bus.expect_read().times(1).returning(move |actual_address, _| {
            assert_eq!(actual_address, address);
            Err(ReadError::Error1)
        });

        self
    }

    pub fn into_mock(self) -> MockI2CBus {
        self.bus
    }
}
