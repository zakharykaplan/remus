//! Basic register models.
//!
//! # Usage
//!
//! The [`Register`] model should be used as a quick memory cell. It is generic
//! over the unsigned integer types, representing registers holding values of
//! types [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`] respectively.
//!
//! To provide access as the represented type, [`Register`] implements [`Deref`]
//! and [`DerefMut`] (it behaves as a newtype).
//!
//! Since [`Register`] implements [`Device`], it may be mapped to another
//! address space using a [`Bus`](crate::bus::Bus), and is [byte-addressable]
//! using [`Device::read()`] and [`Device::write()`].
//!
//! [byte-addressable]: https://en.wikipedia.org/wiki/Byte_addressing

use std::default::Default;
use std::ops::{Deref, DerefMut};

use num::Unsigned;

use crate::blk::Block;
use crate::dev::Device;

/// Register model.
#[derive(Debug, Default)]
pub struct Register<U: Unsigned>(U);

impl<U: Default + Unsigned> Register<U> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<U: Default + Unsigned> Block for Register<U> {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl<U: Unsigned> Deref for Register<U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U: Unsigned> DerefMut for Register<U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<U: Unsigned> From<U> for Register<U> {
    fn from(value: U) -> Self {
        Self(value)
    }
}

impl Device for Register<u8> {
    fn contains(&self, index: usize) -> bool {
        (0..std::mem::size_of::<Self>()).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self.to_le_bytes()[index]
    }

    fn write(&mut self, index: usize, value: u8) {
        let mut bytes = self.to_le_bytes();
        bytes[index] = value;
        self.0 = u8::from_le_bytes(bytes);
    }
}

impl Device for Register<u16> {
    fn contains(&self, index: usize) -> bool {
        (0..std::mem::size_of::<Self>()).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self.to_le_bytes()[index]
    }

    fn write(&mut self, index: usize, value: u8) {
        let mut bytes = self.to_le_bytes();
        bytes[index] = value;
        self.0 = u16::from_le_bytes(bytes);
    }
}

impl Device for Register<u32> {
    fn contains(&self, index: usize) -> bool {
        (0..std::mem::size_of::<Self>()).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self.to_le_bytes()[index]
    }

    fn write(&mut self, index: usize, value: u8) {
        let mut bytes = self.to_le_bytes();
        bytes[index] = value;
        self.0 = u32::from_le_bytes(bytes);
    }
}

impl Device for Register<u64> {
    fn contains(&self, index: usize) -> bool {
        (0..std::mem::size_of::<Self>()).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self.to_le_bytes()[index]
    }

    fn write(&mut self, index: usize, value: u8) {
        let mut bytes = self.to_le_bytes();
        bytes[index] = value;
        self.0 = u64::from_le_bytes(bytes);
    }
}

impl Device for Register<u128> {
    fn contains(&self, index: usize) -> bool {
        (0..std::mem::size_of::<Self>()).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self.to_le_bytes()[index]
    }

    fn write(&mut self, index: usize, value: u8) {
        let mut bytes = self.to_le_bytes();
        bytes[index] = value;
        self.0 = u128::from_le_bytes(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_works() {
        assert_eq!(std::mem::size_of::<Register::<u8>>(), 1);
        assert_eq!(std::mem::size_of::<Register::<u16>>(), 2);
        assert_eq!(std::mem::size_of::<Register::<u32>>(), 4);
        assert_eq!(std::mem::size_of::<Register::<u64>>(), 8);
        assert_eq!(std::mem::size_of::<Register::<u128>>(), 16);
    }

    #[test]
    fn new_works() {
        // 8-bit register
        let r8 = Register::<u8>::new();
        assert_eq!(r8.0, 0_u8);

        // 16-bit register
        let r16 = Register::<u16>::new();
        assert_eq!(r16.0, 0_u16);

        // 32-bit register
        let r32 = Register::<u32>::new();
        assert_eq!(r32.0, 0_u32);

        // 64-bit register
        let r64 = Register::<u64>::new();
        assert_eq!(r64.0, 0_u64);

        // 128-bit register
        let r128 = Register::<u128>::new();
        assert_eq!(r128.0, 0_u128);
    }

    #[test]
    fn from_works() {
        // 8-bit register
        let r8 = Register::<u8>::from(0x01_u8);
        assert_eq!(r8.0, 0x01_u8);

        // 16-bit register
        let r16 = Register::<u16>::from(0x0123_u16);
        assert_eq!(r16.0, 0x0123_u16);

        // 32-bit register
        let r32 = Register::<u32>::from(0x01234567_u32);
        assert_eq!(r32.0, 0x01234567_u32);

        // 64-bit register
        let r64 = Register::<u64>::from(0x0123456789abcdef_u64);
        assert_eq!(r64.0, 0x0123456789abcdef_u64);

        // 128-bit register
        let r128 = Register::<u128>::from(0x0123456789abcdef0123456789abcdef_u128);
        assert_eq!(r128.0, 0x0123456789abcdef0123456789abcdef_u128);
    }

    #[test]
    fn deref_works() {
        // 8-bit register
        let r8 = Register::<u8>::from(0x01_u8);
        assert_eq!(*r8, 0x01_u8);

        // 16-bit register
        let r16 = Register::<u16>::from(0x0123_u16);
        assert_eq!(*r16, 0x0123_u16);

        // 32-bit register
        let r32 = Register::<u32>::from(0x01234567_u32);
        assert_eq!(*r32, 0x01234567_u32);

        // 64-bit register
        let r64 = Register::<u64>::from(0x0123456789abcdef_u64);
        assert_eq!(*r64, 0x0123456789abcdef_u64);

        // 128-bit register
        let r128 = Register::<u128>::from(0x0123456789abcdef0123456789abcdef_u128);
        assert_eq!(*r128, 0x0123456789abcdef0123456789abcdef_u128);
    }

    #[test]
    fn deref_mut_works() {
        // 8-bit register
        let mut r8 = Register::<u8>::new();
        *r8 = 0x01_u8;
        assert_eq!(*r8, 0x01_u8);

        // 16-bit register
        let mut r16 = Register::<u16>::new();
        *r16 = 0x0123_u16;
        assert_eq!(*r16, 0x0123_u16);

        // 32-bit register
        let mut r32 = Register::<u32>::new();
        *r32 = 0x01234567_u32;
        assert_eq!(*r32, 0x01234567_u32);

        // 64-bit register
        let mut r64 = Register::<u64>::new();
        *r64 = 0x0123456789abcdef_u64;
        assert_eq!(*r64, 0x0123456789abcdef_u64);

        // 128-bit register
        let mut r128 = Register::<u128>::new();
        *r128 = 0x0123456789abcdef0123456789abcdef_u128;
        assert_eq!(*r128, 0x0123456789abcdef0123456789abcdef_u128);
    }

    #[test]
    fn device_contains_works() {
        (0..1).for_each(|addr| assert!(Register::<u8>::new().contains(addr)));
        (0..2).for_each(|addr| assert!(Register::<u16>::new().contains(addr)));
        (0..4).for_each(|addr| assert!(Register::<u32>::new().contains(addr)));
        (0..8).for_each(|addr| assert!(Register::<u64>::new().contains(addr)));
        (0..16).for_each(|addr| assert!(Register::<u128>::new().contains(addr)));
    }

    #[test]
    fn device_read_works() {
        // 8-bit register
        let r8 = Register::<u8>::from(0x01_u8);
        assert_eq!(r8.read(0), 0x01);

        // 16-bit register
        let r16 = Register::<u16>::from(0x0123_u16);
        assert_eq!(r16.read(1), 0x01);

        // 32-bit register
        let r32 = Register::<u32>::from(0x01234567_u32);
        assert_eq!(r32.read(2), 0x23);

        // 64-bit register
        let r64 = Register::<u64>::from(0x0123456789abcdef_u64);
        assert_eq!(r64.read(4), 0x67);

        // 128-bit register
        let r128 = Register::<u128>::from(0x0123456789abcdef0123456789abcdef_u128);
        assert_eq!(r128.read(8), 0xef);
    }

    #[test]
    fn device_write_works() {
        // 8-bit register
        let mut r8 = Register::<u8>::new();
        r8.write(0, 0xaa);
        assert_eq!(r8.read(0), 0xaa_u8);

        // 16-bit register
        let mut r16 = Register::<u16>::new();
        r16.write(1, 0xbb);
        assert_eq!(*r16, 0xbb00_u16);

        // 32-bit register
        let mut r32 = Register::<u32>::new();
        r32.write(2, 0xcc);
        assert_eq!(*r32, 0x00cc0000_u32);

        // 64-bit register
        let mut r64 = Register::<u64>::new();
        r64.write(4, 0xdd);
        assert_eq!(*r64, 0x000000dd00000000_u64);

        // 128-bit register
        let mut r128 = Register::<u128>::new();
        r128.write(8, 0xee);
        assert_eq!(*r128, 0x00000000000000ee0000000000000000_u128);
    }
}
