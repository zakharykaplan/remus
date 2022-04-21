//! # Rust Emulation Suite
//! > Modular emulator creation toolkit
//!
//! Remus provides the basic primitives for the creation of emulators. These
//! building blocks can be remixed to emulate a variety of systems.
//!
//! # Examples
//!
//! For an example of how to use Remus, consult
//! <https://github.com/zakharykaplan/gameboy>.

pub use self::blk::Block;
pub use self::clk::Clock;
#[doc(inline)]
pub use self::dev::{Device, SharedDevice};
pub use self::fsm::Machine;
#[doc(inline)]
pub use self::mem::Memory;

mod blk;
pub mod bus;
mod clk;
pub mod dev;
mod fsm;
pub mod mem;
pub mod reg;
