use crate::expander::{Bank, PinID, RefreshInputError};
use crate::guard::RefGuard;
use crate::pins::{Pin, RefreshMode};
use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::digital::v2::InputPin;

impl<'a, B, R> Pin<'a, B, R, RefreshMode>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    pub fn refreshable(expander: &'a R, bank: Bank, id: PinID) -> Self {
        Self {
            expander,
            bus: PhantomData,
            bank,
            id,
            access_mode: PhantomData,
        }
    }

    /// Refreshes the input state of all pins of the same bank
    pub fn refresh_bank(&self) -> Result<(), RefreshInputError<B>> {
        self.refresh(self.bank)
    }

    /// Refreshes the input state of all pins (on all banks)
    pub fn refresh_all(&self) -> Result<(), RefreshInputError<B>> {
        self.refresh(Bank::Bank0)?;
        self.refresh(Bank::Bank1)
    }

    /// Refreshes the given bank
    fn refresh(&self, bank: Bank) -> Result<(), RefreshInputError<B>> {
        let mut result = Ok(());

        self.expander.access(|expander| {
            result = expander.refresh_input_state(bank);
        });

        result
    }
}

impl<'a, B, R> InputPin for Pin<'a, B, R, RefreshMode>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let mut state = false;

        self.expander.access(|expander| {
            state = expander.is_pin_input_high(self.bank, self.id);
        });

        Ok(state)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}
