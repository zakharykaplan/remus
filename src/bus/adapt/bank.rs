use crate::blk::Block;
use crate::dev::{Device, SharedDevice};

/// Device bank.
///
/// # Usage
///
/// The `Bank` device adapter provides a switchable bank of devices to be used
/// when performing [`Device`] operations.
///
/// As it is simply a wrapper, its fields are public can be accessed directly.
#[derive(Debug, Default)]
pub struct Bank {
    sel: usize,
    banks: Vec<SharedDevice>,
}

impl Bank {
    /// Constructs a new, empty `Bank`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the selected device.
    #[must_use]
    pub fn get(&self) -> usize {
        self.sel
    }

    /// Sets the selected device.
    pub fn set(&mut self, sel: usize) {
        self.sel = sel;
    }

    /// Appends a device to the back of a bank.
    pub fn add(&mut self, dev: SharedDevice) {
        self.banks.push(dev)
    }

    /// Clears the bank, removing all devices.
    pub fn clear(&mut self) {
        self.banks.clear()
    }

    /// Inserts an device at position `index` within the bank, shifting all
    /// devices after it to the right.
    pub fn insert(&mut self, index: usize, dev: SharedDevice) {
        self.banks.insert(index, dev)
    }

    /// Removes and returns the device at position `index` within the bank,
    /// shifting all devices after it to the left.
    pub fn remove(&mut self, index: usize) -> SharedDevice {
        self.banks.remove(index)
    }
}

impl Block for Bank {
    fn reset(&mut self) {
        self.sel = 0;
        for bank in &self.banks {
            bank.borrow_mut().reset();
        }
    }
}

impl Device for Bank {
    fn contains(&self, index: usize) -> bool {
        match self.banks.get(self.sel) {
            Some(bank) => bank.borrow().contains(index),
            None => false,
        }
    }

    fn len(&self) -> usize {
        self.banks[self.sel].borrow().len()
    }

    fn read(&self, index: usize) -> u8 {
        self.banks[self.sel].borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        self.banks[self.sel].borrow_mut().write(index, value);
    }
}

impl From<Vec<SharedDevice>> for Bank {
    fn from(banks: Vec<SharedDevice>) -> Self {
        Self {
            banks,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::{Null, Random};
    use crate::mem::Ram;

    fn setup() -> Bank {
        let mut bank = Bank::new();
        let ram = Ram::<0x100>::from(&[0x55; 0x100]).to_shared();
        let null = Null::<0>::new().to_shared();
        let random = Random::<0x100>::new().to_shared();
        bank.banks.extend([ram, null, random]);
        bank
    }

    #[test]
    fn new_works() {
        let _ = Bank::new();
    }

    #[test]
    fn device_contains_works() {
        let mut bank = setup();
        // Test bank 0
        bank.sel = 0;
        (0x00..=0xff).for_each(|addr| assert!(bank.contains(addr)));
        // Test bank 1
        bank.sel = 1;
        (0x00..=0xff).for_each(|addr| assert!(!bank.contains(addr)));
        // Test bank 0
        bank.sel = 2;
        (0x00..=0xff).for_each(|addr| assert!(bank.contains(addr)));
    }

    #[test]
    fn device_len_works() {
        let mut bank = setup();
        // Test bank 0
        bank.sel = 0;
        assert_eq!(bank.len(), 0x100);
        // Test bank 1
        bank.sel = 1;
        assert_eq!(bank.len(), 0);
        // Test bank 0
        bank.sel = 2;
        assert_eq!(bank.len(), 0x100);
    }

    #[test]
    fn device_read_works() {
        let mut bank = setup();
        // Test bank 0
        bank.sel = 0;
        (0x00..=0xff).for_each(|addr| assert_eq!(bank.read(addr), 0x55));
        // Test bank 2
        bank.sel = 2;
        (0x00..=0xff).for_each(|addr| {
            let _ = bank.read(addr);
        });
    }

    #[test]
    fn device_write_works() {
        let mut bank = setup();
        // Test bank 0
        bank.sel = 0;
        (0x00..=0xff).for_each(|addr| bank.write(addr, 0xaa));
        (0x00..=0xff).for_each(|addr| assert_eq!(bank.read(addr), 0xaa));
        // Test bank 2
        bank.sel = 2;
        (0x00..=0xff).for_each(|addr| bank.write(addr, 0xaa));
        // NOTE: For all intents and purposes, this should never fail. If it
        //       does, one of two things happened:
        //       1. You broke something either in Bank or Random
        //       2. You broke this test
        //       3. You broke probability, all hope is lost
        assert!((0x00..=0xff)
            .map(|addr| bank.read(addr))
            .any(|value| value != 0xaa));
    }
}
