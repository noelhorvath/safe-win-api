use win_api_wrapper::win32::core::Result;
use win_api_wrapper::win32::foundation::{
    close_handle, ERROR_INVALID_HANDLE, ERROR_INVALID_WINDOW_HANDLE,
};

/// Provides extra error information for a type.
pub trait ErrorInfo {
    /// Gets the error code that should be used when the type is invalid.
    fn invalid_error_code() -> u32;
}

/// Represents a system resource that should be closed by calling [`close`][`Resource::close`] when it is no longer needed.
pub trait Resource {
    /// Type of the closable resource.
    type Type;

    /// Closes the open resoure.
    ///
    /// # Errors
    ///
    /// Failed to close the resource.
    fn close(resource: &Self::Type) -> Result<()>;
}

#[derive(Debug)]
pub struct Object;

#[derive(Debug)]
pub struct Registry;

#[derive(Debug)]
pub struct Window;

impl ErrorInfo for Object {
    fn invalid_error_code() -> u32 {
        ERROR_INVALID_HANDLE
    }
}

impl ErrorInfo for Window {
    fn invalid_error_code() -> u32 {
        ERROR_INVALID_WINDOW_HANDLE
    }
}

impl ErrorInfo for Registry {
    fn invalid_error_code() -> u32 {
        ERROR_INVALID_HANDLE
    }
}

impl Resource for Object {
    type Type = Handle<Self>;

    #[inline]
    /// Closes the specified handle by calling [`close_handle`].
    ///
    /// # Errors
    ///
    /// See `Erros` section in [`close_handle`].
    fn close(handle: &Self::Type) -> Result<()> {
        close_handle(handle)
    }
}

impl Resource for Registry {
    type Type = Handle<Self>;

    #[inline]
    /// Closes the specified handle by calling [`close_handle`].
    ///
    /// # Errors
    ///
    /// See `Erros` section in [`close_handle`].
    fn close(_handle: &Self::Type) -> Result<()> {
        Ok(())
    }
}

impl Resource for Window {
    type Type = Handle<Self>;

    #[inline]
    /// Closes the specified handle by calling [`close_handle`].
    ///
    /// # Errors
    ///
    /// See `Erros` section in [`close_handle`].
    fn close(_handle: &Self::Type) -> Result<()> {
        Ok(())
    }
}

/// Handle to a system object.
pub struct Handle<T = Object>
where
    T: Resource<Type = Self> + ErrorInfo,
{
    /// Unique identifier of the handle.
    pub(crate) id: isize,
    /// Object group of the handle.
    phantom_data: PhantomData<T>,
}

impl<T> Handle<T>
where
    T: Resource<Type = Self> + ErrorInfo,
{
    /// Null handle.
    pub const NULL_HANDLE: Self = Self::new(0);

    /// Creates a new [`Handle`] with the given `id`.
    const fn new(id: isize) -> Self {
        Self {
            id,
            phantom_data: PhantomData,
        }
    }

    /// Creates a new [`Handle`] with the given `id`.
    ///
    /// # Errors
    ///
    /// - `id` is invalid when it is less than equal to `0`.
    ///
    pub(crate) fn _try_new(id: isize) -> Result<Self> {
        if 0 < id {
            Ok(Self::new(id))
        } else {
            Err(Win32Error::new(T::invalid_error_code()))
        }
    }
}

impl<T> core::fmt::Debug for Handle<T>
where
    T: Resource<Type = Self> + ErrorInfo,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Handle").finish()
    }
}

impl<T> Drop for Handle<T>
where
    T: Resource<Type = Self> + ErrorInfo,
{
    fn drop(&mut self) {
        T::close(self).unwrap_or_default();
    }
}
