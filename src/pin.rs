use crate::expander::{Bank, PinID, RefreshInputError};
use crate::guard::RefGuard;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::digital::v2::InputPin;

/// GPIO pin, which synchronously updates its status via I2C
pub struct RegularPin<B, R>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    expander: R,
    bus: PhantomData<fn(B) -> B>,
    bank: Bank,
    id: PinID,
}

impl<B, R> RegularPin<B, R>
where
    B: Write + Read,
    R: RefGuard<B>,
{
    pub fn new(expander: R, bank: Bank, id: PinID) -> Self {
        RegularPin {
            expander,
            bus: PhantomData,
            bank,
            id,
        }
    }
}

impl<B, R> InputPin for RegularPin<B, R>
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
