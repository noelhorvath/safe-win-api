use std::ffi::OsStr;

use crate::common::{As, ConversionError, Point};
use crate::process::Process;
use crate::thread::Thread;
use win_api_wrapper::core::WinError;
use win_api_wrapper::win32::foundation::NULL_HANDLE;
use win_api_wrapper::win32::ui::window::{
    self, CWP_ALL, CWP_FLAGS, GA_ROOT, GA_ROOTOWNER, GW_ENABLEDPOPUP, GW_HWNDFIRST, GW_HWNDLAST,
    GW_HWNDNEXT, GW_HWNDPREV, GW_OWNER, SHOW_WINDOW_CMD, SW_FORCEMINIMIZE, SW_HIDE, SW_MAXIMIZE,
    SW_MINIMIZE, SW_NORMAL, SW_RESTORE, SW_SHOW, SW_SHOWDEFAULT, SW_SHOWMINIMIZED,
    SW_SHOWMINNOACTIVE, SW_SHOWNA, SW_SHOWNOACTIVATE,
};

type Result<T> = ::core::result::Result<T, WindowError>;

pub enum WindowError {
    WinError { code: i32, message: String },
    ConversionError(ConversionError),
}

impl From<WinError> for WindowError {
    fn from(value: WinError) -> Self {
        WindowError::WinError {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

impl From<ConversionError> for WindowError {
    fn from(value: ConversionError) -> Self {
        WindowError::ConversionError(value)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WindowShowState {
    Hide,
    Normal,
    ShowMinimized,
    Maximize,
    ShowNormalAndNotActived,
    Show,
    Minimize,
    ShowMinimizedAndNotActived,
    ShowNotActived,
    Restore,
    #[default]
    ShowDefault,
    ForceMinimize,
}

impl As<u32> for WindowShowState {
    fn as_type(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<SHOW_WINDOW_CMD> for WindowShowState {
    type Error = WindowError;
    fn try_from(value: SHOW_WINDOW_CMD) -> Result<Self> {
        match value {
            SW_HIDE => Ok(Self::Hide),
            SW_NORMAL => Ok(Self::Normal),
            SW_SHOWMINIMIZED => Ok(Self::ShowMinimized),
            SW_MAXIMIZE => Ok(Self::Maximize),
            SW_SHOWNOACTIVATE => Ok(Self::ShowNormalAndNotActived),
            SW_SHOW => Ok(Self::Show),
            SW_MINIMIZE => Ok(Self::Minimize),
            SW_SHOWMINNOACTIVE => Ok(Self::ShowMinimizedAndNotActived),
            SW_SHOWNA => Ok(Self::ShowNotActived),
            SW_RESTORE => Ok(Self::Restore),
            SW_SHOWDEFAULT => Ok(Self::ShowDefault),
            SW_FORCEMINIMIZE => Ok(Self::ForceMinimize),
            _ => Err(ConversionError::OutOfRange.into()),
        }
    }
}

impl From<WindowShowState> for SHOW_WINDOW_CMD {
    fn from(value: WindowShowState) -> Self {
        SHOW_WINDOW_CMD(value.as_type())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Window {
    handle: isize,
}

/*
unsafe extern "system" fn get_windows_callback(handle: HWND, parameter: LPARAM) -> BOOL {
    let sender = (*(parameter.0 as *const Sender<isize>)).clone();
    if let Err(error) = sender.send(handle.0) {
        println!(
            "GetWindowsCallback -> Error sending handle to receiver: {:?}",
            error
        );
    }
    BOOL::from(true)
}

pub fn get_child_windows(parent_handle: isize) -> impl Iterator<Item = isize> {
    let (sender, receiver) = channel::<isize>();
    unsafe {
        EnumChildWindows(
            HWND(parent_handle),
            Some(get_windows_callback),
            LPARAM(&sender as *const Sender<isize> as isize),
        )
    };

    receiver.into_iter()
}

pub fn get_thread_windows(thread_id: u32) -> impl Iterator<Item = isize> {
    let (sender, receiver) = channel::<isize>();
    unsafe {
        EnumThreadWindows(
            thread_id,
            Some(get_windows_callback),
            LPARAM(&sender as *const Sender<isize> as isize),
        )
    };

    receiver.into_iter()
}

pub fn get_windows() -> Result<impl Iterator<Item = isize>> {
    let (sender, receiver) = channel::<isize>();
    unsafe {
        if !EnumWindows(
            Some(get_windows_callback),
            LPARAM(&sender as *const Sender<isize> as isize),
        )
        .as_bool()
        {
            return Err(Error::from_win32());
        }
    };

    Ok(receiver.into_iter())
}
*/

impl Window {
    pub const BROADCAST_WINDOW: Window = Window::new(0xffff);

    pub(crate) const fn new(handle: isize) -> Self {
        Window { handle }
    }

    pub(crate) fn handle(&self) -> isize {
        self.handle
    }

    pub fn from_point(point: Point<i32>) -> Option<Window> {
        window::from_point(point.into()).map(Window::new)
    }

    pub fn from_physical_point(point: Point<i32>) -> Option<Window> {
        window::from_physical_point(point.into()).map(Window::new)
    }

    pub fn top() -> Option<Window> {
        window::get_top_child(NULL_HANDLE).map(Window::new)
    }

    pub fn foreground() -> Option<Window> {
        window::get_foreground().map(Window::new)
    }

    pub fn desktop() -> Option<Window> {
        window::get_desktop_window().map(Window::new)
    }

    pub fn shell() -> Option<Window> {
        window::get_shell_window().map(Window::new)
    }

    pub fn exists(&self) -> bool {
        window::exists(self.handle)
    }

    pub fn root(&self) -> Option<Window> {
        window::get_ancestor(self.handle, GA_ROOT).map(Window::new)
    }

    pub fn root_owner(&self) -> Option<Window> {
        window::get_ancestor(self.handle, GA_ROOTOWNER).map(Window::new)
    }

    pub fn first(&self) -> Option<Window> {
        window::get_window(self.handle, GW_HWNDFIRST).map(Window::new)
    }

    pub fn next(&self) -> Option<Window> {
        window::get_window(self.handle, GW_HWNDNEXT).map(Window::new)
    }

    pub fn previous(&self) -> Option<Window> {
        window::get_window(self.handle, GW_HWNDPREV).map(Window::new)
    }

    pub fn owner(&self) -> Option<Window> {
        window::get_window(self.handle, GW_OWNER).map(Window::new)
    }

    pub fn pop_up(&self) -> Option<Window> {
        window::get_window(self.handle, GW_ENABLEDPOPUP).map(Window::new)
    }

    pub fn last(&self) -> Option<Window> {
        window::get_window(self.handle, GW_HWNDLAST).map(Window::new)
    }

    pub fn parent(&self) -> Option<Window> {
        window::get_paranet(self.handle).map(Window::new)
    }

    pub fn arrange(&self) -> Result<u32> {
        window::arrange_iconic_windows(self.handle).map_err(WindowError::from)
    }

    pub fn children(&self) -> impl Iterator<Item = Window> {
        window::get_child_windows(self.handle).map(Window::new)
    }

    pub fn child_from_point(&self, point: Point<i32>) -> Option<Window> {
        window::child_from_point(self.handle, point.into(), CWP_ALL).map(Window::new)
    }

    pub fn child_from_point_with_flags(
        &self,
        point: Point<i32>,
        flags: CWP_FLAGS,
    ) -> Option<Window> {
        window::child_from_point(self.handle, point.into(), flags).map(Window::new)
    }

    pub fn close(&self) -> Result<()> {
        window::close(self.handle).map_err(WindowError::from)
    }

    pub fn find_child(&self, class_name: Box<OsStr>, title: Box<OsStr>) -> Option<Window> {
        window::find(self.handle, NULL_HANDLE, Some(class_name), Some(title)).map(Window::new)
    }

    pub fn find_child_by_class(&self, class_name: Box<OsStr>) -> Option<Window> {
        window::find(self.handle, NULL_HANDLE, Some(class_name), None).map(Window::new)
    }

    pub fn find_child_by_title(&self, title: Box<OsStr>) -> Option<Window> {
        window::find(self.handle, NULL_HANDLE, None, Some(title)).map(Window::new)
    }

    pub fn find_next_child(
        &self,
        child: Window,
        class_name: Box<OsStr>,
        title: Box<OsStr>,
    ) -> Option<Window> {
        window::find(self.handle, child.handle, Some(class_name), Some(title)).map(Window::new)
    }

    pub fn find_next_child_by_class(
        &self,
        child: Window,
        class_name: Box<OsStr>,
    ) -> Option<Window> {
        window::find(self.handle, child.handle, Some(class_name), None).map(Window::new)
    }

    pub fn top_child(&self) -> Option<Window> {
        window::get_top_child(self.handle).map(Window::new)
    }

    pub fn find_next_child_by_title(&self, child: Window, title: Box<OsStr>) -> Option<Window> {
        window::find(self.handle, child.handle, None, Some(title)).map(Window::new)
    }

    pub fn is_child_of(&self, parent: Window) -> bool {
        window::is_child(self.handle, parent.handle)
    }

    pub fn is_minimized(&self) -> bool {
        window::is_minimized(self.handle)
    }

    pub fn is_maximized(&self) -> bool {
        window::is_maximized(self.handle)
    }

    pub fn logical_to_physical_point(&self, logical_point: Point<i32>) -> Point<i32> {
        window::logical_to_physical_point(self.handle, logical_point.into()).into()
    }

    pub fn minimize(&self) -> Result<()> {
        window::minimize(self.handle).map_err(WindowError::from)
    }

    pub fn open_minimized(&self) -> Result<()> {
        window::open_minimized(self.handle).map_err(WindowError::from)
    }

    pub fn process_and_thread_ids(&self) -> Result<(u32, u32)> {
        window::get_process_and_thread_ids(self.handle).map_err(WindowError::from)
    }

    pub fn set_parent(&self, new_parent: Window) -> Result<Window> {
        window::set_parent(self.handle, new_parent.handle)
            .map(Window::new)
            .map_err(WindowError::from)
    }

    pub fn set_foreground(&self) -> Result<()> {
        window::set_foreground(self.handle).map_err(WindowError::from)
    }

    pub fn set_title(&self, title: Box<OsStr>) -> Result<()> {
        window::set_text(self.handle, title).map_err(WindowError::from)
    }

    pub fn set_show_state(&self, state: WindowShowState) -> bool {
        window::show(self.handle, state.into())
    }

    pub fn set_show_state_async(&self, state: WindowShowState) -> Result<()> {
        window::show_async(self.handle, state.into()).map_err(WindowError::from)
    }
}

pub fn allow_set_foreground(process: Process) -> Result<()> {
    window::allow_set_foreground(process.id()).map_err(WindowError::from)
}

pub fn thread_windows(thread: Thread) -> impl Iterator<Item = Window> {
    window::get_thread_windows(thread.id()).map(Window::new)
}

pub fn windows() -> Result<impl Iterator<Item = Window>> {
    window::get_windows()
        .map(|iter| iter.map(Window::new))
        .map_err(WindowError::from)
}

pub fn find(class_name: Box<OsStr>, title: Box<OsStr>) -> Option<Window> {
    window::find(NULL_HANDLE, NULL_HANDLE, Some(class_name), Some(title)).map(Window::new)
}

pub fn find_by_class(class_name: Box<OsStr>) -> Option<Window> {
    window::find(NULL_HANDLE, NULL_HANDLE, Some(class_name), None).map(Window::new)
}

pub fn find_by_title(title: Box<OsStr>) -> Option<Window> {
    window::find(NULL_HANDLE, NULL_HANDLE, None, Some(title)).map(Window::new)
}
