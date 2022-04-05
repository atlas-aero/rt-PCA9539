use crate::expander::PCA9539;
use core::cell::RefCell;
use core::ops::DerefMut;
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};

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
    B: Write + Read<u8>,
{
    expander: RefCell<&'a mut PCA9539<B>>,
}

impl<'a, B: Write<SevenBitAddress> + Read<u8>> LockFreeGuard<'a, B> {
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
