// Copyright (c) 2025 joaoviictorti
// Licensed under the MIT License. See LICENSE file in the project root for details.

//! The module defines error types used throughout the library.

use thiserror::Error;

/// Represents errors that can occur during debugging or low-level operations involving
/// Windows APIs and other system-level functionalities.
#[derive(Debug, Error)]
pub enum DbgError {
    /// Raised for general errors that do not fall under other categories.
    #[error("{0}")]
    DbgGeneralError(&'static str),

    /// Raised when an invalid size is encountered during an operation.
    #[error("Invalid size: {0}")]
    InvalidSize(usize),

    /// Raised when a Windows API call fails.
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),

    /// Raised when a string conversion fails due to invalid characters.
    #[error("Invalid string: {0}")]
    InvalidString(#[from] std::ffi::NulError),

    /// Represents I/O-related errors, like reading or writing files.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
