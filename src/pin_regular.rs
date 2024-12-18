use crate::expander::{Bank, Mode, PinID, RefreshInputError};
use crate::guard::RefGuard;
use crate::pins::{Input, Output, Pin, PinMode, RegularAccessMode};
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};
use embedded_hal::i2c::{I2c, SevenBitAddress};

impl<'a, B, R> Pin<'a, B, R, Input, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
    pub fn regular(expander: &'a R, bank: Bank, id: PinID) -> Self {
        Pin {
            expander,
            bus: PhantomData,
            mode: PhantomData,
            access_mode: PhantomData,
            bank,
            id,
        }
    }
}

impl<B, R> ErrorType for Pin<'_, B, R, Input, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
    type Error = RefreshInputError<B>;
}

impl<B, R> InputPin for Pin<'_, B, R, Input, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let mut result = Ok(false);

        self.expander.access(|expander| {
            result = match expander.refresh_input_state(self.bank) {
                Ok(_) => Ok(expander.is_pin_input_high(self.bank, self.id)),
                Err(error) => Err(error),
            }
        });

        result
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}

impl<B, R> ErrorType for Pin<'_, B, R, Output, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
    type Error = RefreshInputError<B>;
}

impl<B, R> OutputPin for Pin<'_, B, R, Output, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
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
            result = expander.write_output_state(self.bank).map_err(RefreshInputError::WriteError);
        });

        result
    }
}

impl<B, R> StatefulOutputPin for Pin<'_, B, R, Output, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
{
    /// As this is just acting on cached register data, its in fact Infallible
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_pin_output_high())
    }

    /// As this is just acting on cached register data, its in fact Infallible
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_pin_output_high())
    }
}

impl<'a, B, M, R> Pin<'a, B, R, M, RegularAccessMode>
where
    B: I2c<SevenBitAddress>,
    R: RefGuard<B>,
    M: PinMode,
{
    pub fn into_input_pin(self) -> Result<Pin<'a, B, R, Input, RegularAccessMode>, B::Error> {
        self.change_mode(Mode::Input)?;

        Ok(Pin {
            expander: self.expander,
            bank: self.bank,
            id: self.id,
            bus: PhantomData,
            mode: PhantomData,
            access_mode: PhantomData,
        })
    }

    pub fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<Pin<'a, B, R, Output, RegularAccessMode>, RefreshInputError<B>> {
        self.change_mode(Mode::Output).map_err(RefreshInputError::WriteError)?;

        let mut pin = Pin {
            expander: self.expander,
            bank: self.bank,
            id: self.id,
            bus: PhantomData,
            mode: PhantomData,
            access_mode: PhantomData,
        };

        pin.set_state(state)?;
        Ok(pin)
    }
}
