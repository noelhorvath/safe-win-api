use super::{
    foundation::get_last_error,
    system::diagnostics::debug::{format_message, FormatMessagetOptions},
};
use widestring::U16CString;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;

/// The result of an error-prone Win32 API call.
pub type Result<T> = core::result::Result<T, Win32Error>;

#[derive(Debug, Clone)]
/// Repersents a Win32 error.
pub struct Win32Error {
    /// The Win32 error code
    pub(crate) code: u32,
    pub(crate) message: Box<str>,
}

impl Win32Error {
    /// Error message that is used when [`format_message`] fails to retreive the error message using the error code.
    pub const FORMAT_MESSAGE_ERROR_MESSAGE: &'static str = "Failed to retrieve the error message!";

    /// Creates a new [`Win32Error`] from the specified error code.
    pub fn from_code(code: u32) -> Self {
        let message = format_message(code, 0, None, FormatMessagetOptions::All)
            .unwrap_or(unsafe {
                // Safety: `FORMAT_MESSAGE_ERROR_MESSAGE` is a valid static str and doesn't contain null values.
                U16CString::from_str_unchecked(Self::FORMAT_MESSAGE_ERROR_MESSAGE)
            })
            .to_string_lossy()
            .into_boxed_str();
        Self { code, message }
    }

    #[inline]
    /// Gets the error code.
    pub const fn code(&self) -> u32 {
        self.code
    }

    #[inline]
    /// Gets the error message that is associated with from a system message-table.
    pub const fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    /// Determines whether the error code is [`ERROR_SUCCESS`]. This method is the opposite of [`is_failure`][`Win32Error::is_failure`].
    pub const fn is_success(&self) -> bool {
        self.code == ERROR_SUCCESS
    }

    #[inline]
    /// Determines whether the error code is not [`ERROR_SUCCESS`]. This method is the opposite of [`is_success`][`Win32Error::is_success`].
    pub const fn is_failure(&self) -> bool {
        !self.is_success()
    }

    #[inline]
    /// Gets the calling thread's last [`Win32Error`].
    pub(crate) fn get_last() -> Self {
        Self::from_code(get_last_error())
    }
}

impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Win32Error: {} -> {}", self.code, self.message)
    }
}
