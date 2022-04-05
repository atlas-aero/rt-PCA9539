#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate alloc;

pub mod expander;
pub mod guard;
pub mod pin;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
