use std::fmt::{Debug, Display};
use std::ops::Deref;

use crate::blk::Block;
use crate::dev::Device;
use crate::mem::Memory;

/// Read-only memory model.
///
/// Panics on [`write`](Rom::write).
#[derive(Debug)]
pub struct Rom<const N: usize>([u8; N]);

impl<const N: usize> Rom<N> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<const N: usize> Block for Rom<N> {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl<const N: usize> Default for Rom<N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<const N: usize> Deref for Rom<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Device for Rom<N> {
    fn contains(&self, index: usize) -> bool {
        (0..<[u8]>::len(self)).contains(&index)
    }

    fn read(&self, index: usize) -> u8 {
        self[index]
    }

    /// # Panics
    ///
    /// Panics when attempting to write to a [`Rom`].
    fn write(&mut self, _index: usize, _value: u8) {
        panic!("called `Device::write()` on a `Rom`");
    }
}

impl<const N: usize> Display for Rom<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self as &dyn Memory)
    }
}

impl<const N: usize> From<&[u8; N]> for Rom<N> {
    fn from(arr: &[u8; N]) -> Self {
        Self(*arr)
    }
}

impl<const N: usize> Memory for Rom<N> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::Device;

    #[test]
    fn size_of_works() {
        assert_eq!(std::mem::size_of::<Rom::<0x0>>(), 0x0);
        assert_eq!(std::mem::size_of::<Rom::<0x1>>(), 0x1);
        assert_eq!(std::mem::size_of::<Rom::<0x10>>(), 0x10);
        assert_eq!(std::mem::size_of::<Rom::<0x100>>(), 0x100);
        assert_eq!(std::mem::size_of::<Rom::<0x1000>>(), 0x1000);
        assert_eq!(std::mem::size_of::<Rom::<0x10000>>(), 0x10000);
    }

    #[test]
    fn new_works() {
        let rom = Rom::<0x100>::new();
        assert!(rom.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn from_works() {
        const N: usize = 0x100;

        let arr = [0; N];
        let rom = Rom::<N>::from(&arr);
        assert_eq!(*rom, arr);

        let vec: Vec<u8> = (0..N).map(|x| x as u8).collect();
        let buf = vec.try_into().unwrap();
        let rom = Rom::<N>::from(&buf);
        assert_eq!(*rom, buf);
    }

    #[test]
    fn device_contains_works() {
        const N0: usize = 0x0;
        let rom = Rom::<N0>::new();
        (0..N0).for_each(|addr| assert!(rom.contains(addr)));

        const N1: usize = 0x1;
        let rom = Rom::<N1>::new();
        (0..N1).for_each(|addr| assert!(rom.contains(addr)));

        const N2: usize = 0x10;
        let rom = Rom::<N2>::new();
        (0..N2).for_each(|addr| assert!(rom.contains(addr)));

        const N3: usize = 0x100;
        let rom = Rom::<N3>::new();
        (0..N3).for_each(|addr| assert!(rom.contains(addr)));

        const N4: usize = 0x1000;
        let rom = Rom::<N4>::new();
        (0..N4).for_each(|addr| assert!(rom.contains(addr)));

        const N5: usize = 0x10000;
        let rom = Rom::<N5>::new();
        (0..N5).for_each(|addr| assert!(rom.contains(addr)));
    }

    #[test]
    fn device_read_works() {
        let rom = Rom::<0x1>::from(&[0xaa]);
        assert_eq!(rom.read(0x0), 0xaa);
    }

    #[test]
    #[should_panic]
    fn device_write_panics() {
        let mut rom = Rom::<0x1>::from(&[0xaa]);
        rom.write(0x0, 0xaa);
    }
}
