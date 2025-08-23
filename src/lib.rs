//! # dbg-rs 🦀
//!
//! Safe Rust bindings for the **Windows debugging engine (DbgEng)** COM interfaces.
//!
//! ## Features
//! - Safe abstractions over COM-based debugging interfaces.
//! - Easy-to-use macros for logging to the debugger.
//! - Helpers for managing symbols, memory, and CPU registers.
//! - Seamless integration with the Windows debugger engine.
//!
//! ## Examples
//!
//! ### Logging to the Debugger
//! ```no_run
//! use dbg_rs::dprintln;
//!
//! fn main() {
//!     dprintln!(dbg, "Hello, {}!", "Debugger");
//!     dprintln!(dbg, "Number: {}", 42);
//! }
//! ```
//!
//! ### Executing Commands
//! ```no_run
//! use dbg_rs::Dbg;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let dbg = Dbg::new()?;
//!     dbg.exec(".echo Hello, Debugger!")?;
//!     Ok(())
//! }
//! ```
//!
//! ### Reading Virtual Memory
//! ```no_run
//! use dbg_rs::Dbg;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let dbg = Dbg::new()?;
//!     let mut buffer = vec![0u8; 128];
//!     dbg.read_vaddr(0x7FFEBEEF0000, &mut buffer)?;
//!     println!("Read memory: {:?}", &buffer[..16]);
//!     Ok(())
//! }
//! ```
//!
//! # More Information
//!
//! For additional examples and usage, visit the [repository].
//!
//! [repository]: https://github.com/joaoviictorti/dbg-rs

pub mod error;

mod dbg;
pub use dbg::*;
