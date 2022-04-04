use crate::expander::Bank::{Bank0, Bank1};
use crate::expander::PinID::{Pin3, Pin4, Pin5, Pin6};
use crate::expander::PCA9539;
use crate::mocks::MockI2CBus;

#[test]
fn test_expander_output_mode_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(2).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_0111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.output_mode(Bank0, Pin3).unwrap();
    assert_eq!(0b1111_0111, expander.configuration_0);
    assert_eq!(0b1111_1111, expander.configuration_1);

    // Calling again does not change the result
    expander.output_mode(Bank0, Pin3).unwrap();
    assert_eq!(0b1111_0111, expander.configuration_0);
    assert_eq!(0b1111_1111, expander.configuration_1);
}

#[test]
fn test_expander_output_mode_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(2).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1011_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.output_mode(Bank1, Pin6).unwrap();
    assert_eq!(0b1111_1111, expander.configuration_0);
    assert_eq!(0b1011_1111, expander.configuration_1);

    // Calling again does not change the result
    expander.output_mode(Bank1, Pin6).unwrap();
    assert_eq!(0b1111_1111, expander.configuration_0);
    assert_eq!(0b1011_1111, expander.configuration_1);
}

#[test]
fn test_expander_input_mode_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(2).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(2).returning(move |address, data| {
        assert_eq!(0x06, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0001_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.all_output(Bank0).unwrap();
    expander.all_output(Bank1).unwrap();

    expander.input_mode(Bank0, Pin4).unwrap();
    assert_eq!(0b0001_0000, expander.configuration_0);
    assert_eq!(0x0000_0000, expander.configuration_1);

    // Calling again does not change the result
    expander.input_mode(Bank0, Pin4).unwrap();
    assert_eq!(0b0001_0000, expander.configuration_0);
    assert_eq!(0x0000_0000, expander.configuration_1);
}

#[test]
fn test_expander_input_mode_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(2).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(2).returning(move |address, data| {
        assert_eq!(0x07, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0010_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);
    expander.all_output(Bank0).unwrap();
    expander.all_output(Bank1).unwrap();

    expander.input_mode(Bank1, Pin5).unwrap();
    assert_eq!(0x0000_0000, expander.configuration_0);
    assert_eq!(0b0010_0000, expander.configuration_1);

    // Calling again does not change the result
    expander.input_mode(Bank1, Pin5).unwrap();
    assert_eq!(0x0000_0000, expander.configuration_0);
    assert_eq!(0b0010_0000, expander.configuration_1);
}

#[test]
fn test_expander_all_input_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x6, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.all_output(Bank0).unwrap();
    expander.all_input(Bank0).unwrap();
    assert_eq!(0b1111_1111, expander.configuration_0);
    assert_eq!(0b1111_1111, expander.configuration_1);
}

#[test]
fn test_expander_all_input_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |_, _| Ok(()));
    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x7, address);

        assert_eq!(1, data.len());
        assert_eq!(0b1111_1111, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.all_output(Bank1).unwrap();
    expander.all_input(Bank1).unwrap();
    assert_eq!(0b1111_1111, expander.configuration_0);
    assert_eq!(0b1111_1111, expander.configuration_1);
}

#[test]
fn test_expander_all_output_bank0() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x6, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.all_output(Bank0).unwrap();
    assert_eq!(0b0000_0000, expander.configuration_0);
    assert_eq!(0b1111_1111, expander.configuration_1);
}

#[test]
fn test_expander_all_output_bank1() {
    let mut i2c_bus = MockI2CBus::new();

    i2c_bus.expect_write().times(1).returning(move |address, data| {
        assert_eq!(0x7, address);

        assert_eq!(1, data.len());
        assert_eq!(0b0000_0000, data[0]);
        Ok(())
    });

    let mut expander = PCA9539::new(i2c_bus);

    expander.all_output(Bank1).unwrap();
    assert_eq!(0b1111_1111, expander.configuration_0);
    assert_eq!(0b0000_0000, expander.configuration_1);
}
