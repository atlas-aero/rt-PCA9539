use crate::expander::{Bank, PinID, RefreshInputError};
use crate::guard::RefGuard;
use crate::pins::{Pin, RegularAccessMode};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState, StatefulOutputPin};

impl<'a, B, R> Pin<'a, B, R, RegularAccessMode>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    pub fn regular(expander: &'a R, bank: Bank, id: PinID) -> Self {
        Pin {
            expander,
            bus: PhantomData,
            access_mode: PhantomData,
            bank,
            id,
        }
    }
}

impl<'a, B, R> InputPin for Pin<'a, B, R, RegularAccessMode>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    type Error = RefreshInputError<B>;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let mut result = Ok(false);

        self.expander.access(|expander| {
            result = match expander.refresh_input_state(self.bank) {
                Ok(_) => Ok(expander.is_pin_input_high(self.bank, self.id)),
                Err(error) => Err(error),
            }
        });

        result
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}

impl<'a, B, R> OutputPin for Pin<'a, B, R, RegularAccessMode>
where
    B: Read + Write,
    R: RefGuard<B>,
{
    type Error = <B as Write>::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High)
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        let mut result = Ok(());

        self.expander.access(|expander| {
            expander.set_state(self.bank, self.id, state == PinState::High);
            result = expander.write_output_state(self.bank);
        });

        result
    }
}

impl<'a, B, R> StatefulOutputPin for Pin<'a, B, R, RegularAccessMode>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    /// As this is just acting on cached register data, its in fact Infallible
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_pin_output_high())
    }

    /// As this is just acting on cached register data, its in fact Infallible
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_pin_output_high())
    }
}
