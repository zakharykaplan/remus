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

pub use blk::Block;
pub use dev::Device;
pub use fsm::Machine;

mod blk;
pub mod bus;
pub mod clk;
pub mod dev;
mod fsm;
pub mod mem;
pub mod reg;
