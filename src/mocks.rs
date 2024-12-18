use embedded_hal::i2c::{Error, ErrorKind, ErrorType, I2c, Operation, SevenBitAddress};
use mockall::mock;

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum DummyError {
    ReadError,
    WriteError,
}

mock! {
    #[derive(Debug)]
    pub I2CBus{}

    impl I2c<SevenBitAddress> for I2CBus {
        fn transaction<'a>(&mut self, address: SevenBitAddress, operations: &mut [Operation<'a>]) -> Result<(), DummyError>;
    }
}

impl ErrorType for MockI2CBus {
    type Error = DummyError;
}

impl Error for DummyError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
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
    pub fn mock_transaction(mut self, times: usize) -> Self {
        self.bus.expect_transaction().times(times).returning(move |_, _| Ok(()));
        self
    }

    pub fn expect_write(mut self, times: usize, data: &[u8]) -> Self {
        let data_vec = data.to_vec();

        self.bus
            .expect_transaction()
            .times(times)
            .returning(move |address, operations| {
                assert_eq!(1, operations.len());
                assert_eq!(0x74, address);

                match operations[0] {
                    Operation::Read(_) => panic!("Expected write operation"),
                    Operation::Write(buffer) => {
                        assert_eq!(data_vec.len(), buffer.len());
                        assert_eq!(data_vec.as_slice(), buffer);
                    }
                }

                Ok(())
            });

        self
    }

    pub fn expect_read(mut self, times: usize, data: u8) -> Self {
        self.bus
            .expect_transaction()
            .times(times)
            .returning(move |address, operations| {
                assert_eq!(1, operations.len());
                assert_eq!(0x74, address);

                match &mut operations[0] {
                    Operation::Read(buffer) => {
                        assert_eq!(1, buffer.len());
                        buffer[0] = data;
                    }
                    Operation::Write(_) => panic!("Expected read operation"),
                }

                Ok(())
            });

        self
    }

    pub fn write_error(mut self, command: u8) -> Self {
        self.bus.expect_transaction().times(1).returning(move |address, operations| {
            assert_eq!(0x74, address);

            match operations[0] {
                Operation::Read(_) => panic!("Expected write operation"),
                Operation::Write(buffer) => {
                    assert_eq!(command, buffer[0]);
                }
            }

            Err(DummyError::WriteError)
        });

        self
    }

    pub fn read_error(mut self) -> Self {
        self.bus.expect_transaction().times(1).returning(move |address, operations| {
            assert_eq!(0x74, address);
            if let Operation::Write(_) = operations[0] {
                panic!("Expected read operation");
            }

            Err(DummyError::ReadError)
        });

        self
    }

    pub fn into_mock(self) -> MockI2CBus {
        self.bus
    }
}
