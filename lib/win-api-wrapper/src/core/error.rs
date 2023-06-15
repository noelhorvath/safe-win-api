use crate::common::To;
use crate::win32::foundation::get_last_error;
use crate::win32::system::diagnostics::debug::{format_message, FormatMessagetOptions};
use crate::win32::system::library_loader::{free_library, load_library};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Code {
    Win32(u32),
    HResult(i32),
    NtStatus(i32),
}

impl Code {
    #[inline]
    pub const fn is_ok(self) -> bool {
        match self {
            Self::HResult(val) => val >= 0,
            _ => self.as_u32() == 0,
        }
    }

    #[inline]
    pub const fn is_error(self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn from_hresult(value: i32) -> Self {
        Self::HResult(value)
    }

    #[inline]
    pub const fn from_nt(value: i32) -> Self {
        Self::NtStatus(value)
    }

    #[inline]
    pub const fn from_win32(value: u32) -> Self {
        Self::Win32(value)
    }

    #[inline]
    pub const fn as_u32(self) -> u32 {
        match self {
            Code::HResult(val) => val as u32,
            Code::NtStatus(val) => val as u32,
            Code::Win32(val) => val,
        }
    }

    #[inline]
    pub const fn as_i32(self) -> i32 {
        match self {
            Code::HResult(val) => val,
            Code::NtStatus(val) => val,
            Code::Win32(val) => val as i32,
        }
    }

    #[inline]
    pub fn is_hresult(self) -> bool {
        if let Self::HResult(_) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_nt(self) -> bool {
        if let Self::NtStatus(_) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_win32(self) -> bool {
        if let Self::Win32(_) = self {
            true
        } else {
            false
        }
    }

    pub fn to_message(self) -> String {
        if self.is_nt() {
            // TODO: fix no message
            let lib_name = widestring::u16cstr!("Ntdll");
            // Ntdll.dll library is needed to be loaded inorder to get a proper message for an NTSTATUS.
            // https://stackoverflow.com/questions/25566234/how-to-convert-specific-ntstatus-value-to-the-hresult
            let module_handle = match load_library(lib_name) {
                Ok(handle) => handle,
                Err(err) => {
                    println!("{:?}", err);
                    return self.to_string();
                }
            };
            println!("handle: {}", module_handle);
            let message = format_message(self.as_u32(), 0, None, FormatMessagetOptions::All)
                .map_or(self.to_string(), |message| message.to_string_lossy());
            match free_library(module_handle) {
                _ => message,
            }
        } else {
            format_message(self.as_u32(), 0, None, FormatMessagetOptions::All)
                .map_or(self.to_string(), |message| message.to_string_lossy())
        }
    }
}

impl core::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HResult(val) => write!(f, "HRESULT({:#010x})", val),
            Self::NtStatus(val) => write!(f, "NTSTATUS({:#010x})", val),
            Self::Win32(val) => write!(f, "WIN32_ERROR({:#010x})", val),
        }
    }
}

impl core::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HResult(val) => write!(f, "{:#010x}", val),
            Self::NtStatus(val) => write!(f, "{:#010x}", val),
            Self::Win32(val) => write!(f, "{:#010x}", val),
        }
    }
}

impl To<Error> for Code {
    #[inline]
    fn to(&self) -> Error {
        Error::from_code(*self)
    }
}

#[derive(Clone, PartialEq, Eq)]
/// Repersents a Windows API error.
pub struct Error {
    /// The Windows API error message
    pub(crate) message: Box<str>,
    /// The Windows API error code
    pub(crate) code: Code,
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
        Self::new(code, code.to_message().into_boxed_str())
    }

    #[inline]
    /// Gets the error code.
    pub const fn code(&self) -> i32 {
        self.code.as_i32()
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
        match self.code {
            Code::HResult(val) => write!(f, "HRESULT: {} ({:#010x})", self.message, val),
            Code::NtStatus(val) => write!(f, "NTSTATUS: {} ({:#010x})", self.message, val),
            Code::Win32(val) => write!(f, "WIN32_ERROR: {} ({:#010x})", self.message, val),
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! last_error {
    (Win32) => {
        $crate::core::error::Error::last_win32()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_from {
    (HResult: $code:expr) => {
        $crate::core::error::Error::from_code($crate::core::error::Code::from_hresult($code))
    };
    (Win32: $code:expr) => {
        $crate::core::error::Error::from_code($crate::core::error::Code::from_win32($code))
    };
    (NtStatus: $code:expr) => {
        $crate::core::error::Error::from_code($crate::core::error::Code::from_nt($code))
    };
}
