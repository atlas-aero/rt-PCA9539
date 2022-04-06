#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate alloc;

pub mod expander;
pub mod guard;
pub mod pin_refreshable;
pub mod pin_regular;
pub mod pins;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
