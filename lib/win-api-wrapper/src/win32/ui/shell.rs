use crate::core::Result;
use crate::{call, call_BOOL, call_HRESULT, default_sized, free};
use core::ptr::{self, addr_of};
use core::slice;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::Shell::{
    AssocQueryStringW, CommandLineToArgvW, DuplicateIcon, ExtractIconExW, ExtractIconW,
    InitNetworkAddressControl, SHAppBarMessage, SHEmptyRecycleBinW, SHGetDiskFreeSpaceExW,
    SHGetDriveMedia, SHQueryRecycleBinW, SHQueryUserNotificationState, ShellAboutW,
    ShellExecuteExW, ShellMessageBoxW, Shell_NotifyIconGetRect, Shell_NotifyIconW, APPBARDATA,
    ASSOCF, ASSOCSTR, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER, SHQUERYRBINFO,
};

pub use windows_sys::Win32::UI::Shell::{
    QUNS_ACCEPTS_NOTIFICATIONS, SEE_MASK_ASYNCOK, SEE_MASK_CLASSKEY, SEE_MASK_CLASSNAME,
    SEE_MASK_CONNECTNETDRV, SHELLEXECUTEINFOW, SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI,
    SHERB_NOSOUND,
};

/// Contains drag-and-drop related Windows API functions.
pub mod drag;

/// Parses a Unicode command line string and returns an array of command line arguments.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// [TODO][https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw]
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw
///
pub fn parse_args(commandline: &U16CStr) -> Result<Vec<U16CString>> {
    let mut args_count = 0;
    let arg_ptrs = call! {
        CommandLineToArgvW(
            commandline.as_ptr(),
            &mut args_count,
        ) == ptr::null_mut::<*mut u16> () => return if Error
    };
    let mut args = if args_count == 0 {
        free!(Local: arg_ptrs);
        return Ok(Vec::new());
    } else {
        Vec::with_capacity(args_count as usize)
    };
    // Safety: `arg_ptrs` is valid pointer to an array of `*mut u16` pointers for `args_count` length. If length is 0 an empty slice will be crated which is safe.
    let arg_ptrs_slice = unsafe { slice::from_raw_parts(arg_ptrs, args_count as usize) };
    for &arg in arg_ptrs_slice {
        // Safety: each `arg_ptr` is a pointer to a null-terminated Unicode string.
        args.push(unsafe { U16CString::from_ptr_str(arg) });
    }

    free!(Local: arg_ptrs);
    Ok(args)
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Creates a duplicate of an icon specified by `icon_handle`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-duplicateicon
///
pub fn duplicate_icon(instance_handle: isize, icon_handle: isize) -> Result<isize> {
    call! { DuplicateIcon(instance_handle, icon_handle) != 0 }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets a handle to an icon from the executable file, DLL, or icon file specified by `file_path`.
/// If no icons were found in the file, the return value is `None`.
///
/// To retrieve an array of handles to large or small icons,
/// use [`extract_large_icons`] or [`extract_small_icons`].
///
/// When the icon is no longer needed, you must destroy the icon handle by calling [`destroy_icon`][super::window::destroy_icon].
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-extracticonw
///
pub fn extract_icon(app_instance_handle: isize, file_path: &U16CStr, index: u32) -> Option<isize> {
    let icon_handle = unsafe { ExtractIconW(app_instance_handle, file_path.as_ptr(), index) };
    if icon_handle == 0 {
        None
    } else {
        Some(icon_handle)
    }
}

/// Gets the total number of icons in the file specified by `file_path`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// See the implementation of [`extract_small_icons`] or [`extract_large_icons`].
///
pub fn extract_icon_count(file_path: &U16CStr) -> Result<u32> {
    call! {
        ExtractIconExW(
            file_path.as_ptr(),
            -1,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        ) != u32::MAX
    }
}

/// Gets an array of handles to small icons extracted from the executable file,
/// DLL, or icon file specified by `file_path`.
///
/// When the icons are no longer needed, you must destroy each icon handle
/// by calling [`destroy_icon`][super::window::destroy_icon].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-extracticonexw
///
pub fn extract_small_icons(file_path: &U16CStr) -> Result<Vec<isize>> {
    let mut icon_count = extract_icon_count(file_path)?;
    if icon_count == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0; icon_count as usize];
    icon_count = call! {
        ExtractIconExW(
            file_path.as_ptr(),
            0,
            buffer.as_mut_ptr(),
            ptr::null_mut(),
            0,
        ) == u32::MAX => return if Error
    };
    buffer.truncate(icon_count as usize);
    Ok(buffer)
}

/// Gets an array of handles to large icons extracted from the executable file,
/// DLL, or icon file specified by `file_path`.
///
/// When the icons are no longer needed, you must destroy each icon handle
/// by calling [`destroy_icon`][super::window::destroy_icon].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-extracticonexw
///
pub fn extract_large_icons(file_path: &U16CStr) -> Result<Vec<isize>> {
    let mut icon_count = extract_icon_count(file_path)?;
    if icon_count == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0; icon_count as usize];
    icon_count = call! {
        ExtractIconExW(
            file_path.as_ptr(),
            0,
            ptr::null_mut(),
            buffer.as_mut_ptr(),
            0,
        ) == u32::MAX => return if Error
    };
    buffer.truncate(icon_count as usize);
    Ok(buffer)
}

/// Initializes the network address control window class.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * The initialization failed.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-initnetworkaddresscontrol
///
pub fn init_network_address_control() -> Result<()> {
    call_BOOL! { InitNetworkAddressControl() }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Sends an appbar message to the system.
/// The content of `data` on entry and on exit depends on the value set in
/// the `message` parameter.
///
/// The returned value is a message-dependent value.
/// For more information, see the [Windows SDK documentation][https://learn.microsoft.com/en-us/windows/win32/shell/application-desktop-toolbars#appbar-notification-messages]
/// for the specific appbar message sent.
///
/// # Arguments
///
/// * `message`: The appbar message value to send.
/// * `data`: A mutable reference to an [`APPBARDATA`] instance.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shappbarmessage
///
pub fn send_app_bar_message(message: u32, data: &mut APPBARDATA) -> usize {
    unsafe { SHAppBarMessage(message, data) }
}

/// Searches for and gets the length of a file or protocol association-related
/// string from the registry.
///
/// # Arguments
///
/// * `search_flags`: The flags that can be used to control the search.
/// * `string_type`: The [`ASSOCSTR`] value that specifies the type of string that is to be returned.
/// * `association_string`: A string that is used to determine the root key.
/// * `extra`: Additional information about the location of the string.
///     * It is typically set to a Shell verb such as `open`.
///     * Set this parameter to `0` if it is not used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
pub fn get_assoc_string_len(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
    extra: Option<&U16CStr>,
) -> Result<u32> {
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            extra.map_or(ptr::null(), |ext| ext.as_ptr()),
            ptr::null_mut(),
            &mut char_count,
        ) -> mut char_count: u32
    }
}

/// Searches for and retrieves a file or protocol association-related string
/// from the registry.
///
/// # Arguments
///
/// * `search_flags`: The flags that can be used to control the search.
/// * `string_type`: The [`ASSOCSTR`] value that specifies the type of string that is to be returned.
/// * `association_string`: A string that is used to determine the root key.
/// * `extra`: Additional information about the location of the string.
///     * It is typically set to a Shell verb such as `open`.
///     * Set this parameter to `0` if it is not used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-assocquerystringw
///
pub fn get_asssoc_string(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
    extra: Option<&U16CStr>,
) -> Result<U16CString> {
    let mut char_count =
        get_assoc_string_len(search_flags, string_type, association_string, extra)?;
    let mut buffer = vec![0_u16; char_count as usize];
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            extra.map_or(ptr::null(), |ext| ext.as_ptr()),
            buffer.as_mut_ptr(),
            &mut char_count,
        ) return Error;
    }
    // Safety: `buffer` contains no interior null values
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

#[inline]
/// Performs an operation on specified by `execute_info`.
///
/// # Remarks
///
/// Because [`execute`] can delegate execution to `Shell` extensions
/// (data sources, context menu handlers, verb implementations) that are activated using
/// `Component Object Model` (`COM`), `COM` should be initialized before [`execute`]
///  is called. Some `Shell` extensions require the `COM` single-threaded apartment
/// (`STA`) type. In that case, COM should be initialized with the following options
/// as shown here:
/// ```
/// # use crate::win32::com::{self, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};
/// let options = COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE;
/// if let Err(error) = com::initialize(options) {
///     println!("COM library initialization failed: {}", error);
/// }
/// ```
/// There are instances where [`execute`] does not use one of these types of `Shell `
/// extension and those instances would not require `COM` to be initialized at all.
/// Nonetheless, it is good practice to always initialize `COM` before using this function.
///
/// With multiple monitors, if you specify a window handle and set the `lpVerb` field of
/// the [`SHELLEXECUTEINFOW`] instance specified by `execute_info` to "Properties",
/// any windows created by ShellExecuteEx might not appear in the correct position.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible erros
///
/// * [`ERROR_FILE_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND]
///     * The specified file in `execute_info` was not found.
/// * [`ERROR_PATH_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_PATH_NOT_FOUND]
///     * The specified path in `execute_info` was not found.
/// * [`ERROR_DDE_FAIL`][windows_sys::Win32::Foundation::ERROR_DDE_FAIL]
///     * The `Dynamic Data Exchange` (`DDE`) transaction failed.
/// * [`ERROR_NO_ASSOCIATION`][windows_sys::Win32::Foundation::ERROR_NO_ASSOCIATION]
///     * There is no application associated with the file name extension
///       specified in `execute_info`
/// * [`ERROR_ACCESS_DENIED`][windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED]
///     * Access to the file specified in `execute_info` is denied.
/// * [`ERROR_DLL_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_DLL_NOT_FOUND]
///     * One of the library files necessary to run the application can't be found.
/// * [`ERROR_CANCELLED`][windows_sys::Win32::Foundation::ERROR_CANCELLED]
///     * The function prompted the user for additional information, but the user
///       canceled the request.
/// * [`ERROR_NOT_ENOUGH_MEMORY`][windows_sys::Win32::Foundation::ERROR_NOT_ENOUGH_MEMORY]
///     * There is not enough memory to perform the specified action.
/// * [`ERROR_SHARING_VIOLATION`][windows_sys::Win32::Foundation::ERROR_SHARING_VIOLATION]
///     * A sharing violation occured.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation]
/// or [Launching Applications] article.
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw
/// [Launching Application]: https://learn.microsoft.com/en-us/windows/win32/shell/launch
pub fn execute(execute_info: &mut SHELLEXECUTEINFOW) -> Result<()> {
    call_BOOL! { ShellExecuteExW(execute_info) }
}

/// Gets the screen coordinates of the bounding rectangle of a notification icon
/// specified by `notify_icon_id`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyicongetrect
///
pub fn get_notify_icon_rect(notify_icon_id: NOTIFYICONIDENTIFIER) -> Result<RECT> {
    call_HRESULT!(
        Shell_NotifyIconGetRect(
            addr_of!(notify_icon_id),
            &mut rect,
        ) -> mut rect = default_sized!(RECT)
    )
}

/// Sends a message to the taskbar's status area.
///
/// # Arguments
///
/// * `message`: A value that specifies the action to be taken by this function.
/// * `data`: A reference to [`NOTIFYICONDATA`] instance.
///     * The content of the parameter depends on the value of `message`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw
///
pub fn send_notify_icon_message(message: u32, data: &NOTIFYICONDATAW) -> Result<()> {
    call_BOOL! {
        Shell_NotifyIconW(message, data)
    }
}

/// Displays a `ShellAbout` dialog box with the specified title, text and icon.
///
/// # Arguments
///
/// * `window_handle`: A window handle to a parent window.
///     * This parameter can be `0`.
/// * `title`: A string that contains text to be displayed in the title bar of
///          the `ShellAbout` dialog box and on the first line of the dialog
///          box after the text `Microsoft`.
///     * If `title` contains a separator (#) that divides it into two parts,
///       the function displays the first part in the title bar and the second part
///       on the first line after the text `Microsoft`.
/// * `text`: A string that contains text to be displayed in the dialog box after
///           the version and copyright information.
/// * `icon_handle`: The handle of an icon that the function displays in the dialog box.
///     * If the parameter is `0`, the function display the Windows icon.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `window_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellaboutw
///
pub fn show_about(
    window_handle: isize,
    title: &U16CStr,
    text: &U16CStr,
    icon_handle: isize,
) -> Result<()> {
    call_BOOL! {
        ShellAboutW(window_handle, title.as_ptr(), text.as_ptr(), icon_handle)
    }
}

/// Displays a `ShellMessageBox` message box with the specified title, message and style.
/// `ShellMessageBox` is a special instance of `MessageBox` that provides the
/// option of using the owner window's title as the title of the message box.
///
/// The return value indicates a button that was pressed in the message box.
///
/// # Arguments
///
/// * `module_handle`: The handle of the module from which to load a string resource named in `title`.
///     * If `title` does not name a string resource, this parameter is ignored.
/// * `window_handle`: A handle to the owner window of the message box to be created.
///     * If this parameter is not `0`, the title of the owner window is used as the
///       title of the message box.
/// * `message`: A string that contains either the message to be displayed or
///              a resource ID specifying where the message is to be retrieved from.
/// * `title`: An optional string that contains the dialog box title or a resource ID specifying
///            where the title is to be retrieved.
///     * If `tile` is `None` and `window_handle` is `0`, no title is displayed.
///     * If this parameter is a loadable resource it overrides
///       `window_handle` as the title.
/// * `style`: Specifies the contents and behavior of the dialog box.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `module_handle` is invalid.
/// * `window_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellmessageboxw
///
pub fn show_message_box(
    module_handle: isize,
    window_handle: isize,
    message: &U16CStr,
    title: Option<&U16CStr>,
    style: u32,
) -> Result<i32> {
    call! {
        ShellMessageBoxW(
            module_handle,
            window_handle,
            message.as_ptr(),
            title.map_or(ptr::null(), |title| title.as_ptr()),
            style
        ) != 0
    }
}

/// Empties the `Recycle Bin` on the drive specified by `root_drive_path`.
///
/// # Arguments
///
/// * `window_handle`: A handle to the parent window of any dialog boxes that might
///                    be displayed during the operation.
/// * `root_drive_path`: A path to the root drive on which the Recycle Bin is located.
///     * If this parameter is an empty string, all  Recycle Bins` on all drives will be emptied.
/// * `flags`: One or more of the following values:
///     * [`SHERB_NOCONFIRMATION`]: No dialog box confirming the deletion of the objects will be displayed.
///     * [`SHERB_NOPROGRESSUI`]: No dialog box indicating the progress will be displayed.
///     * [`SHERB_NOSOUND`]: No sound will be played when the operation is complete.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `window_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shemptyrecyclebinw
///
pub fn empty_bin(window_handle: isize, root_drive_path: &U16CStr, flags: u32) -> Result<()> {
    call_HRESULT! { SHEmptyRecycleBinW(window_handle, root_drive_path.as_ptr(), flags) }
}

/// Retrieves disk space information for a disk volume specified by `dir_path`.
///
/// If the function succeeds the returned `(u64, u64, u64)`
/// contains the following values in the following order:
///
///  * Number of bytes on the volumme available to the calling application.
///  * The total size of the volume in bytes.
///  * The number of bytes of free space on the volume.
///
/// # Arguments
///
/// * `dir_path`: A string that specifies the volume for which size information is retrieved.
///               This can be a drive letter, `UNC` name, or the path of a folder.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `dir_path` does not exist.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetdiskfreespaceexw
///
pub fn get_disk_free_space(dir_path: &U16CStr) -> Result<(u64, u64, u64)> {
    call_BOOL! {
        SHGetDiskFreeSpaceExW(
            dir_path.as_ptr(),
            &mut available_bytes,
            &mut total_bytes,
            &mut total_free_bytes
        ) -> mut (available_bytes, total_bytes, total_free_bytes): (u64, u64, u64)
    }
}

/// Gets the type of media that is in `drive`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `drive` does not exist.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetdrivemedia
///
pub fn get_drive_media(drive: &U16CStr) -> Result<u32> {
    call_HRESULT! {
        SHGetDriveMedia(
            drive.as_ptr(),
            &mut media_content
        ) -> mut media_content: u32
    }
}

/// Gets the size of the `Recycle Bin` and the number of items in it,
/// for a specified drive.
///
/// The `hIcon` member of the `SHFILEINFO` must be freed with
/// [`destroy_icon`][crate::win32::ui::window::destroy_icon].
///
/// # Arguments
///
/// * `root_path`: The path of the root drive on which the `Recycle Bin` is located.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `root_path` does not exist.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shqueryrecyclebinw
///
pub fn query_bin(root_path: &U16CStr) -> Result<SHQUERYRBINFO> {
    call_HRESULT! {
        SHQueryRecycleBinW(
            root_path.as_ptr(),
            &mut bin_info,
        ) -> mut bin_info = default_sized!(Cb -> mut bin_info: SHQUERYRBINFO)
    }
}

/// Checks the state of the computer for the current user to determine
/// whether sending a notification is appropriate.
///
/// # Arguments
///
/// * `root_path`: The path of the root drive on which the `Recycle Bin` is located.
///
/// # Remarks
///
/// Applications should call [`query_user_notification_state`] and test the return
/// value before displaying any notification UI that is similar to the balloon
/// notifications generated by [`send_notify_icon_message`]. Notifications should only
/// be displayed if this API returns [`QUNS_ACCEPTS_NOTIFICATIONS`]. This informs the
/// application whether the user is running processes that should not be interrupted.
/// Top-level windows receive a [`WM_SETTINGCHANGE`][super::messaging::WM_SETTINGCHANGE]
/// message when the user turns presentation settings on or off, and also when
/// the user's session is locked or unlocked. Note that there are no notifications
/// sent when the user starts or stops a full-screen application.
///
/// If this function returns QUNS_QUIET_TIME, notifications should be displayed
/// only if critical.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `root_path` does not exist.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shqueryusernotificationstate
///
pub fn query_user_notification_state() -> Result<i32> {
    call_HRESULT! {
        SHQueryUserNotificationState(&mut state) -> mut state: i32
    }
}
