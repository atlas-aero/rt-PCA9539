//! # Abstraction of PCA9539
//!
//! Abstraction for I/O expander [PCA9539](<https://www.ti.com/lit/ds/symlink/pca9539.pdf?ts=1649342250975>).
//! This crate offers the following features:
//! * Individual pin instances, fully implementing [digital::v2 traits of embedded_hal](https://docs.rs/embedded-hal/latest/embedded_hal/digital/v2/index.html)
//! * Central I/O control, s. [PCA9539 module](crate::expander)
//! * Two state management modes for reduced I2C overhead, s. [pins module](crate::pins)
//! * Three concurrency models, s. [concurrency section](crate::pins#concurrency)
//! * no_std support
//!
//! ## Example
//! ```
//! use pca9539::example::DummyI2CBus;
//! use pca9539::expander::Bank::Bank0;
//! use pca9539::expander::PCA9539;
//! use pca9539::expander::PinID::Pin1;
//! use embedded_hal::digital::v2::InputPin;
//!
//! let i2c_bus = DummyI2CBus::new();
//! let mut  expander = PCA9539::new(i2c_bus, 0x74);
//! let pins = expander.pins();
//!
//! let pin01 = pins.get_pin(Bank0, Pin1);
//! assert!(pin01.is_high().unwrap());
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate alloc;
extern crate embedded_hal;

#[cfg(feature = "example")]
pub mod example;
pub mod expander;
pub mod guard;
pub mod pins;

pub(crate) mod pin_refreshable;
pub(crate) mod pin_regular;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
