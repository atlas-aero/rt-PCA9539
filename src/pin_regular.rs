use crate::expander::{Bank, PinID, RefreshInputError};
use crate::guard::RefGuard;
use crate::pins::{Pin, RegularAccessMode};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::digital::v2::InputPin;

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
                Ok(_) => Ok(expander.is_pin_high(self.bank, self.id)),
                Err(error) => Err(error),
            }
        });

        result
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}
