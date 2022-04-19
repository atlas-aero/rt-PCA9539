use crate::expander::Bank::{Bank0, Bank1};
use crate::expander::Mode::{Input, Output};
use crate::expander::PinID::{Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7};
use crate::expander::PCA9539;
#[cfg(not(feature = "spin"))]
use crate::guard::LockFreeGuard;
#[cfg(feature = "spin")]
use crate::guard::SpinGuard;
use crate::mocks::{BusMockBuilder, MockI2CBus, WriteError};
use crate::pin_refreshable::{RefreshableInputPin, RefreshableOutputPin};
use crate::pins::Pins;
use alloc::string::ToString;
use embedded_hal::digital::v2::{InputPin, IoPin, OutputPin, PinState, StatefulOutputPin, ToggleableOutputPin};

#[test]
fn test_expander_output_mode_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x06, 0b1111_0111])
        .expect_write(1, &[0x06, 0b1111_0110])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode(Bank0, Pin3, Output).unwrap();
    expander.set_mode(Bank0, Pin0, Output).unwrap();
}

#[test]
fn test_expander_output_mode_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x07, 0b1011_1111])
        .expect_write(1, &[0x07, 0b0011_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode(Bank1, Pin6, Output).unwrap();
    expander.set_mode(Bank1, Pin7, Output).unwrap();
}

#[test]
fn test_expander_input_mode_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x06, 0b0000_0100])
        .expect_write(1, &[0x06, 0b1000_0100])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank0, Output).unwrap();
    expander.set_mode(Bank0, Pin2, Input).unwrap();
    expander.set_mode(Bank0, Pin7, Input).unwrap();
}

#[test]
fn test_expander_input_mode_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x07, 0b0000_0001])
        .expect_write(1, &[0x07, 0b0000_1001])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank1, Output).unwrap();
    expander.set_mode(Bank1, Pin0, Input).unwrap();
    expander.set_mode(Bank1, Pin3, Input).unwrap();
}

#[test]
fn test_expander_state_low_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x02, 0b1111_1101])
        .expect_write(1, &[0x02, 0b1110_1101])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state(Bank0, Pin1, false);
    expander.write_output_state(Bank0).unwrap();
    expander.set_state(Bank0, Pin4, false);
    expander.write_output_state(Bank0).unwrap();
}

#[test]
fn test_expander_state_low_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x03, 0b1111_1011])
        .expect_write(1, &[0x03, 0b1111_1001])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state(Bank1, Pin2, false);
    expander.write_output_state(Bank1).unwrap();
    expander.set_state(Bank1, Pin1, false);
    expander.write_output_state(Bank1).unwrap();
}

#[test]
fn test_expander_state_high_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x02, 0b0010_0000])
        .expect_write(1, &[0x02, 0b0010_0001])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, false).unwrap();
    expander.set_state(Bank0, Pin5, true);
    expander.write_output_state(Bank0).unwrap();
    expander.set_state(Bank0, Pin0, true);
    expander.write_output_state(Bank0).unwrap();
}

#[test]
fn test_expander_state_high_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x03, 0b0100_0000])
        .expect_write(1, &[0x03, 0b0101_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank1, false).unwrap();
    expander.set_state(Bank1, Pin6, true);
    expander.write_output_state(Bank1).unwrap();
    expander.set_state(Bank1, Pin4, true);
    expander.write_output_state(Bank1).unwrap();
}

#[test]
fn test_set_mode_all_input_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x06, 0b1111_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank0, Output).unwrap();
    expander.set_mode_all(Bank0, Input).unwrap();
}

#[test]
fn test_set_mode_all_output_bank0() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x06, 0b0000_0000]).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank0, Output).unwrap();
}

#[test]
fn test_set_mode_all_input_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x07, 0b1111_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank1, Output).unwrap();
    expander.set_mode_all(Bank1, Input).unwrap();
}

#[test]
fn test_set_mode_all_output_bank1() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x07, 0b0000_0000]).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_mode_all(Bank1, Output).unwrap();
}

#[test]
fn test_set_state_all_low_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x02, 0b0000_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, true).unwrap();
    expander.set_state_all(Bank0, false).unwrap();
}

#[test]
fn test_set_state_all_low_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x03, 0b0000_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank1, true).unwrap();
    expander.set_state_all(Bank1, false).unwrap();
}

#[test]
fn test_set_state_all_high_bank0() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x02, 0b1111_1111]).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, true).unwrap();
}

#[test]
fn test_set_state_all_high_bank1() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x03, 0b1111_1111]).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank1, true).unwrap();
}

#[test]
fn test_reverse_polarity_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x04, 0b0000_0100])
        .expect_write(1, &[0x04, 0b0001_0100])
        .expect_write(1, &[0x04, 0b0001_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.reverse_polarity(Bank0, Pin2, true).unwrap();
    expander.reverse_polarity(Bank0, Pin4, true).unwrap();
    expander.reverse_polarity(Bank0, Pin2, false).unwrap();
}

#[test]
fn test_refresh_input_state_bank0_success() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x00])
        .expect_read(1, 0b0001_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.refresh_input_state(Bank0).unwrap();
}

#[test]
fn test_refresh_input_state_bank1_success() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x01])
        .expect_read(1, 0b0001_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.refresh_input_state(Bank1).unwrap();
}

#[test]
fn test_refresh_input_state_write_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x00).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let result = expander.refresh_input_state(Bank0);

    assert_eq!("WriteError", result.unwrap_err().to_string());
}

#[test]
fn test_refresh_input_state_read_error() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x00]).read_error().into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let result = expander.refresh_input_state(Bank0);

    assert_eq!("ReadError", result.unwrap_err().to_string());
}

#[test]
fn test_is_pin_high_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x00])
        .expect_read(1, 0b0111_1010)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.refresh_input_state(Bank0).unwrap();

    assert!(!expander.is_pin_input_high(Bank0, Pin7));
    assert!(expander.is_pin_input_high(Bank0, Pin6));
    assert!(expander.is_pin_input_high(Bank0, Pin5));
    assert!(expander.is_pin_input_high(Bank0, Pin4));

    assert!(expander.is_pin_input_high(Bank0, Pin3));
    assert!(!expander.is_pin_input_high(Bank0, Pin2));
    assert!(expander.is_pin_input_high(Bank0, Pin1));
    assert!(!expander.is_pin_input_high(Bank0, Pin0));
}

#[test]
fn test_is_pin_high_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x01])
        .expect_read(1, 0b0100_0111)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.refresh_input_state(Bank1).unwrap();

    assert!(!expander.is_pin_input_high(Bank1, Pin7));
    assert!(expander.is_pin_input_high(Bank1, Pin6));
    assert!(!expander.is_pin_input_high(Bank1, Pin5));
    assert!(!expander.is_pin_input_high(Bank1, Pin4));

    assert!(!expander.is_pin_input_high(Bank1, Pin3));
    assert!(expander.is_pin_input_high(Bank1, Pin2));
    assert!(expander.is_pin_input_high(Bank1, Pin1));
    assert!(expander.is_pin_input_high(Bank1, Pin0));
}

#[test]
fn test_regular_pin_input_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(4, &[0x00])
        .expect_read(2, 0b0000_0100)
        .expect_read(2, 0b0100_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank0, Pin2);

    assert!(pin.is_high().unwrap());
    assert!(!pin.is_low().unwrap());
    assert!(!pin.is_high().unwrap());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_regular_pin_input_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(4, &[0x01])
        .expect_read(2, 0b0100_0100)
        .expect_read(2, 0b0000_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank1, Pin6);

    assert!(pin.is_high().unwrap());
    assert!(!pin.is_low().unwrap());
    assert!(!pin.is_high().unwrap());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_regular_pin_input_write_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x01).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank1, Pin6);

    assert_eq!("WriteError", pin.is_high().unwrap_err().to_string())
}

#[test]
fn test_regular_pin_input_read_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(1).read_error().into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank1, Pin6);

    assert_eq!("ReadError", pin.is_high().unwrap_err().to_string())
}

#[test]
fn test_refreshable_pin_input_bank0() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(2, &[0x00])
        .expect_read(1, 0b0000_0100)
        .expect_read(1, 0b0100_1000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin02 = pins.get_refreshable_pin(Bank0, Pin2);
    let pin03 = pins.get_refreshable_pin(Bank0, Pin3);

    pin02.refresh_bank().unwrap();
    assert!(pin02.is_high().unwrap());
    assert!(!pin02.is_low().unwrap());
    assert!(!pin03.is_high().unwrap());
    assert!(pin03.is_low().unwrap());

    pin03.refresh_bank().unwrap();
    assert!(!pin02.is_high().unwrap());
    assert!(pin02.is_low().unwrap());
    assert!(pin03.is_high().unwrap());
    assert!(!pin03.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_input_bank1() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(2, &[0x01])
        .expect_read(1, 0b0010_0100)
        .expect_read(1, 0b0000_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin12 = pins.get_refreshable_pin(Bank1, Pin2);
    let pin15 = pins.get_refreshable_pin(Bank1, Pin5);

    pin12.refresh_bank().unwrap();
    assert!(pin12.is_high().unwrap());
    assert!(!pin12.is_low().unwrap());
    assert!(pin15.is_high().unwrap());
    assert!(!pin15.is_low().unwrap());

    pin15.refresh_bank().unwrap();
    assert!(!pin12.is_high().unwrap());
    assert!(pin12.is_low().unwrap());
    assert!(!pin15.is_high().unwrap());
    assert!(pin15.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_input_mixed_banks() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x00])
        .expect_write(1, &[0x01])
        .expect_write(1, &[0x00])
        .expect_write(1, &[0x01])
        .expect_read(1, 0b0001_0001)
        .expect_read(1, 0b1000_0000)
        .expect_read(1, 0b0000_0001)
        .expect_read(1, 0b0000_0000)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin00 = pins.get_refreshable_pin(Bank0, Pin0);
    let pin17 = pins.get_refreshable_pin(Bank1, Pin7);

    pin00.refresh_all().unwrap();
    assert!(pin00.is_high().unwrap());
    assert!(!pin00.is_low().unwrap());
    assert!(pin17.is_high().unwrap());
    assert!(!pin17.is_low().unwrap());

    pin17.refresh_all().unwrap();
    assert!(pin00.is_high().unwrap());
    assert!(!pin00.is_low().unwrap());
    assert!(!pin17.is_high().unwrap());
    assert!(pin17.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_refresh_bank_write_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x0).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin = pins.get_refreshable_pin(Bank0, Pin0);
    let error = pin.refresh_bank().unwrap_err();

    assert_eq!("WriteError", error.to_string());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_refresh_bank_read_error() {
    let i2c_bus = BusMockBuilder::new().expect_write(1, &[0x00]).read_error().into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin = pins.get_refreshable_pin(Bank0, Pin0);
    let error = pin.refresh_bank().unwrap_err();

    assert_eq!("ReadError", error.to_string());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_refresh_all_write_error() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x0])
        .expect_read(1, 0b0001_0000)
        .write_error(0x1)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin = pins.get_refreshable_pin(Bank0, Pin0);
    let error = pin.refresh_all().unwrap_err();

    assert_eq!("WriteError", error.to_string());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_refreshable_pin_refresh_all_read_error() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x0])
        .expect_read(1, 0b0001_0000)
        .expect_write(1, &[0x1])
        .read_error()
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);

    let pin = pins.get_refreshable_pin(Bank0, Pin0);
    let error = pin.refresh_all().unwrap_err();

    assert_eq!("ReadError", error.to_string());
    assert!(pin.is_low().unwrap());
}

#[test]
fn test_regular_pin_set_output_state() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(6) // Mode switch
        .expect_write(1, &[0x03, 0b1111_1011])
        .expect_write(1, &[0x02, 0b1110_1111])
        .expect_write(1, &[0x02, 0b1110_1110])
        .expect_write(1, &[0x02, 0b1111_1110])
        .expect_write(1, &[0x02, 0b1111_1110])
        .expect_write(1, &[0x02, 0b1111_1111])
        .expect_write(1, &[0x03, 0b1111_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin00 = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::High).unwrap();
    let mut pin04 = pins.get_pin(Bank0, Pin4).into_output_pin(PinState::High).unwrap();
    let mut pin12 = pins.get_pin(Bank1, Pin2).into_output_pin(PinState::High).unwrap();

    pin12.set_low().unwrap();
    assert!(pin12.is_set_low().unwrap());
    assert!(!pin12.is_set_high().unwrap());

    pin04.set_low().unwrap();
    assert!(pin04.is_set_low().unwrap());
    assert!(!pin04.is_set_high().unwrap());

    pin00.set_state(PinState::Low).unwrap();
    assert!(pin00.is_set_low().unwrap());
    assert!(!pin00.is_set_high().unwrap());

    pin04.set_state(PinState::High).unwrap();
    assert!(!pin04.is_set_low().unwrap());
    assert!(pin04.is_set_high().unwrap());

    pin04.set_high().unwrap();
    assert!(!pin04.is_set_low().unwrap());
    assert!(pin04.is_set_high().unwrap());

    pin00.set_high().unwrap();
    assert!(!pin00.is_set_low().unwrap());
    assert!(pin00.is_set_high().unwrap());

    pin12.set_high().unwrap();
    assert!(!pin12.is_set_low().unwrap());
    assert!(pin12.is_set_high().unwrap());
}

#[test]
fn test_regular_pin_set_low_write_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(2).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::Low).unwrap();

    let result = pin.set_low();
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_regular_pin_set_high_write_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(2).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::Low).unwrap();

    let result = pin.set_high();
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_regular_pin_set_state_write_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(2).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::Low).unwrap();

    let result = pin.set_state(PinState::High);
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_refreshable_pin_set_output_state() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2) // setting all low
        .mock_write(16) // mode switch
        .expect_write(1, &[0x02, 0b0000_0110]) // Update Bank 0
        .expect_write(1, &[0x03, 0b1110_0000]) // Update Bank 1
        .expect_write(1, &[0x02, 0b0000_0110]) // Update all
        .expect_write(1, &[0x03, 0b1110_0000]) // Update all
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, false).unwrap();
    expander.set_state_all(Bank1, false).unwrap();

    let pins = get_pins(&mut expander);
    let mut pin00 = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::Low).unwrap();
    let mut pin01 = pins.get_refreshable_pin(Bank0, Pin1).into_output_pin(PinState::Low).unwrap();
    let mut pin02 = pins.get_refreshable_pin(Bank0, Pin2).into_output_pin(PinState::Low).unwrap();
    let mut pin03 = pins.get_refreshable_pin(Bank0, Pin3).into_output_pin(PinState::Low).unwrap();

    let mut pin14 = pins.get_refreshable_pin(Bank1, Pin4).into_output_pin(PinState::Low).unwrap();
    let mut pin15 = pins.get_refreshable_pin(Bank1, Pin5).into_output_pin(PinState::Low).unwrap();
    let mut pin16 = pins.get_refreshable_pin(Bank1, Pin6).into_output_pin(PinState::Low).unwrap();
    let mut pin17 = pins.get_refreshable_pin(Bank1, Pin7).into_output_pin(PinState::Low).unwrap();

    pin00.set_low().unwrap();
    assert!(pin00.is_set_low().unwrap());
    assert!(!pin00.is_set_high().unwrap());

    pin01.set_high().unwrap();
    assert!(!pin01.is_set_low().unwrap());
    assert!(pin01.is_set_high().unwrap());

    pin02.set_high().unwrap();
    assert!(!pin02.is_set_low().unwrap());
    assert!(pin02.is_set_high().unwrap());

    pin03.set_low().unwrap();
    assert!(pin03.is_set_low().unwrap());
    assert!(!pin03.is_set_high().unwrap());

    pin14.set_low().unwrap();
    assert!(pin14.is_set_low().unwrap());
    assert!(!pin14.is_set_high().unwrap());

    pin15.set_high().unwrap();
    assert!(!pin15.is_set_low().unwrap());
    assert!(pin15.is_set_high().unwrap());

    pin16.set_high().unwrap();
    assert!(!pin16.is_set_low().unwrap());
    assert!(pin16.is_set_high().unwrap());

    pin17.set_high().unwrap();
    assert!(!pin17.is_set_low().unwrap());
    assert!(pin17.is_set_high().unwrap());

    pin03.update_bank().unwrap();
    pin16.update_bank().unwrap();
    pin17.update_all().unwrap();
}

#[test]
fn test_refreshable_pin_update_bank_write_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(2).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::Low).unwrap();

    let result = pin.update_bank();
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_refreshable_pin_update_all_write_error() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2)
        .expect_write(1, &[0x2, 0b1111_1111]) // Update Bank 0
        .write_error(0x3)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let pin = pins.get_refreshable_pin(Bank1, Pin0).into_output_pin(PinState::Low).unwrap();

    let result = pin.update_all();
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_regular_pin_into_output_pin() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x06, 0b1111_1110])
        .expect_write(1, &[0x02, 0b0000_0001])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, false).unwrap();
    let pins = get_pins(&mut expander);
    let _pin = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::High).unwrap();
}

#[test]
fn test_regular_pin_into_input_pin() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2)
        .expect_write(1, &[0x06, 0b1111_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let _pin = pins
        .get_pin(Bank0, Pin0)
        .into_output_pin(PinState::High)
        .unwrap()
        .into_input_pin()
        .unwrap();
}

#[test]
fn test_regular_pin_into_output_pin_mode_switch_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x6).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_regular_pin_into_output_pin_state_set_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(1).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_regular_pin_into_input_pin_mode_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x6).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_refreshable_pin_into_output_pin() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(1)
        .expect_write(1, &[0x06, 0b1111_1110])
        .expect_write(1, &[0x02, 0b0000_0001])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    expander.set_state_all(Bank0, false).unwrap();
    let pins = get_pins(&mut expander);
    let _pin = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::High).unwrap();
}

#[test]
fn test_refreshable_pin_into_input_pin() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2)
        .expect_write(1, &[0x06, 0b1111_1111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let _pin = pins
        .get_refreshable_pin(Bank0, Pin0)
        .into_output_pin(PinState::High)
        .unwrap()
        .into_input_pin()
        .unwrap();
}

#[test]
fn test_refreshable_pin_into_output_pin_mode_switch_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x6).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_refreshable_pin_into_output_pin_state_set_error() {
    let i2c_bus = BusMockBuilder::new().mock_write(1).write_error(0x2).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_refreshable_pin_into_input_pin_mode_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x6).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let result = pins.get_refreshable_pin(Bank0, Pin0).into_output_pin(PinState::High);

    assert!(result.is_err())
}

#[test]
fn test_regular_pin_toggle() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2) // Mode switch
        .expect_write(1, &[0x02, 0b1111_1011])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_pin(Bank0, Pin2).into_output_pin(PinState::High).unwrap();

    pin.toggle().unwrap();
}

#[test]
fn test_regular_pin_toggle_error() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2) // Mode switch
        .write_error(0x2)
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_pin(Bank0, Pin2).into_output_pin(PinState::High).unwrap();

    let result = pin.toggle();
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_refreshable_pin_toggle() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2) // Mode switch
        .expect_write(1, &[0x02, 0b1111_0111])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_refreshable_pin(Bank0, Pin3).into_output_pin(PinState::High).unwrap();

    pin.toggle().unwrap();
    pin.update_bank().unwrap();
}

#[test]
fn test_refreshable_pin_toggle_no_update() {
    let i2c_bus = BusMockBuilder::new()
        .mock_write(2) // Mode switch
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);
    let pins = get_pins(&mut expander);
    let mut pin = pins.get_refreshable_pin(Bank0, Pin3).into_output_pin(PinState::High).unwrap();

    pin.toggle().unwrap();
}

#[test]
fn test_regular_pin_invert_polarity() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x04, 0b0001_0000])
        .expect_write(1, &[0x04, 0b0000_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank0, Pin4);

    pin.invert_polarity(true).unwrap();
    pin.invert_polarity(false).unwrap();
}

#[test]
fn test_regular_pin_invert_polarity_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x04).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let pin = pins.get_pin(Bank0, Pin4);

    let result = pin.invert_polarity(true);
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

#[test]
fn test_refreshable_pin_invert_polarity() {
    let i2c_bus = BusMockBuilder::new()
        .expect_write(1, &[0x05, 0b0010_0000])
        .expect_write(1, &[0x05, 0b0000_0000])
        .into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let pin = pins.get_refreshable_pin(Bank1, Pin5);

    pin.invert_polarity(true).unwrap();
    pin.invert_polarity(false).unwrap();
}

#[test]
fn test_refreshable_pin_invert_polarity_error() {
    let i2c_bus = BusMockBuilder::new().write_error(0x05).into_mock();

    let mut expander = PCA9539::new(i2c_bus, 0x74);

    let pins = get_pins(&mut expander);
    let pin = pins.get_refreshable_pin(Bank1, Pin4);

    let result = pin.invert_polarity(true);
    assert_eq!(WriteError::Error1, result.unwrap_err());
}

/// Testing spin based RefGuard
#[cfg(feature = "spin")]
fn get_pins(expander: &mut PCA9539<MockI2CBus>) -> Pins<MockI2CBus, SpinGuard<MockI2CBus>> {
    expander.pins_spin_mutex()
}

/// Testing lock-free RefGuard
#[cfg(not(feature = "spin"))]
fn get_pins(expander: &mut PCA9539<MockI2CBus>) -> Pins<MockI2CBus, LockFreeGuard<MockI2CBus>> {
    expander.pins()
}
