use thiserror::Error;

/// Represents errors that can occur during debugging or low-level operations involving
/// Windows APIs and other system-level functionalities.
#[derive(Debug, Error)]
pub enum DbgError {
    /// Raised for general errors that do not fall under other categories.
    ///
    /// # Arguments
    ///
    /// * `{0}` - A static message providing details about the general error.
    #[error("{0}")]
    DbgGeneralError(&'static str),

    /// Raised when an invalid size is encountered during an operation.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The size value that caused the error.
    #[error("Invalid size: {0}")]
    InvalidSize(usize),

    /// Raised when a Windows API call fails.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The error returned by the Windows API, wrapped as `windows::core::Error`.
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),

    /// Raised when a string conversion fails due to invalid characters.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The error describing the issue with the string, wrapped as `std::ffi::NulError`.
    #[error("Invalid string: {0}")]
    InvalidString(#[from] std::ffi::NulError),

    /// Represents I/O-related errors, like reading or writing files.
    /// 
    /// Arguments:
    /// 
    /// * `{0}` - A std::io::Error describing the issue.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
