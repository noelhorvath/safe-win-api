use std::marker::PhantomData;
use win_api_wrapper::core::WinError;
use win_api_wrapper::win32::foundation::close_handle;
use win_api_wrapper::win32::system::diagnostics::toolhelp::{
    self, CREATE_TOOLHELP_SNAPSHOT_FLAGS, PROCESSENTRY32W, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD,
    THREADENTRY32,
};

use crate::process::Process;
use crate::thread::Thread;

fn find_index_of_u16_slice(slice: &[u16], value: u16) -> usize {
    let mut last_index = 0;
    for &item in slice {
        if item == value {
            break;
        }

        last_index += 1;
    }

    last_index
}

impl From<PROCESSENTRY32W> for Process {
    fn from(value: PROCESSENTRY32W) -> Self {
        let last_index = find_index_of_u16_slice(value.szExeFile.as_slice(), '\0' as u16);
        Process::new(
            value.th32ProcessID,
            value.th32ParentProcessID,
            String::from_utf16_lossy(&value.szExeFile[..last_index]).into_boxed_str(),
        )
    }
}

impl From<THREADENTRY32> for Thread {
    fn from(value: THREADENTRY32) -> Self {
        Thread::new(value.th32ThreadID, value.th32OwnerProcessID)
    }
}
