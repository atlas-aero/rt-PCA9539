use crate::expander::PCA9539;
use core::cell::RefCell;
use core::ops::DerefMut;
use embedded_hal::blocking::i2c::{Read, Write};

/// Manages the access of pins to expander reference
pub trait RefGuard<B>
where
    B: Write + Read<u8>,
{
    fn access<F>(&self, f: F)
    where
        F: FnMut(&mut PCA9539<B>);
}

/// Guard which is neither Send or Sync, but is the most efficient
pub struct LockFreeGuard<'a, B>
where
    B: Write + Read,
{
    expander: RefCell<&'a mut PCA9539<B>>,
}

impl<'a, B: Write + Read> LockFreeGuard<'a, B> {
    pub fn new(expander: RefCell<&'a mut PCA9539<B>>) -> Self {
        LockFreeGuard { expander }
    }
}

impl<'a, B> RefGuard<B> for LockFreeGuard<'a, B>
where
    B: Write + Read<u8>,
{
    fn access<F>(&self, mut f: F)
    where
        F: FnMut(&mut PCA9539<B>) -> (),
    {
        f(self.expander.borrow_mut().deref_mut());
    }
}

#[cfg(feature = "cortex-m")]
use cortex_m::interrupt::Mutex;

/// Guard bases on Cortex-M mutex, which is using critical sections internally
#[cfg(feature = "cortex-m")]
pub struct CsMutexGuard<'a, B>
where
    B: Write + Read<u8>,
{
    expander: Mutex<RefCell<&'a mut PCA9539<B>>>,
}

#[cfg(feature = "cortex-m")]
impl<'a, B: Write + Read> CsMutexGuard<'a, B> {
    pub fn new(expander: Mutex<RefCell<&'a mut PCA9539<B>>>) -> Self {
        CsMutexGuard { expander }
    }
}

#[cfg(feature = "cortex-m")]
impl<'a, B> RefGuard<B> for CsMutexGuard<'a, B>
where
    B: Write + Read<u8>,
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
