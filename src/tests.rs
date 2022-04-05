use crate::expander::Bank::{Bank0, Bank1};
use crate::expander::Mode::{Input, Output};
use crate::expander::PinID::{Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7};
use crate::expander::PCA9539;
use crate::mocks::{MockI2CBus, ReadError, WriteError};
use alloc::string::ToString;

#[test]
fn test_expander_output_mode_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_0111, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_0110, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode(Bank0, Pin3, Output).unwrap();
    expander.set_mode(Bank0, Pin0, Output).unwrap();
}

#[test]
fn test_expander_output_mode_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1011_1111, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0011_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode(Bank1, Pin6, Output).unwrap();
    expander.set_mode(Bank1, Pin7, Output).unwrap();
}

#[test]
fn test_expander_input_mode_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0100, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1000_0100, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank0, Output).unwrap();
    expander.set_mode(Bank0, Pin2, Input).unwrap();
    expander.set_mode(Bank0, Pin7, Input).unwrap();
}

#[test]
fn test_expander_input_mode_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0001, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_1001, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank1, Output).unwrap();
    expander.set_mode(Bank1, Pin0, Input).unwrap();
    expander.set_mode(Bank1, Pin3, Input).unwrap();
}

#[test]
fn test_expander_state_low_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1101, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1110_1101, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state(Bank0, Pin1, false).unwrap();
    expander.set_state(Bank0, Pin4, false).unwrap();
}

#[test]
fn test_expander_state_low_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1011, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1001, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state(Bank1, Pin2, false).unwrap();
    expander.set_state(Bank1, Pin1, false).unwrap();
}

#[test]
fn test_expander_state_high_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0010_0000, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0010_0001, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank0, false).unwrap();
    expander.set_state(Bank0, Pin5, true).unwrap();
    expander.set_state(Bank0, Pin0, true).unwrap();
}

#[test]
fn test_expander_state_high_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0100_0000, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0101_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank1, false).unwrap();
    expander.set_state(Bank1, Pin6, true).unwrap();
    expander.set_state(Bank1, Pin4, true).unwrap();
}

#[test]
fn test_set_mode_all_input_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank0, Output).unwrap();
    expander.set_mode_all(Bank0, Input).unwrap();
}

#[test]
fn test_set_mode_all_output_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank0, Output).unwrap();
}

#[test]
fn test_set_mode_all_input_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank1, Output).unwrap();
    expander.set_mode_all(Bank1, Input).unwrap();
}

#[test]
fn test_set_mode_all_output_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_mode_all(Bank1, Output).unwrap();
}

#[test]
fn test_set_state_all_low_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank0, true).unwrap();
    expander.set_state_all(Bank0, false).unwrap();
}

#[test]
fn test_set_state_all_low_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank1, true).unwrap();
    expander.set_state_all(Bank1, false).unwrap();
}

#[test]
fn test_set_state_all_high_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x02, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank0, true).unwrap();
}

#[test]
fn test_set_state_all_high_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x03, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.set_state_all(Bank1, true).unwrap();
}

#[test]
fn test_reverse_polarity_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x04, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0100, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x04, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0001_0100, data[0]);
        Ok(())
    });

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x04, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0001_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.reverse_polarity(Bank0, Pin2, true).unwrap();
    expander.reverse_polarity(Bank0, Pin4, true).unwrap();
    expander.reverse_polarity(Bank0, Pin2, false).unwrap();
}

#[test]
fn test_refresh_input_state_bank0_success() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x00, address);
        Ok(())
    });

    i2c_bus.expect_read().times(1).returning(move |address, buffer| {
        assert_eq!(0x00, address);
        assert_eq!(1, buffer.len());
        buffer[0] = 0b0001_0000;

        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.refresh_input_state(Bank0).unwrap();
}

#[test]
fn test_refresh_input_state_bank1_success() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x01, address);
        Ok(())
    });

    i2c_bus.expect_read().times(1).returning(move |address, buffer| {
        assert_eq!(0x01, address);
        assert_eq!(1, buffer.len());
        buffer[0] = 0b0001_0000;

        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.refresh_input_state(Bank1).unwrap();
}

#[test]
fn test_refresh_input_state_write_error() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x00, address);
        Err(WriteError::Error1)
    });

    let mut expander = PCA9539::new(i2c_bus);
    let result = expander.refresh_input_state(Bank0);

    assert_eq!("WriteError", result.unwrap_err().to_string());
}

#[test]
fn test_refresh_input_state_read_error() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x00, address);
        Ok(())
    });

    i2c_bus.expect_read().times(1).returning(move |address, _| {
        assert_eq!(0x00, address);
        Err(ReadError::Error1)
    });

    let mut expander = PCA9539::new(i2c_bus);
    let result = expander.refresh_input_state(Bank0);

    assert_eq!("ReadError", result.unwrap_err().to_string());
}

#[test]
fn test_is_pin_high_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x00, address);
        Ok(())
    });

    i2c_bus.expect_read().times(1).returning(move |address, buffer| {
        assert_eq!(0x00, address);
        assert_eq!(1, buffer.len());
        buffer[0] = 0b0111_1010;

        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.refresh_input_state(Bank0).unwrap();

    assert!(!expander.is_pin_high(Bank0, Pin7));
    assert!(expander.is_pin_high(Bank0, Pin6));
    assert!(expander.is_pin_high(Bank0, Pin5));
    assert!(expander.is_pin_high(Bank0, Pin4));

    assert!(expander.is_pin_high(Bank0, Pin3));
    assert!(!expander.is_pin_high(Bank0, Pin2));
    assert!(expander.is_pin_high(Bank0, Pin1));
    assert!(!expander.is_pin_high(Bank0, Pin0));
}

#[test]
fn test_is_pin_high_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, _| {
        assert_eq!(0x01, address);
        Ok(())
    });

    i2c_bus.expect_read().times(1).returning(move |address, buffer| {
        assert_eq!(0x01, address);
        assert_eq!(1, buffer.len());
        buffer[0] = 0b0100_0111;

        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.refresh_input_state(Bank1).unwrap();

    assert!(!expander.is_pin_high(Bank1, Pin7));
    assert!(expander.is_pin_high(Bank1, Pin6));
    assert!(!expander.is_pin_high(Bank1, Pin5));
    assert!(!expander.is_pin_high(Bank1, Pin4));

    assert!(!expander.is_pin_high(Bank1, Pin3));
    assert!(expander.is_pin_high(Bank1, Pin2));
    assert!(expander.is_pin_high(Bank1, Pin1));
    assert!(expander.is_pin_high(Bank1, Pin0));
}
