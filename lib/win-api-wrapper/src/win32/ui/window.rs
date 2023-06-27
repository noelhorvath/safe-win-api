use crate::core::Result;
use crate::{call, call_BOOL, from_BOOL, to_BOOL};
use core::ptr;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::{HWND, LPARAM, POINT};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AllowSetForegroundWindow, ArrangeIconicWindows, BringWindowToTop, ChildWindowFromPointEx,
    CloseWindow, DestroyIcon, DestroyWindow, EnumChildWindows, EnumThreadWindows, EnumWindows,
    FindWindowExW, GetAncestor, GetDesktopWindow, GetParent, GetShellWindow, GetTopWindow,
    GetWindow, GetWindowTextLengthW, GetWindowTextW, IsChild, IsIconic, IsWindow, IsWindowVisible,
    IsZoomed, LogicalToPhysicalPoint, MoveWindow, OpenIcon, RealChildWindowFromPoint, SetParent,
    SetWindowTextW, ShowWindow, ShowWindowAsync,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow, WindowFromPhysicalPoint,
    WindowFromPoint,
};

pub use windows_sys::Win32::UI::WindowsAndMessaging::{
    CWP_ALL, CWP_FLAGS, CWP_SKIPDISABLED, CWP_SKIPINVISIBLE, CWP_SKIPTRANSPARENT, GA_PARENT,
    GA_ROOT, GA_ROOTOWNER, GET_ANCESTOR_FLAGS, GET_WINDOW_CMD, GW_CHILD, GW_ENABLEDPOPUP,
    GW_HWNDFIRST, GW_HWNDLAST, GW_HWNDNEXT, GW_HWNDPREV, GW_OWNER, HWND_MESSAGE, SHOW_WINDOW_CMD,
    SW_FORCEMINIMIZE, SW_HIDE, SW_MAX, SW_MAXIMIZE, SW_MINIMIZE, SW_NORMAL, SW_RESTORE, SW_SHOW,
    SW_SHOWDEFAULT, SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA, SW_SHOWNOACTIVATE, WS_POPUP,
};

/// An application defined function that can be passed to [`enum_child_windows`],
/// [`enum_thread_windows`] or [`enum_windows`].
pub type EnumWindowsCallback = unsafe extern "system" fn(param0: HWND, param1: LPARAM) -> BOOL;

/// Enumerates the child windows that belong to the parent window specified by `handle`
/// by passing the handle to each child window, in turn, to `callback` function.
/// The function continues until the last child window is enumerated or the
/// callback function returns `FALSE` (`0`).
///
/// To enumerate child windows of a particular window, use [`enum_child_windows`].
///
/// # Arguments
///
/// * `handle`: A handle to the parent window whose child windows are to be enumerated.
///     * If this parameter is `0`, this function is equivalent to [`enum_windows`].
/// * `callback`: A pointer to an application-defined callback function.
/// * `parameter`: An application-defined value to be passed to the callback function.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows
///
pub fn enum_child_windows(handle: isize, callback: EnumWindowsCallback, parameter: isize) {
    unsafe { EnumChildWindows(handle, Some(callback), parameter) };
}

/// Enumerates all nonchild windows associated with a thread by passing the handle to each
/// window, in turn, to `callback` function.
/// The function continues until the last window is enumerated or the
/// callback function returns `FALSE` (`0`).
///
/// If `callback` returns `TRUE` (non-zero value) for all windows in the thread specified by
/// `thread_id`, the reutrn valie is `TRUE`. If the callback function returns `FALSE` (`0`) on
/// any enumerated window, or it there are no windows found in the thread, the return value is
/// `FALSE`.
///
/// # Arguments
///
/// * `thread_id`: The identifier of the thread whose windows are to be enumerated.
///     * If this parameter is `0`, this function is equivalent to [`enum_windows`].
/// * `callback`: A pointer to an application-defined callback function.
/// * `parameter`: An application-defined value to be passed to the callback function.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumthreadwindows
///
pub fn enum_thread_windows(
    thread_id: u32,
    callback: EnumWindowsCallback,
    parameter: isize,
) -> bool {
    from_BOOL!(unsafe { EnumThreadWindows(thread_id, Some(callback), parameter) })
}

/// Enumerates all top-level windows on the screen by passing the handle to each window,
/// in turn, to `callback` function.
/// The function continues until the last top-level window is enumerated or the
/// callback function returns `FALSE` (`0`).
///
/// # Arguments
///
/// * `callback`: A pointer to an application-defined callback function.
/// * `parameter`: An application-defined value to be passed to the callback function.
///
/// # Remarks
///
/// For `Windows 8` and later, [`enum_windows`] enumerates only top-level windows of desktop apps.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `callback` has set an error code using [`set_last_error`][crate::win32::foundation::set_last_error].
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows
///
pub fn enum_windows(callback: EnumWindowsCallback, parameter: isize) -> Result<()> {
    call_BOOL! { EnumWindows(Some(callback), parameter) }
}

/// Gets a handle to a window whose class name and window name match the specified strings.
/// The function searches child windows, beginning with the one following the specified
/// child window. This function does not perform a case-sensitive search.
///
/// The function searches only direct child windows. It does not search other descendants.
///
/// # Arguments
///
/// * `parent_handle`: A handle to the parent window whose child windows are to be searched.
///     * If the parameter is
///         * `0`, the function uses the desktop windows as the parent window
///           and searches among the windows that are child of the desktop.
///         * [`HWND_MESSAGE`], the function searches all message-only windows.
/// * `child_handle`: A handle to a child window that is direct child of `parent_handle`.
///     * If this parameter is  `0`, the search begins with the first child window of `parent_handle`.
///     * If both this parameter and `parent_handle` are `0`, the function searches all top-level
///       and message-only windows.
/// * `class_name`: The class name or a class atom.
/// * `title`: The window name.
///     * If this parameter is `0`, all window names match
///
/// # Remarks
///
/// The function searches only direct child windows. It does not search other descendants.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `parent_handle` is invalid.
/// * `child_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw
///
pub fn find(
    parent_handle: isize,
    child_handle: isize,
    class_name: Option<&U16CStr>,
    title: Option<&U16CStr>,
) -> Result<Option<isize>> {
    {
        call! {
            FindWindowExW(
                parent_handle,
                child_handle,
                class_name.map_or(ptr::null(), |name| name.as_ptr()),
                title.map_or(ptr::null(), |title| title.as_ptr()),
            ) != 0 => Result<Option>
        }
    }
}

/// Gets a handle to the foreground window (the window with which the user is currently working).
/// The system assigns a slightly higher priority to the thread that creates the foreground window
/// than it does to other threads.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow
///
pub fn get_foreground() -> Option<isize> {
    call! { GetForegroundWindow() != 0 => Option }
}

/// Brings the thread that created the specified window into the foreground and activates the window.
/// Keyboard input is directed to the window, and various visual cues are changed for the user.
/// The system assigns a slightly higher priority to the thread that created the foreground window
/// than it does to other threads.
///
/// The return value specifies whether the window specified by `handle` was brought
/// to the foreground.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow
///
pub fn set_foreground(handle: isize) -> Result<bool> {
    call_BOOL! { SetForegroundWindow(handle) -> bool }
}

/// Gets a handle to the window that contains the specified point.
/// If no windows exists at the given point, the return value is `None`.
///
/// The function does not retrieve a handle to a hidden or disabled window.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint
///
pub fn from_point(point: POINT) -> Option<isize> {
    call! { WindowFromPoint(point) != 0 => Option }
}

/// Gets a handle to the window that contains the specified physical point.
/// If no windows exists at the given point, the return value is `None`.
///
/// The function does not retrieve a handle to a hidden or disabled window.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint
///
pub fn from_physical_point(point: POINT) -> Option<isize> {
    call! { WindowFromPhysicalPoint(point) != 0 => Option }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets a handle to the desktop window. The desktop window covers the entire screen.
/// The desktop window is the area on top of which other windows are painted.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow
///
pub fn get_desktop_window() -> isize {
    unsafe { GetDesktopWindow() }
}

/// Gets a handle to the window's parent or owner.
///
/// If the window is a child window, the return value is a handle to the parent window.
/// If the window is a top-level window with the WS_POPUP style, the return value is a
/// handle to the owner window.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * The window specified by `handle` is a top-level window that is unowned
///   or does not have the [`WS_POPUP`] style.
/// * The owner window has [`WS_POPUP`] style.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent
///
pub fn get_parent(handle: isize) -> Result<Option<isize>> {
    call! { GetParent(handle) != 0 => Result<Option> }
}

/// Gets a handle to the Shell's desktop window.
/// If no `Shell` process is present, the return value is `None`.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow
///
pub fn get_shell_window() -> Option<isize> {
    call! { GetShellWindow() != 0 => Option }
}

/// Gets the handle to the ancestor of the specified window.
///
/// The `flags` parameter can be one of the followin values:
///
/// | [`GET_ANCESTOR_FLAGS`] | Description |
/// |------|------|
/// | [`GA_PARENT`] | Retrieves the parent window. This does not include the owner, as it does with the [`get_parent`]. |
/// | [`GA_ROOT`] | Retrieves the root window by walking the chain of parent windows. |
/// | [`GA_ROOTOWNER`] | Retrieves the owned root window by walking the chain of parent and owner windows returned by [`get_parent`]. |
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor
///
pub fn get_ancestor(handle: isize, flags: GET_ANCESTOR_FLAGS) -> Option<isize> {
    call! { GetAncestor(handle, flags) != 0 => Option }
}

/// Gets the identifier of the thread that created the window specified by `handle` and
/// the identifier of the process that created the window.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid
///
pub fn get_process_and_thread_ids(handle: isize) -> Result<(u32, u32)> {
    let mut pid = 0;
    let thread_id = call! {
        GetWindowThreadProcessId(
            handle,
            &mut pid,
        ) == 0 => return if Error
    };
    Ok((pid, thread_id))
}

/// Gets the length, in characters, of the specified window's title bar text
/// (if the window has a title bar). If the specified window is a control,
/// the function retrieves the length of the text within the control.
/// However, [`get_text_len`] cannot retrieve the length of the text of an
/// edit control in another application.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw
///
pub fn get_text_len(handle: isize) -> Result<i32> {
    call! { GetWindowTextLengthW(handle) == 0 => ClearLastError }
}

/// Gets the text of the specified window's title bar (if it has one).
/// If the specified window is a control, the text of the control is copied.
/// However, [`get_text`] cannot retrieve the text of a control in another application.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw
///
pub fn get_text(handle: isize) -> Result<U16CString> {
    let mut len = get_text_len(handle)?;
    let mut buffer = vec![0; len as usize];
    len = call! {
        GetWindowTextW(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as i32
        ) == 0 => return if Error;
    };
    // Safety: `buffer` is valid pointer to a null-terminated string that has no interior null values
    Ok(unsafe { U16CString::from_ptr_unchecked(buffer.as_ptr(), len as usize) })
}

/// Copies the text of the specified window's title bar (if it has one) into a buffer.
/// If the specified window is a control, the text of the control is copied.
/// However, [`get_text_with_buffer`] cannot retrieve the text of a control in another application.
///
/// If the text is shorther than `buffer`'s length, the string is truncated and termintated
/// with a null character.
/// To avoid passing small buffer, use [`get_text_len`] to get the length of the window`s text,
/// before calling this function.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw
///
pub fn get_text_with_buffer(handle: isize, buffer: &mut [u16]) -> Result<u32> {
    call! {
        GetWindowTextW(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as i32,
        ) != 0 => u32
    }
}

/// Examines the Z order of the child windows associated with the parent window specified by
/// `handle` and gets a handle to the child window at the top of the Z order.
///
/// If the specified window has no child windows the return value is `None`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow
///
pub fn get_top_child(handle: isize) -> Result<Option<isize>> {
    call! { GetTopWindow(handle) != 0 => Result<Option> }
}

/// Retrieves a handle to a window that has the specified relationship (Z-Order or owner)
/// to the window specified by `handle`.
///
/// If no window exists with the relationship specified by `flags`to the given window,
/// the return value is `None`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow
///
pub fn get_window(handle: isize, flags: GET_WINDOW_CMD) -> Result<Option<isize>> {
    call! { GetWindow(handle, flags) != 0 => Result<Option> }
}

/// Enables the specified process to set the foreground window using [`set_foreground`].
/// The calling process must already be able to set the foreground window.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `pid` is invalid.
/// * The calling process specified by `pid` cannot set the foreground window.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-allowsetforegroundwindow
///
pub fn allow_set_foreground(pid: u32) -> Result<()> {
    call_BOOL! { AllowSetForegroundWindow(pid) }
}

/// Determines whether the window specified by `child_handle` is a child window or
/// descendant window of `parent_handle`. A child window is the direct descendant of a
/// specified parent window if that parent window is in the chain of parent windows;
/// the chain of parent windows leads from the original overlapped or pop-up window
/// to the child window.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild
///
pub fn is_child(parent_handle: isize, child_handle: isize) -> bool {
    from_BOOL!(unsafe { IsChild(parent_handle, child_handle) })
}

/// Determines whether the window specified by `handle` is minimezed.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic
///
pub fn is_minimized(handle: isize) -> bool {
    from_BOOL!(unsafe { IsIconic(handle) })
}

/// Determines the visibility state of the window specified by `handle`.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible
///
pub fn is_visible(handle: isize) -> bool {
    from_BOOL!(unsafe { IsWindowVisible(handle) })
}

/// Determines whether the window specified by `handle` is maximized.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed
///
pub fn is_maximized(handle: isize) -> bool {
    from_BOOL!(unsafe { IsZoomed(handle) })
}

/// Determines whether the window specified by `handle` is an existing window.
///
/// A thread should not use [`exists`] for a window that it did not create because
/// the window could be destroyed after this function was called.
/// Further, because window handles are recycled the handle could even point to
/// a different window.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow
///
pub fn exists(handle: isize) -> bool {
    from_BOOL!(unsafe { IsWindow(handle) })
}

/// Arranges all the minimized child windows of the parent window specified by `handle`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows
///
pub fn arrange_iconic_windows(handle: isize) -> Result<u32> {
    call! { ArrangeIconicWindows(handle) != 0 }
}

/// Converts the logical coordinates of a point in the window specified by `handle`
/// to physical coordinates.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint
///
pub fn logical_to_physical_point(handle: isize, point: POINT) -> Result<POINT> {
    call_BOOL! {
        LogicalToPhysicalPoint(
            handle,
            &mut converted,
        ) -> mut converted = point
    }
}

/// Restores a minimized window to its previous size and position.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openicon
///
pub fn open_minimized(handle: isize) -> Result<()> {
    call_BOOL! { OpenIcon(handle) }
}

/// Changes the position and dimensions of the window specified by `handle`. For a top-level
/// window, the position and dimensions are relative to the upper-left corner of the screen.
/// For a child window, they are relative to the upper-left corner of the parent window's
/// client area.
///
/// # Arguments
///
/// * `handle`: A handle to the window.
/// * `point`: The new coordinates of the window's top-left corner (`x` and `y` fields).
/// * `height`: The new width of the window.
/// * `width`: The new height of the window.
/// * `redraw`: Indicates whether the window is to be repainted.
///     * If the parameter is `true`. the system sends [`WM_PAINT`][super::messaging::WM_PAINT]
///       message to the window procedure immediately after moving the window.
///     * If the parameter is `false`, the application must explicitly invalidate or
///       redraw any parts of the window and parent window that need redrawing.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow
///
pub fn r#move(handle: isize, point: POINT, height: i32, width: i32, redraw: bool) -> Result<()> {
    call_BOOL! { MoveWindow(handle, point.x, point.y, height, width, to_BOOL!(redraw)) }
}

/// Brings the window specified by `handle` to the top of the Z order.
/// If the window is a top-level window, it is activated.
/// If the window is a child window, the top-level parent window associated with the
/// child window is activated.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop
///
pub fn to_top(handle: isize) -> Result<()> {
    call_BOOL! { BringWindowToTop(handle) }
}

/// Determines which, if any, of the child windows belonging to the parent window
/// specified by `handle` contains `point`. The function can ignore invisible,
/// disabled, and transparent child windows. The search is restricted to immediate child
/// windows.
/// Grandchildren and deeper descendants are not searched.
///
/// | [`CWP_FLAGS`] | Description |
/// |------|------|
/// | [`CWP_ALL`] | Does not skip any child windows. |
/// | [`CWP_SKIPDISABLED`] | Skips disabled child windows. |
/// | [`CWP_SKIPINVISIBLE`] | Skips invisible child windows. |
/// | [`CWP_SKIPTRANSPARENT`] | Skips transparent child windows. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompointex
///
pub fn child_from_point(handle: isize, point: POINT, flags: CWP_FLAGS) -> Result<Option<isize>> {
    call! {
        ChildWindowFromPointEx(handle, point, flags) != 0 => Result<Option>
    }
}

/// Gets a handle to the child window of the parent window specified by `handle` at `point`.
/// The search is restricted to immediate child windows; grandchildren and deeper
/// descendant windows are not searched.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint
///
pub fn real_chold_from_point(handle: isize, point: POINT) -> Result<Option<isize>> {
    call! { RealChildWindowFromPoint(handle, point) != 0 => Result<Option> }
}

/// Minimizes the window specified by `handle`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow
///
pub fn minimize(handle: isize) -> Result<()> {
    call_BOOL! { CloseWindow(handle) }
}

/// Destroys the window specified by `handle`.
/// The function sends [`WM_DESTROY`][super::messaging::WM_DESTROY] and [`WM_NCDESTROY`][super::messaging::WM_NCDESTROY]
/// messages to the window to deactivate it and remove the keyboard focus from it.
/// The function also destroys the window's menu, flushes the thread message queue,
/// destroys timers, removes clipboard ownership, and breaks the clipboard viewer
/// chain (if the window is at the top of the viewer chain).
///
/// If the specified window is a parent or owner window, DestroyWindow automatically
/// destroys the associated child or owned windows when it destroys the parent or
///  owner window. The function first destroys child or owned windows, and then it
/// destroys the parent or owner window.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow
///
pub fn close(handle: isize) -> Result<()> {
    call_BOOL! { DestroyWindow(handle) }
}

/// Changes the title bar text to `text` (if it has one) of the window specified by `handle`.
/// If the window is a control, the text of the control is changed.
/// However, [`set_text`] cannot change the text of a control in another application.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw
///
pub fn set_text(handle: isize, text: &U16CStr) -> Result<()> {
    call_BOOL! { SetWindowTextW(handle, text.as_ptr()) }
}

/// Changes the parent window to `new_parent_handle` of the child window specified by `handle`.
///
/// If `new_parent_handle` is
/// * `0`, the desktop window becomse the new parent.
/// * [`HWND_MESSAGE`], the child window becomse a message-only window.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `child_handle` is invalid.
/// * `new_parent_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent
///
pub fn set_parent(child_handle: isize, new_parent_handle: isize) -> Result<isize> {
    call! { SetParent(child_handle, new_parent_handle) != 0 }
}

/// Sets the show state of the window specified by `handle`.
///
/// | [`SHOW_WINDOW_CMD`] | Description |
/// |------|------|
/// | [`SW_HIDE`] | Hides the window and activates another window. |
/// | [`SW_NORMAL`] | Activates and displays a window. If the window is minimized, maximized, or arranged, the system restores it to its original size and position. An application should specify this flag when displaying the window for the first time. |
/// | [`SW_SHOWMINIMIZED`] | Activates the window and displays it as a minimized window. |
/// | [`SW_MAXIMIZE`] | Activates the window and displays it as a maximized window. |
/// | [`SW_SHOWNOACTIVATE`] | Displays a window in its most recent size and position. This value is similar to [`SW_NORMAL`], except that the window is not activated. |
/// | [`SW_SHOW`] | Activates the window and displays it in its current size and position. |
/// | [`SW_MINIMIZE`] | Minimizes the specified window and activates the next top-level window in the Z order. |
/// | [`SW_SHOWMINNOACTIVE`] | Displays the window as a minimized window. This value is similar to [`SW_SHOWMINIMIZED`], except the window is not activated. |
/// | [`SW_SHOWNA`] | Displays the window in its current size and position. This value is similar to [`SW_SHOW`], except that the window is not activated. |
/// | [`SW_RESTORE`] | Activates and displays the window. If the window is minimized, maximized, or arranged, the system restores it to its original size and position. An application should specify this flag when restoring a minimized window. |
/// | [`SW_SHOWDEFAULT`] | Sets the show state based on the `SW_` value specified in the `STARTUPINFO` structure passed to the `CreateProcess` function by the program that started the application.` |
/// | [`SW_FORCEMINIMIZE`] | Minimizes a window, even if the thread that owns the window is not responding. This flag should only be used when minimizing windows from a different thread. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
///
pub fn show(handle: isize, show_command: SHOW_WINDOW_CMD) -> Result<bool> {
    call_BOOL! { ShowWindow(handle, show_command) -> bool }
}

/// Sets the show state of the window specified by `handle` without waiting for the
/// operation to complete.
///
/// | [`SHOW_WINDOW_CMD`] | Description |
/// |------|------|
/// | [`SW_HIDE`] | Hides the window and activates another window. |
/// | [`SW_NORMAL`] | Activates and displays a window. If the window is minimized, maximized, or arranged, the system restores it to its original size and position. An application should specify this flag when displaying the window for the first time. |
/// | [`SW_SHOWMINIMIZED`] | Activates the window and displays it as a minimized window. |
/// | [`SW_MAXIMIZE`] | Activates the window and displays it as a maximized window. |
/// | [`SW_SHOWNOACTIVATE`] | Displays a window in its most recent size and position. This value is similar to [`SW_NORMAL`], except that the window is not activated. |
/// | [`SW_SHOW`] | Activates the window and displays it in its current size and position. |
/// | [`SW_MINIMIZE`] | Minimizes the specified window and activates the next top-level window in the Z order. |
/// | [`SW_SHOWMINNOACTIVE`] | Displays the window as a minimized window. This value is similar to [`SW_SHOWMINIMIZED`], except the window is not activated. |
/// | [`SW_SHOWNA`] | Displays the window in its current size and position. This value is similar to [`SW_SHOW`], except that the window is not activated. |
/// | [`SW_RESTORE`] | Activates and displays the window. If the window is minimized, maximized, or arranged, the system restores it to its original size and position. An application should specify this flag when restoring a minimized window. |
/// | [`SW_SHOWDEFAULT`] | Sets the show state based on the `SW_` value specified in the `STARTUPINFO` structure passed to the `CreateProcess` function by the program that started the application.` |
/// | [`SW_FORCEMINIMIZE`] | Minimizes a window, even if the thread that owns the window is not responding. This flag should only be used when minimizing windows from a different thread. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync
///
pub fn show_async(handle: isize, show_command: SHOW_WINDOW_CMD) -> Result<()> {
    call_BOOL! { ShowWindowAsync(handle, show_command) }
}

/// Destroys an icon and frees any memory the icon occupied.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `icon_handle` is invalid.
/// * `icon_handle` is in use.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon
///
pub fn destroy_icon(icon_handle: isize) -> Result<()> {
    call_BOOL! { DestroyIcon(icon_handle) }
}
