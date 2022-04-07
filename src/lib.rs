#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate alloc;

pub mod expander;
pub mod pins;

pub(crate) mod guard;
pub(crate) mod pin_refreshable;
pub(crate) mod pin_regular;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
