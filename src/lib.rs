#![doc = include_str!("../README.md")]

/// The `error` module defines error types used throughout the library.
pub mod error;

/// The `dbg` module provides the core debugging interface for interacting with the debugger.
mod dbg;
pub use dbg::*;
