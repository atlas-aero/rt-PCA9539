//! # Concurrency wrappers
//!
//! See [concurrency section](crate::pins#concurrency) for more details.

use crate::expander::PCA9539;
use core::cell::RefCell;
use core::ops::DerefMut;

/// Manages the access of pins to expander reference
pub trait RefGuard<B>
where
    B: I2c<SevenBitAddress>,
{
    fn access<F>(&self, f: F)
    where
        F: FnMut(&mut PCA9539<B>);
}

/// Guard which is neither Send or Sync, but is the most efficient
pub struct LockFreeGuard<'a, B>
where
    B: I2c<SevenBitAddress>,
{
    expander: RefCell<&'a mut PCA9539<B>>,
}

impl<'a, B: I2c<SevenBitAddress>> LockFreeGuard<'a, B> {
    pub fn new(expander: RefCell<&'a mut PCA9539<B>>) -> Self {
        LockFreeGuard { expander }
    }
}

impl<B> RefGuard<B> for LockFreeGuard<'_, B>
where
    B: I2c<SevenBitAddress>,
{
    fn access<F>(&self, mut f: F)
    where
        F: FnMut(&mut PCA9539<B>),
    {
        f(self.expander.borrow_mut().deref_mut());
    }
}

#[cfg(feature = "cortex-m")]
use cortex_m::interrupt::Mutex as CsMutex;
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// Guard bases on Cortex-M mutex, which is using critical sections internally
#[cfg(feature = "cortex-m")]
pub struct CsMutexGuard<'a, B>
where
    B: I2c<SevenBitAddress>,
{
    expander: CsMutex<RefCell<&'a mut PCA9539<B>>>,
}

#[cfg(feature = "cortex-m")]
impl<'a, B: I2c<SevenBitAddress>> CsMutexGuard<'a, B> {
    pub fn new(expander: CsMutex<RefCell<&'a mut PCA9539<B>>>) -> Self {
        CsMutexGuard { expander }
    }
}

#[cfg(feature = "cortex-m")]
impl<B> RefGuard<B> for CsMutexGuard<'_, B>
where
    B: I2c<SevenBitAddress>,
{
    fn access<F>(&self, mut f: F)
    where
        F: FnMut(&mut PCA9539<B>),
    {
        cortex_m::interrupt::free(|cs| {
            f(self.expander.borrow(cs).borrow_mut().deref_mut());
        })
    }
}

#[cfg(feature = "spin")]
use spin::Mutex as SpinMutex;

#[cfg(feature = "spin")]
pub struct SpinGuard<'a, B>
where
    B: I2c<SevenBitAddress>,
{
    expander: SpinMutex<RefCell<&'a mut PCA9539<B>>>,
}

#[cfg(feature = "spin")]
impl<'a, B: I2c<SevenBitAddress>> SpinGuard<'a, B> {
    pub fn new(expander: SpinMutex<RefCell<&'a mut PCA9539<B>>>) -> Self {
        SpinGuard { expander }
    }
}

#[cfg(feature = "spin")]
impl<B> RefGuard<B> for SpinGuard<'_, B>
where
    B: I2c<SevenBitAddress>,
{
    fn access<F>(&self, mut f: F)
    where
        F: FnMut(&mut PCA9539<B>),
    {
        f(self.expander.lock().borrow_mut().deref_mut());
    }
}
