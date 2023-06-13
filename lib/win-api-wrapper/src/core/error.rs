use crate::common::To;
use crate::win32::foundation::get_last_error;
use crate::win32::system::diagnostics::debug::{format_message, FormatMessagetOptions};

pub const FACILITY_WIN32: u32 = 7;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Code(pub(crate) i32);

impl Code {
    pub const OK: Self = Code(0);

    #[inline]
    pub const fn is_ok(&self) -> bool {
        self.0 >= 0
    }

    #[inline]
    pub const fn is_error(self) -> bool {
        !self.is_ok()
    }

    #[inline]
    /// Converts a Win32 error code to an [`HRESULT`][windows_sys::core::HRESULT].
    ///
    /// For more information see the [`HRESULT_FROM_WIN32`][https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0c0bcf55-277e-4120-b5dc-f6115fc8dc38] macro specification.
    ///
    pub const fn from_win32(code: u32) -> Self {
        Self(if code == windows_sys::Win32::Foundation::ERROR_SUCCESS {
            code
        } else {
            (code & 0x0000_FFFF) | (FACILITY_WIN32 << 16) | 0x8000_0000
        } as i32)
    }

    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl To<i32> for Code {
    #[inline]
    fn to(&self) -> i32 {
        self.0
    }
}

impl To<u32> for Code {
    #[inline]
    fn to(&self) -> u32 {
        self.as_u32()
    }
}

impl To<Code> for i32 {
    #[inline]
    fn to(&self) -> Code {
        Code(*self)
    }
}

impl To<Code> for u32 {
    #[inline]
    fn to(&self) -> Code {
        Code::from_win32(*self)
    }
}

impl core::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl To<Error> for Code {
    #[inline]
    fn to(&self) -> Error {
        Error::from_code(*self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Repersents a Windows API error.
pub struct Error {
    /// The Windows API error code
    pub(crate) code: Code,
    /// The Windows API error message
    pub(crate) message: Box<str>,
}

impl Error {
    /// Error message that is used when [`format_message`] fails to retreive the error message using the error code.
    pub const FORMAT_MESSAGE_ERROR_MESSAGE: &'static str =
        "Failed to retrieve the error message from the system!";

    #[inline]
    pub(crate) const fn new(code: Code, message: Box<str>) -> Self {
        Self { code, message }
    }

    /// Creates a new [`Error`] from the specified error code.
    #[inline]
    pub fn from_code(code: Code) -> Self {
        let message = format_message(code.to(), 0, None, FormatMessagetOptions::All)
            .map_or(code.to_string(), |message| message.to_string_lossy())
            .into_boxed_str();
        Self::new(code, message)
    }

    #[inline]
    /// Gets the error code.
    pub const fn code(&self) -> i32 {
        self.code.0
    }

    #[inline]
    /// Gets the error message that is associated with from a system message-table.
    pub const fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    /// Determines whether the error code represents success. This method is the opposite of [`is_failure`][`Error::is_failure`].
    pub const fn is_success(&self) -> bool {
        self.code.is_ok()
    }

    #[inline]
    /// Determines whether the error code represents failure. This method is the opposite of [`is_success`][`Error::is_success`].
    pub const fn is_failure(&self) -> bool {
        self.code.is_error()
    }

    #[inline]
    /// Creates a new [`Error`] from the calling thread's last Win32 error code.
    pub(crate) fn last_win32() -> Self {
        Self::from_code(Code::from_win32(get_last_error()))
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.message(), self.code)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! last_error {
    () => {
        $crate::core::error::Error::last_win32()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_from {
    ($code:ident) => {
        $crate::core::error::Error::from_code($crate::common::To::<$crate::core::error::Code>::to(
            &$code,
        ))
    };
}
