use super::foundation::get_last_error;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;

/// The result of an error-prone Win32 API call.
pub type Result<T> = core::result::Result<T, Win32Error>;

#[derive(Debug, Copy, Clone)]
/// Repersents a Win32 error.
pub struct Win32Error {
    /// The Win32 error code
    pub(crate) code: u32,
}

impl Win32Error {
    /// Creates a new [`Win32Error`] with the given error code.
    pub const fn new(code: u32) -> Self {
        Self { code }
    }

    #[inline]
    /// Gets the error code.
    pub const fn code(&self) -> u32 {
        self.code
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
        Self::new(get_last_error())
    }
}

impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Win32Error: {} -> message", self.code)
    }
}
