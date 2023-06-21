/// Information about a snapshot entry.
pub trait SnapshotInfo: Sized {
    /// Gets the associated [CREATE_TOOLHELP_SNAPSHOT_FLAGS] flags of the type.
    ///
    /// [CREATE_TOOLHELP_SNAPSHOT_FLAGS]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/struct.CREATE_TOOLHELP_SNAPSHOT_FLAGS.html
    fn snapshot_creation_flags() -> CREATE_TOOLHELP_SNAPSHOT_FLAGS;

    /// Returns the first entry in `snapshot` or `None`.
    ///
    /// # Errors
    ///
    /// Returns a [`WinError`] if the function fails, that includes the Windows API error code and message, providing more details about the cause of the failure.
    ///
    /// ## Possible errors
    /// - `snapshot` handle is invalid.
    ///
    fn first_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle;

    /// Returns the next entry in `snapshot` or `None`.
    ///
    /// # Errors
    ///
    /// Returns a [`WinError`] if the function fails, that includes the Windows API error code and message, providing more details about the cause of the failure.
    ///
    /// ## Possible errors
    /// - `snapshot` handle is invalid.
    ///
    fn next_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle;
}

impl SnapshotInfo for ProcessEntry {
    fn snapshot_creation_flags() -> CREATE_TOOLHELP_SNAPSHOT_FLAGS {
        TH32CS_SNAPPROCESS
    }

    #[inline]
    fn first_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle,
    {
        first_process(snapshot.handle.id())
    }

    #[inline]
    fn next_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle,
    {
        next_process(snapshot.handle.id())
    }
}

impl SnapshotInfo for ThreadEntry {
    fn snapshot_creation_flags() -> CREATE_TOOLHELP_SNAPSHOT_FLAGS {
        TH32CS_SNAPTHREAD
    }

    #[inline]
    fn first_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle,
    {
        first_thread(snapshot.handle.id())
    }

    #[inline]
    fn next_entry<TInfo, THandle>(snapshot: &Snapshot<TInfo, THandle>) -> Result<Option<Self>>
    where
        TInfo: SnapshotInfo,
        THandle: Handle,
    {
        next_thread(snapshot.handle.id())
    }
}

/// A read-only system snapshot.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let snapshot: Snapshot<ProcessEntry> = match Snapshot::try_create() {
///     Ok(snapshot) => snapshot,
///     /* Err(error) => handle_snapshot_creation_error(error); */
/// #   Err(error) => return Err(error),
/// };
/// for entry in snapshot {
///     /* Do something with the entry */
/// }
/// ```
pub struct Snapshot<TInfo, THandle = BasicSnapshotHandle>
where
    TInfo: SnapshotInfo,
    THandle: Handle,
{
    /// Open handle to the snapshot
    handle: THandle,
    /// Indicates wheter the first entry was consumed
    was_first_called: bool,
    /// Phantom data of the `TInfo`
    phantom_type: PhantomData<TInfo>,
}

impl<T: SnapshotInfo> core::fmt::Debug for Snapshot<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Snapshot")
            .field("was_first_called", &self.was_first_called)
            .finish()
    }
}

impl<TInfo, THandle> Snapshot<TInfo, THandle>
where
    TInfo: SnapshotInfo,
    THandle: Handle,
{
    /// Creates a new [snapshot](Snapshot) that includes the process specified by `pid`.
    ///
    /// # Errors
    ///
    /// Returns a [`WinError`] if the function fails, that includes the Windows API error code and message, providing more details about the cause of the failure.
    ///
    /// ## Possible errors
    /// - `pid` is the idle process or one of the CSRSS processes. ([ERROR_ACCESS_DENIED])
    /// - `pid` is a 64-bit process and the caller is a 32-bit process. ([ERROR_PARTIAL_COPY])
    ///
    /// [ERROR_ACCESS_DENIED]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/constant.ERROR_ACCESS_DENIED.html
    /// [ERROR_PARTIAL_COPY]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/constant.ERROR_PARTIAL_COPY.html
    fn try_create_with_pid(pid: u32) -> Result<Self> {
        let handle = THandle::new(create_snapshot(TInfo::snapshot_creation_flags(), pid)?);
        let snapshot = Self {
            handle,
            was_first_called: false,
            phantom_type: PhantomData::default(),
        };

        // First call to `first_entry` must be checked in order to make sure that the snapshot iteration can be started without failure.
        // It is important to check after the snapshot is created, beacause `next` iterator calls will yield `None` instead of failing.
        TInfo::first_entry(&snapshot)?;
        Ok(snapshot)
    }

    #[inline]
    /// Creates a new [snapshot](Snapshot).
    ///
    /// # Errors
    ///
    /// Returns a [`WinError`] if the function fails, that includes the Windows API error code and message, providing more details about the cause of the failure.
    ///
    /// ## Possible errors
    /// - `pid` is the idle process or one of the CSRSS processes. ([ERROR_ACCESS_DENIED])
    /// - `pid` is a 64-bit process and the caller is a 32-bit process. ([ERROR_PARTIAL_COPY])
    ///
    /// [ERROR_ACCESS_DENIED]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/constant.ERROR_ACCESS_DENIED.html
    /// [ERROR_PARTIAL_COPY]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/constant.ERROR_PARTIAL_COPY.html
    pub fn try_create() -> Result<Self> {
        // `pid` is ignored
        Self::try_create_with_pid(0)
    }

    /// Resets the snapshot iterator.
    ///
    /// # Errors
    ///
    /// Returns a [`WinError`] if the function fails, that includes the Windows API error code and message, providing more details about the cause of the failure.
    pub fn reset(&mut self) -> Result<()> {
        TInfo::first_entry(self)?;

        if self.was_first_called {
            self.was_first_called = false;
        }

        Ok(())
    }

    /// Gets a value that specifies whether the first entry was consumed.
    pub const fn was_first_called(&self) -> bool {
        self.was_first_called
    }

    /// Sets `was_first_called` to `true` if it is `false`.
    fn can_use_next(&mut self) {
        if !self.was_first_called {
            self.was_first_called = true;
        }
    }
}

impl<T: Handle> Iterator for Snapshot<ProcessEntry, T> {
    type Item = ProcessEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.was_first_called() {
            next_process(self.handle.id()).unwrap_or(None)
        } else {
            let result = first_process(self.handle.id()).unwrap_or(None);
            self.can_use_next();
            result
        }
    }
}

impl<T: Handle> Iterator for Snapshot<ThreadEntry, T> {
    type Item = ThreadEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.was_first_called() {
            next_thread(self.handle.id()).unwrap_or(None)
        } else {
            let result = first_thread(self.handle.id()).unwrap_or(None);
            self.can_use_next();
            result
        }
    }
}
