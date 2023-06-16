use crate::core::error::{Code, Error};
use crate::core::Result;
use crate::win32::foundation::get_last_error;
use crate::{call, call_BOOL, default_sized, from_BOOL};
use core::ptr;
use widestring::U16CStr;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::Foundation::{ERROR_SUCCESS, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DefWindowProcW, DispatchMessageW, GetMessageExtraInfo, GetMessagePos,
    GetMessageTime, GetMessageW, InSendMessageEx, PeekMessageW, PostMessageW, PostQuitMessage,
    PostThreadMessageW, RegisterWindowMessageW, ReplyMessage, SendMessageCallbackW,
    SendMessageTimeoutW, SendMessageW, SendNotifyMessageW, SetMessageExtraInfo, SetWindowsHookExW,
    TranslateAcceleratorW, TranslateMessage, UnhookWindowsHookEx, WaitMessage,
};

pub use windows_sys::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, ISMEX_NOSEND, ISMEX_REPLIED, ISMEX_SEND, MSG,
    PEEK_MESSAGE_REMOVE_TYPE as PeekMessageRemoveType,
    SEND_MESSAGE_TIMEOUT_FLAGS as SendMessageTimoutFlags, SMTO_ABORTIFHUNG, SMTO_BLOCK,
    SMTO_ERRORONEXIT, SMTO_NORMAL, SMTO_NOTIMEOUTIFNOTHUNG, WH_CALLWNDPROC, WH_CALLWNDPROCRET,
    WH_CBT, WH_DEBUG, WH_FOREGROUNDIDLE, WH_GETMESSAGE, WH_KEYBOARD, WH_KEYBOARD_LL, WH_MOUSE,
    WH_MOUSE_LL, WH_MSGFILTER, WH_SHELL, WH_SYSMSGFILTER, WINDOWS_HOOK_ID, WM_CHAR, WM_COMMAND,
    WM_DEADCHAR, WM_DESTROY, WM_DROPFILES, WM_INPUT, WM_KEYDOWN, WM_KEYFIRST, WM_KEYLAST, WM_KEYUP,
    WM_MOUSEFIRST, WM_MOUSELAST, WM_NCDESTROY, WM_PAINT, WM_QUIT, WM_SETTINGCHANGE, WM_SYSCHAR,
    WM_SYSCOMMAND, WM_SYSDEADCHAR, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TIMER, WM_USER,
};

/// A hook procedure that can be passed to [`set_hook`].
pub type HookProcedure =
    unsafe extern "system" fn(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

/// A callback function that can be passed to [`send_with_callback`].
pub type SendAsyncCallback =
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: LRESULT) -> ();

#[allow(clippy::undocumented_unsafe_blocks)]
/// Passes the hook `hook_code` and `parameters` to the next hook procedure in the current hook chain.
/// A hook procedure can call this function either before or after processing the hook information.
///
/// Calling [`call_next_hook`] ensures that other applications receive hook notifications.
/// Therefore this function should not be called if one wants to prevent notifications from being seen by other applications.
///
/// # Arguments
///
/// * `hook_code`: The hook code passed to the current hook procedure.
/// * `parameters`: Tuple of `(usize, isize)` parameters passed to the current hook procedure.
///     * The meaning of each parameter depends on the type of hook associated with the current hook chain.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex
///
pub fn call_next_hook(hook_code: i32, parameters: (usize, isize)) -> isize {
    unsafe {
        CallNextHookEx(
            0, // ignored
            hook_code,
            parameters.0,
            parameters.1,
        )
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Dispatches `message` to a window procedure. It is tipically used to dispatch a message retrieved by [`get`].
///
/// # Remarks
///
/// * `message` must containt valid message values.
/// * If `message` is a [`WM_TIMER`] message and the its `lParam` field is `0`, `lParam`
///   points to a function that is called instead of the window procedure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew
///
pub fn dispatch(message: &MSG) -> isize {
    unsafe { DispatchMessageW(message) }
}

/// Gets a message from the calling thread's message queue, that is associated with the window identified
/// by `window_handle` or any of its children, as specified by [`is_child`][crate::win32::ui::window::is_child].
/// The function dispatches incoming sent messages until a posted message is available for retrieval.
/// Unlike [`get`], [`peek`] does not wait for a message to be posted before returning.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window, that belongs to the current thread, whose messages are to be retrieved.
/// * `message_filter_min`: The integer value of the lowest message value to be retrieved.
/// * `message_filter_max`: The integer value of the highest message value to be retrieved.
///
/// The `window_handle` parameter can accept values that have special meaning, such as:
/// * `0`: [`get`] retrieves messages for any window that belongs to the current thread,
///   and any messages on the current thread's message queue whose hwnd value is `0`.
/// * `-1`: [`get`] retrieves only messages on the current thread's message queue whose `hwnd` (window handle)
///   value is `0`, that is, thread messages as posted by [`post`] (when the `window_handle` parameter is `0`) or [`post_thread`].
///
/// ## Message filter ranges
///
/// | Message value | Min filter | Max filter |
/// | -------- | -------- | -------- |
/// | Keyboard input | [`WM_KEYFIRST`] | [`WM_KEYLAST`] |
/// | Mouse input | [`WM_MOUSEFIRST`] | [`WM_MOUSELAST`] |
/// | All | `0` | `0` |
/// | `*X` | `X` | `X` |
///
/// `*X`: An arbitrary message filter like `WM_INPUT` or `WM_KEYFIRST`.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew
///
pub fn get(window_handle: isize, message_filter_min: u32, message_filter_max: u32) -> Result<MSG> {
    call! {
        GetMessageW(
            &mut message,
            window_handle,
            message_filter_min,
            message_filter_max,
        ) != -1 => mut message = default_sized!(MSG)
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the message time for the last message retrieved by [`get`].
/// The returned `i32` value specifies the elapsed time, in milliseconds,
/// from the time the system was started to the time the message was created
/// (that is, placed in the thread's message queue).
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagetime
///
pub fn get_last_time() -> i32 {
    unsafe { GetMessageTime() }
}

/// Gets the cursor position for the last message retrieved by [`get`].
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagepos
///
pub fn get_last_cursor_position() -> POINT {
    let result = unsafe { GetMessagePos() };
    POINT {
        x: (result >> u16::BITS) as i32,
        y: result as i32,
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the extra message information for the current thread.
/// Extra message information is an application- or driver-defined value associated with the current thread's message queue.
///
/// [`set_extra_info`] can be used to set a thread's extra message information.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessageextrainfo
///
pub fn get_extra_info() -> isize {
    unsafe { GetMessageExtraInfo() }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Sets the extra message information for the current thread.
/// Extra message information is an application- or driver-defined value associated with the current thread's message queue.
///
/// [`get_extra_info`] can be used to get a thread's extra message information.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmessageextrainfo
///
pub fn set_extra_info(extra_info: isize) -> isize {
    unsafe { SetMessageExtraInfo(extra_info) }
}

/// Dispatches incoming nonqueued messages, checks the thread message queue for a posted message, and gets
/// the message associated with the window specified by `window_handle` or any of its children,
/// as specified by [`is_child`][crate::win32::ui::window::is_child]. If there's no available message, the return value is `None`.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window, that belongs to the current thread, whose messages are to be retrieved.
/// * `message_filter_min`: The integer value of the lowest message value to be retrieved.
/// * `message_filter_max`: The integer value of the highest message value to be retrieved.
/// * `message_remove_type`: Specifies how messages are to be handled.
///
/// The `window_handle` parameter can accept values that have special meaning, such as:
/// * `0`  - The function retrieves messages for any window that belongs to the current thread,
///          and any messages on the current thread's message queue whose hwnd value is `0`.
/// * `-1` - The function retrieves only messages on the current thread's message queue whose `hwnd` (window handle)
///          value is `0`, that is, thread messages as posted by [`post`] (when the `window_handle` parameter is `0`)
///          or [`post_thread`].
///
/// ## Message filter ranges
///
/// | Message value | Min filter | Max filter |
/// | -------- | -------- | -------- |
/// | Keyboard input | [`WM_KEYFIRST`] | [`WM_KEYLAST`] |
/// | Mouse input | [`WM_MOUSEFIRST`] | [`WM_MOUSELAST`] |
/// | All | `0` | `0` |
/// | `*X` | `X` | `X` |
///
/// `*X`: An arbitrary message filter like `WM_INPUT` or `WM_KEYFIRST`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew
///
pub fn peek(
    window_handle: isize,
    filter_min: u32,
    filter_max: u32,
    message_remove_type: PeekMessageRemoveType,
) -> Option<MSG> {
    call_BOOL! {
        PeekMessageW(
            &mut message,
            window_handle,
            filter_min,
            filter_max,
            message_remove_type,
        ) -> Option { mut message = default_sized!(MSG) }
    }
}

/// Places a message in the message queue associated with the thread that created the window specified
/// by `window_handle` and returns without waiting for the thread to process the message.
/// The posted messages can be rettrieved in a message queue by [`get`] or [`peek`].
///
/// Use [`post_thread`] to post a message in the message queue associated with a thread.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
///
/// The `window_handle` parameter can accept values that have special meaning, such as:
/// * `0xFFFF` - The message is posted to all top-level windows in the system, including disabled or invisible unowned windows,
///   overlapped windows, and pop-up windows. The message is not posted to child windows.
/// * `0`      - The function behaves like a call to [`post_thread`] with `thread_id` parameter set to the id of the current thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `window_handle` is invalid.
/// * [`ERROR_ACCESS_DENIED`][windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED]
///     * Message is blocked by `UIPI`.
///
/// # Remarks
///
/// * Starting with `Windows Vista`, message posting is subject to `UIPI`.
///   The thread of a process can post messages only to message queues of threads
///   in processes of lesser or equal integrity level.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew
///
pub fn post(window_handle: isize, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        PostMessageW(window_handle, message, parameters.0, parameters.1)
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Posts a [`WM_QUIT`] message, which is is typically used in response to a [`WM_DESTROY`] message,
/// to the thread's message queue indicating to the system
/// that the calling thread has made a request to terminate at some time in the future.
/// When the thread retrieves the [`WM_QUIT`] message from its message queue,
/// it should exit its message loop and return control to the system.
/// The exit value specified by `exit_code` will be the `wParam` parameter of the [`WM_QUIT`] message.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage
///
pub fn post_quit(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) }
}

/// Posts a message to the message queue of the thread specified by `thread_id`.
/// It returns without waiting for the thread to process the message.
///
/// Use [`get`] or [`peek`] to get the posted thread message. Thread messages have their `hwnd` field set to `0`.
///
/// # Arguments
///
/// * `thread_id`: The identifier of the thread to which the message is to be posted.
/// * `message`: The type of message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * The thread specified by `thread_id` doesn't have a message queue.
/// * [`ERROR_INVALID_THREAD_ID`][windows_sys::Win32::Foundation::ERROR_INVALID_THREAD_ID]
///     * `thread_id` is not a valid thread identifier.  
///     * The thread identified by `thread_id` doesn't have the [`SE_TCB_NAME`][windows_sys::Win32::Security::SE_TCB_NAME]
///       privilege to post a message to a thread that belongs to a process with the same locally unique identifier (`LUID`)
///       but is in a different desktop.
///     * The thread identified by `thread_id` doesn't belong to the same desktop as the calling thread or to a process
///       of the same `LUID`.
/// * [`ERROR_NOT_ENOUGH_QUOTA`][windows_sys::Win32::Foundation::ERROR_NOT_ENOUGH_QUOTA]
///     * Message limit has been hit.
/// * [`ERROR_ACCESS_DENIED`][windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED]
///     * The posted message is blocked by `UIPI`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew
///
pub fn post_thread(thread_id: u32, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        PostThreadMessageW(thread_id, message, parameters.0, parameters.1)
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Replies to a message sent from another thread by [`send`].
/// The return values specifies whether the calling thread was processing a message sent from another thread or process.
///
/// By calling this function, the window procedure that receives the message allows the thread
/// that called [`send`] to continue to run as though the thread receiving the message had returned control.
/// The thread that calls the [`reply`] function also continues to run.
///
/// If the message was not sent through [`send`] or if the message was sent by the same thread,
/// [`reply`] has no effect.
///
/// # Examples
/// [TODO][https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues]
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-replymessage
///
pub fn reply(processing_result: isize) -> bool {
    from_BOOL!(unsafe { ReplyMessage(processing_result) })
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Sends `message` with `parameters` to a window or windows specified by `window_handle`.
/// The [`send`] call the window procedure for the specified windows and oes not return until the window
/// procedure has processed the message.
///
/// Use [`send_with_callback`] or [`send_notify`] to send a message and return immediately.
///
/// To post a message to a thread's message queue and return immediately, use [`post`] or [`post_thread`].
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
///
/// # Remarks
///
/// If `window_handle` is `0xFFFF` (`HWND_BROADCAST`) the message is sent to all top-level windows in the system, including disabled or
/// invisible unowned windoes, overlapped windows, and pop-up windows, but the message is not sent to child windows.
/// * Use [`register_window_message`] to obtain a unique message for inter-application communication.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `window_handle` is invalid.
/// * [`ERROR_ACCESS_DENIED`][windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED]
///     * The posted message is blocked by `UIPI`.
///
/// # Examples
/// [TODO][https://learn.microsoft.com/en-us/windows/win32/inputdev/using-keyboard-input]
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew
///
pub fn send(window_handle: isize, message: u32, parameters: (usize, isize)) -> Result<isize> {
    let result = unsafe { SendMessageW(window_handle, message, parameters.0, parameters.1) };
    // checking for possible errors
    let last_error_code = get_last_error();
    if last_error_code == ERROR_SUCCESS {
        Ok(result)
    } else {
        Err(Error::from_code(Code::from_win32(last_error_code)))
    }
}

/// Sends `message` and `parameters` to a window or windows. If the window was created by the calling thread,
/// [`send_notify`] calls the window procedure for the window and does not return until the window procedure
/// has processed the message. If the window was created by a different thread,
/// [`send_notify`] passes the message to the window procedure and returns immediately;
/// it does not wait for the window procedure to finish processing the message.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
///
/// # Remarks
///
/// If `window_handle` is `0xFFFF` (`HWND_BROADCAST`) the message is sent to all top-level windows in the system, including disabled or
/// invisible unowned windoes, overlapped windows, and pop-up windows, but the message is not sent to child windows.
/// * Use [`register_window_message`] to obtain a unique message for inter-application communication.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendnotifymessagew
///
pub fn send_notify(window_handle: isize, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        SendNotifyMessageW(window_handle, message, parameters.0, parameters.1)
    }
}

/// Sends `message` and `parameters` to one or more windows.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
/// * `timeout_flags`: The behavior of this function.
/// * `timeout`: The duration of the time-out period, in milliseconds.
///
/// # Remarks
///
/// If `window_handle` is `0xFFFF` (`HWND_BROADCAST`) the message is sent to all top-level windows in the system, including disabled or
/// invisible unowned windoes, overlapped windows, and pop-up windows, but the message is not sent to child windows.
/// * Use [`register_window_message`] to obtain a unique message for inter-application communication.
///
/// The function does not return until each window has timed out.
/// Therefore, the total wait time can be up to the value of `timeout` multiplied by the number of top-level windows.
///
/// Each window can use the full time-out period specified by `timeout`.
/// For example, if you specify a five second time-out period and there are three top-level windows
/// that fail to process the message, you could have up to a `15` second delay.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `window_handle` is invalid.
/// * [`ERROR_TIMEOUT`][windows_sys::Win32::Foundation::ERROR_TIMEOUT]
///     * The function timed out. If `0xFFFF` is passed as `window_handle` the function doesn't return timeout error.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw
///
pub fn send_with_timeout(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
    timeout_flags: SendMessageTimoutFlags,
    timeout: u32,
) -> Result<usize> {
    call! {
        SendMessageTimeoutW(
            window_handle,
            message,
            parameters.0,
            parameters.1,
            timeout_flags,
            timeout,
            &mut processing_result,
        ) != 0 => mut processing_result: usize
    }
}

/// Sends `message` and `parameters` to a window or windows specified by `window_handle`. It calls the window procedure
/// for the specified window and returns immediately if the window belongs to another thread.
/// After the window procedure processes the message, the system calls `callback`,
/// passing the result of the message processing and an application-defined value to the function.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
/// * `callback`: A function pointer to a callback function that the system calls after the window procedure
///               processes the message.
/// * `callback_parameter`: An application-defined value to be sent to the callback function
///                         pointed to by `callback`.
///
/// If `window_handle` is `0xFFFF` (`HWND_BROADCAST`):
///     
/// * The message is sent to all top-level windows in the system, including disabled or
///   invisible unowned windoes, overlapped windows, and pop-up windows, but the message is not sent to child windows.
///     * Use [`register_window_message`] to obtain a unique message for inter-application communication.
/// * The system calls the callback function once for each top-level window.
///
/// # Remarks
///
/// If the target window belongs to the same thread as the caller, then the window procedure
/// is called synchronously, and the callback function specified by `callback` is called immediately
/// after the window procedure returns. If the target window belongs to a different thread from the caller, then `callback`
/// is called only when the thread that called [`send_with_callback`] also calls [`get`], [`peek`], or [`wait`].
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagecallbackw
///
pub fn send_with_callback(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
    callback: SendAsyncCallback,
    callback_parameter: usize,
) -> Result<()> {
    call_BOOL! {
        SendMessageCallbackW(
            window_handle,
            message,
            parameters.0,
            parameters.1,
            Some(callback),
            callback_parameter,
        )
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
#[inline]
/// Determines whether the current window procedure is processing a message
/// that was sent from another thread in the same process or a different process.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex
///
pub fn in_send() -> u32 {
    unsafe { InSendMessageEx(ptr::null()) }
}

#[inline]
/// Determines whether the sender is blocked.
///
/// # Examples
/// TODO
///
pub fn is_sender_blocked() -> bool {
    in_send() & (ISMEX_REPLIED | ISMEX_SEND) == ISMEX_SEND
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Translates virtual-key messages into character messages.
/// The character messages are posted to the calling thread's message queue,
/// to be read the next time the thread calls the [`get`] or [`peek`] function.
/// The function does not modify the `message` parameter.
///
/// The function returns `true` if the message is translated or if the message is
/// [`WM_KEYDOWN`], [`WM_KEYUP`], [`WM_SYSKEYDOWN`], or [`WM_SYSKEYUP`].
/// If the message is not translated (that is, a character message is not posted to the thread's message queue),
/// the return value is `false`.
///
/// [`WM_KEYDOWN`] and [`WM_KEYUP`] combinations produce a [`WM_CHAR`] or [`WM_DEADCHAR`] message.
/// [`WM_SYSKEYDOWN`] and [`WM_SYSKEYUP`] combinations produce a [`WM_SYSCHAR`] or [`WM_SYSDEADCHAR`] message.
/// [`translate`] produces [`WM_CHAR`] messages only for keys that are mapped to ASCII characters by the keyboard driver.
///
/// If applications process virtual-key messages for some other purpose,
/// they should not call [`translate`].
/// For instance, an application should not call [`translate`] if the [`translate_accelerator`]
/// function succeeds.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage
///
pub fn translate(message: MSG) -> bool {
    from_BOOL!(unsafe { TranslateMessage(&message) })
}

/// Processes accelerator keys for menu commands.
/// The function translates a [`WM_KEYDOWN`] or [`WM_SYSKEYDOWN`] message to a [`WM_COMMAND`] or [`WM_SYSCOMMAND`]
/// message (if there is an entry for the key in the specified accelerator table) and then sends the [`WM_COMMAND`]
/// or [`WM_SYSCOMMAND`] message directly to the specified window procedure. TranslateAccelerator does not return
/// until the window procedure has processed the message.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw
///
pub fn translate_accelerator(
    window_handle: isize,
    accelerator_table_handle: isize,
    message: MSG,
) -> Result<i32> {
    call! {
        TranslateAcceleratorW(window_handle, accelerator_table_handle, &message) != 0
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Calls the default window procedure to provide default processing for any window messages that an application
/// does not process. This function ensures that every message is processed. [`use_default_procedure`] is called
/// with the same parameters received by the window procedure.
///
/// The return value is the result of the message processing and depends on the message.
///
/// # Arguments
///
/// * `window_handle`: A handle to the window whose window procedure is to receive the message.
/// * `message`: The message to be posted.
/// * `parameters`: Each tuple value is an additional message-specific information.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw
///
pub fn use_default_procedure(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
) -> isize {
    unsafe { DefWindowProcW(window_handle, message, parameters.0, parameters.1) }
}

/// Installs an application-defined hook procedure into a hook chain.
/// You would install a hook procedure to monitor the system for certain types of events.
/// These events are associated either with a specific thread or with all threads in the
/// same desktop as the calling thread.
///
/// The return value is the result of the message processing and depends on the message.
///
/// # Arguments
///
/// * `hook_id`: The type of hook procedure to be installed.
/// * `hook_procedure`: A pointer to the hook procedure.
///     * If the `thread_id` parameter is zero or specifies the identifier of a thread created by a different process,
///       the `hook_procedure` parameter must point to a hook procedure in a DLL.
///       Otherwise, `hook_procedure` can point to a hook procedure in the code associated with the current process.
/// * `module_handle`: A handle to the DLL containing the hook procedure pointed to by the `hook_procedure` parameter.
///     * The `module_handle` parameter must be set to `0` if `thread_id` specifies a thread created by the current
///       process and if `hook_procedure` is within the code associated with the current process.
/// * `thread_id`: The identifier of the thread with which the hook procedure is to be associated.
///     * For desktop apps, if this `thread_id` is `0`, `hook_procedure` is associated with all existing threads running
///       in the same desktop as the calling thread.
///       For Windows Store apps, see the Remarks section in the [documentation].
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw
///
pub fn set_hook(
    hook_id: WINDOWS_HOOK_ID,
    hook_procedure: HookProcedure,
    module_handle: isize,
    thread_id: u32,
) -> Result<isize> {
    call! {
        SetWindowsHookExW(
            hook_id,
            Some(hook_procedure),
            module_handle,
            thread_id,
        ) != 0
    }
}

/// Removes a hook procedure installed in a hook chain by the [`set_hook`].
///
/// The hook procedure can be in the state of being called by another thread even after [`unhook`] returns.
/// If the hook procedure is not being called concurrently, the hook procedure is removed immediately before
/// [`unhook`] returns.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `hook_handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex
///
pub fn unhook(hook_handle: isize) -> Result<()> {
    call_BOOL! { UnhookWindowsHookEx(hook_handle) }
}

/// Yields control to other threads when a thread has no other messages in its message queue.
/// The function suspends the thread and does not return until a new message is placed in the thread's message queue.
///
/// [`wait`] does not return if there is unread input in the message queue after the thread has called a function
/// to check the queue. A subsequent call to the function will not return until new input of the specified type arrive
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage
///
pub fn wait() -> Result<()> {
    call_BOOL! { WaitMessage() }
}

/// Defines a new window message that is guaranteed to be unique throughout the system.
/// The message value can be used when sending or posting messages.
///
/// If the message is successfully registered, the return value contains the message identifier in the
/// range `0xC000` through `0xFFFF`
///
/// Only use [`register_window_message`] when more than one application must process the same message.
/// For sending private messages within a window class, an application can use any integer in the
/// range [`WM_USER`] through `0x7FFF`.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerwindowmessagew
///
pub fn register_window_message(message: &U16CStr) -> Result<u32> {
    call! { RegisterWindowMessageW(message.as_ptr()) != 0 }
}
