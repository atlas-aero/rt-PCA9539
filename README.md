# Abstraction for I/O expander PCA9539
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io](https://img.shields.io/crates/v/pca9539.svg)](https://crates.io/crates/pca9539)
[![Actions Status](https://github.com/pegasus-aero/rt-PCA9539/workflows/QA/badge.svg)](http://github.com/pegasus-aero/rt-PCA9539/actions)

Abstraction for I/O expander [PCA9539](<https://www.ti.com/lit/ds/symlink/pca9539.pdf?ts=1649342250975>).

This crate offers the following features:
* Individual pin instances, fully implementing [digital::v2 traits of embedded_hal](https://docs.rs/embedded-hal/latest/embedded_hal/digital/v2/index.html)
* Central I/O control, s. [PCA9539 module](https://docs.rs/pca9539/0.1.0/pca9539/expander/index.html)
* Two state management modes for reduced I2C overhead, s. [pins module](https://docs.rs/pca9539/0.1.0/pca9539/pins/index.html)
* Three concurrency models, s. [concurrency section](https://docs.rs/pca9539/0.1.0/pca9539/pins/index.html#concurrency)
* no_std support

## Example
```rust
use rca9539::example::DummyI2CBus;
use rca9539::expander::Bank::Bank0;
use rca9539::expander::PCA9539;
use rca9539::expander::PinID::Pin1;
use embedded_hal::digital::v2::InputPin;

let i2c_bus = DummyI2CBus::new();
let mut  expander = PCA9539::new(i2c_bus);
let pins = expander.pins();

let pin01 = pins.get_pin(Bank0, Pin1);
assert!(pin01.is_high().unwrap());
```

## Development

Any form of support is greatly appreciated. Feel free to create issues and PRs.
See [DEVELOPMENT](DEVELOPMENT.md) for more details.  

## License
Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

Each contributor agrees that his/her contribution covers both licenses.