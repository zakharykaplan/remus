//! Device adapters.
//!
//! # Usage
//!
//! All device adapters are used as wrappers around another (shared) device.
//! Through nesting adapters, users should be able to emulate fairly complex
//! memory-mapped layouts.
//!
//! Device adapters are themselves [`Device`](crate::dev::Device), so they use the same interface as
//! the devices they are modifying. As well, they all own the devices they
//! modify through an `Rc<RefCell<dyn Device>>`, allowing them to be shared or
//! remapped elsewhere.

pub use self::remap::Remap;
pub use self::view::View;
use super::DynDevice;

mod remap;
mod view;