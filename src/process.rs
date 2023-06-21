use super::common::{As, ConversionError};
use crate::common::{MemoryPriority, PowerThrottlingState};
use crate::diagnostics::{DiagnosticsError, Snapshot};
use crate::kernel::ProcessorIndex;
use std::fmt::Debug;
use std::path::Path;
use std::time::SystemTime;
use win_api_wrapper::core::WinError;
use win_api_wrapper::win32::foundation;
use win_api_wrapper::win32::system::process_status;
use win_api_wrapper::win32::system::threading::process;

use win_api_wrapper::win32::system::threading::process::access::{
    ProcessAccessRights, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION,
    PROCESS_SET_INFORMATION, PROCESS_SET_LIMITED_INFORMATION, PROCESS_SET_QUOTA,
    PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
};
use win_api_wrapper::win32::system::threading::process::{
    ABOVE_NORMAL_PRIORITY_CLASS, APP_MEMORY_INFORMATION, BELOW_NORMAL_PRIORITY_CLASS,
    HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, IO_COUNTERS, MEMORY_PRIORITY_INFORMATION,
    NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS, PROCESS_LEAP_SECOND_INFO,
    PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND, PROCESS_MODE_BACKGROUND_BEGIN,
    PROCESS_MODE_BACKGROUND_END, PROCESS_NAME_FORMAT, PROCESS_NAME_NATIVE, PROCESS_NAME_WIN32,
    PROCESS_POWER_THROTTLING_STATE, PROCESS_PROTECTION_LEVEL_INFORMATION,
    PROTECTION_LEVEL_ANTIMALWARE_LIGHT, PROTECTION_LEVEL_AUTHENTICODE,
    PROTECTION_LEVEL_CODEGEN_LIGHT, PROTECTION_LEVEL_LSA_LIGHT, PROTECTION_LEVEL_NONE,
    PROTECTION_LEVEL_PPL_APP, PROTECTION_LEVEL_WINDOWS, PROTECTION_LEVEL_WINDOWS_LIGHT,
    PROTECTION_LEVEL_WINTCB, PROTECTION_LEVEL_WINTCB_LIGHT, REALTIME_PRIORITY_CLASS, STILL_ACTIVE,
};
use win_api_wrapper::win32::system::time::{file_time_to_std_system_time, FILETIME};

type Result<T> = core::result::Result<T, ProcessError>;

// TODO: Proc groups? move const to struct impl
const MAX_PROCESSOR_COUNT_PER_GROUP: u32 = usize::BITS;

#[derive(Debug)]
pub enum ProcessError {
    WinError { code: i32, message: String },
    AffinityInfoError(AffinityInfoError),
    ConversionError(ConversionError),
    DiagnosticsError(DiagnosticsError),
}

impl ::core::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WinError { code, message } => write!(f, "WinApi error: {} ({:X})", message, code),
            Self::AffinityInfoError(error) => write!(f, "AffinityInfo error: {:?}", error), // TODO: impl Display for AffinityMask
            Self::ConversionError(error) => write!(f, "Conversion error: {:?}", error), // TODO: impl Display for ConversionError
            Self::DiagnosticsError(error) => write!(f, "Diagnostics error: {:?}", error), // TODO: impl Display for ConversionError
        }
    }
}

impl From<WinError> for ProcessError {
    fn from(value: WinError) -> Self {
        ProcessError::WinError {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

impl From<AffinityInfoError> for ProcessError {
    fn from(value: AffinityInfoError) -> Self {
        ProcessError::AffinityInfoError(value)
    }
}

impl From<ConversionError> for ProcessError {
    fn from(value: ConversionError) -> Self {
        ProcessError::ConversionError(value)
    }
}

impl From<DiagnosticsError> for ProcessError {
    fn from(value: DiagnosticsError) -> Self {
        ProcessError::DiagnosticsError(value)
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum AffinityInfoError {
    InsufficientBuffer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AffinityInfo {
    system_mask: usize,
    process_mask: usize,
}

impl AffinityInfo {
    fn new(system_mask: usize, process_mask: usize) -> Self {
        AffinityInfo {
            system_mask,
            process_mask,
        }
    }

    pub fn is_processor_enabled(&self, index: ProcessorIndex) -> Option<bool> {
        if !self.is_valid_processor(index) {
            return None;
        }

        Some(self.process_mask & (1 << index.as_type()) == index.mask())
    }

    pub fn is_valid_processor(&self, index: ProcessorIndex) -> bool {
        self.system_mask & (1 << index.as_type()) == index.mask()
    }

    pub fn processor_count(&self) -> u8 {
        self.process_mask.ilog2() as u8
    }

    pub fn highest_processor(&self) -> Result<ProcessorIndex> {
        ProcessorIndex::try_from(self.processor_count()).map_err(ProcessError::from)
    }

    pub fn list(&self, buffer: &mut [bool]) -> Result<usize> {
        let processor_count = self.processor_count() as usize;
        if buffer.len() < processor_count {
            return Err(AffinityInfoError::InsufficientBuffer.into());
        }

        for (i, is_enabled) in buffer.iter_mut().enumerate() {
            *is_enabled = self.process_mask & (1 << i) != 0;
        }

        Ok(processor_count)
    }
}

impl From<(usize, usize)> for AffinityInfo {
    fn from(value: (usize, usize)) -> Self {
        AffinityInfo::new(value.0, value.1)
    }
}

impl ::core::fmt::Display for AffinityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "System processor affinity mask: {:b}", self.system_mask);
        let _ = writeln!(
            f,
            "Process processor affinity mask: {:b}",
            self.process_mask
        );

        let mut buffer = [false; MAX_PROCESSOR_COUNT_PER_GROUP as usize];
        let buffer_position = unsafe { self.list(buffer.as_mut_slice()).unwrap_unchecked() };

        for (i, &is_enabled) in buffer[..buffer_position].iter().enumerate() {
            let _ = writeln!(
                f,
                "{}: {}",
                unsafe { ProcessorIndex::try_from(i as u8).unwrap_unchecked() },
                if is_enabled { 1 } else { 0 }
            );
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IoCounters {
    read_operation_count: u64,
    write_operation_count: u64,
    other_operation_count: u64,
    read_transfer_count: u64,
    write_transfer_count: u64,
    other_transfer_count: u64,
}

impl IoCounters {
    pub fn new(
        read_operation_count: u64,
        write_operation_count: u64,
        other_operation_count: u64,
        read_transfer_count: u64,
        write_transfer_count: u64,
        other_transfer_count: u64,
    ) -> Self {
        IoCounters {
            read_operation_count,
            write_operation_count,
            other_operation_count,
            read_transfer_count,
            write_transfer_count,
            other_transfer_count,
        }
    }

    pub fn read_operation_count(&self) -> u64 {
        self.read_operation_count
    }

    pub fn write_operation_count(&self) -> u64 {
        self.write_operation_count
    }

    pub fn other_operation_count(&self) -> u64 {
        self.other_operation_count
    }

    pub fn read_transfer_count(&self) -> u64 {
        self.read_transfer_count
    }

    pub fn write_transfer_count(&self) -> u64 {
        self.write_transfer_count
    }

    pub fn other_transfer_count(&self) -> u64 {
        self.other_transfer_count
    }
}

impl ::core::ops::Add for IoCounters {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        IoCounters::new(
            self.read_operation_count + rhs.read_operation_count,
            self.write_operation_count + rhs.write_operation_count,
            self.other_operation_count + rhs.other_operation_count,
            self.read_transfer_count + rhs.read_transfer_count,
            self.write_transfer_count + rhs.write_transfer_count,
            self.other_transfer_count + rhs.other_transfer_count,
        )
    }
}

impl ::core::ops::AddAssign for IoCounters {
    fn add_assign(&mut self, rhs: Self) {
        self.read_operation_count += rhs.read_operation_count;
        self.write_operation_count += rhs.write_operation_count;
        self.other_operation_count += rhs.other_operation_count;
        self.read_transfer_count += rhs.read_transfer_count;
        self.write_transfer_count += rhs.write_transfer_count;
        self.other_transfer_count += rhs.other_transfer_count;
    }
}

impl ::core::ops::Sub for IoCounters {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        IoCounters::new(
            self.read_operation_count - rhs.read_operation_count,
            self.write_operation_count - rhs.write_operation_count,
            self.other_operation_count - rhs.other_operation_count,
            self.read_transfer_count - rhs.read_transfer_count,
            self.write_transfer_count - rhs.write_transfer_count,
            self.other_transfer_count - rhs.other_transfer_count,
        )
    }
}

impl ::core::ops::SubAssign for IoCounters {
    fn sub_assign(&mut self, rhs: Self) {
        self.read_operation_count -= rhs.read_operation_count;
        self.write_operation_count -= rhs.write_operation_count;
        self.other_operation_count -= rhs.other_operation_count;
        self.read_transfer_count -= rhs.read_transfer_count;
        self.write_transfer_count -= rhs.write_transfer_count;
        self.other_transfer_count -= rhs.other_transfer_count;
    }
}

impl From<IO_COUNTERS> for IoCounters {
    fn from(value: IO_COUNTERS) -> Self {
        IoCounters::new(
            value.ReadOperationCount,
            value.WriteOperationCount,
            value.OtherOperationCount,
            value.ReadTransferCount,
            value.WriteTransferCount,
            value.OtherTransferCount,
        )
    }
}

impl From<(u64, u64, u64, u64, u64, u64)> for IoCounters {
    fn from(value: (u64, u64, u64, u64, u64, u64)) -> Self {
        IoCounters::new(value.0, value.1, value.2, value.3, value.4, value.5)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct LeapSecondInfo {
    flags: u32,
    reserved: u32,
}

impl LeapSecondInfo {
    fn new(flags: u32) -> Self {
        LeapSecondInfo {
            flags,
            ..Default::default()
        }
    }

    pub fn is_sixty_second_enabled(&self) -> bool {
        self.flags & PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND
            == PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND
    }
}

impl From<LeapSecondInfo> for PROCESS_LEAP_SECOND_INFO {
    fn from(value: LeapSecondInfo) -> PROCESS_LEAP_SECOND_INFO {
        PROCESS_LEAP_SECOND_INFO {
            Flags: value.flags,
            Reserved: value.reserved,
        }
    }
}

impl From<PROCESS_LEAP_SECOND_INFO> for LeapSecondInfo {
    fn from(value: PROCESS_LEAP_SECOND_INFO) -> Self {
        LeapSecondInfo::new(value.Flags)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct MemoryUsage {
    available: u64,
    peak_private_usage: u64,
    private_usage: u64,
    total_usage: u64,
}

impl MemoryUsage {
    const fn new(
        available: u64,
        peak_private_usage: u64,
        private_usage: u64,
        total_usage: u64,
    ) -> Self {
        MemoryUsage {
            available,
            peak_private_usage,
            private_usage,
            total_usage,
        }
    }

    pub fn available(&self) -> u64 {
        self.available
    }

    pub fn peak_private_usage(&self) -> u64 {
        self.peak_private_usage
    }

    pub fn private_usage(&self) -> u64 {
        self.private_usage
    }

    pub fn total_usage(&self) -> u64 {
        self.total_usage
    }

    pub fn to_chart_string(&self) -> String {
        format!(
            r#"
            Memory usage
            Available:      {}
            Peak private:   {}
            Private:        {}
            Total:          {}
            "#,
            self.available, self.peak_private_usage, self.private_usage, self.total_usage,
        )
    }
}

//pub enum MemoryUsageUnit { } TODO: units and better display for mem

impl ::core::fmt::Display for MemoryUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_chart_string())
    }
}

impl From<APP_MEMORY_INFORMATION> for MemoryUsage {
    fn from(value: APP_MEMORY_INFORMATION) -> Self {
        MemoryUsage::new(
            value.AvailableCommit,
            value.PeakPrivateCommitUsage,
            value.PrivateCommitUsage,
            value.TotalCommitUsage,
        )
    }
}

#[repr(u32)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ProcessPriorityClass {
    Idle = IDLE_PRIORITY_CLASS.0,
    BelowNormal = BELOW_NORMAL_PRIORITY_CLASS.0,
    #[default]
    Normal = NORMAL_PRIORITY_CLASS.0,
    AboveNormal = ABOVE_NORMAL_PRIORITY_CLASS.0,
    High = HIGH_PRIORITY_CLASS.0,
    Realtime = REALTIME_PRIORITY_CLASS.0,
    BackgroundModeBegin = PROCESS_MODE_BACKGROUND_BEGIN.0,
    BackgroundModeEnd = PROCESS_MODE_BACKGROUND_END.0,
}

impl ::core::fmt::Display for ProcessPriorityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Idle => write!(f, "Idle"),
            Self::High => write!(f, "High"),
            Self::Realtime => write!(f, "Realtime"),
            Self::BelowNormal => write!(f, "Below normal"),
            Self::AboveNormal => write!(f, "Above normal"),
            Self::BackgroundModeBegin => write!(f, "Background mode begin"),
            Self::BackgroundModeEnd => write!(f, "Background mode end"),
        }
    }
}

impl As<u32> for ProcessPriorityClass {
    fn as_type(&self) -> u32 {
        *self as u32
    }
}

impl From<ProcessPriorityClass> for PROCESS_CREATION_FLAGS {
    fn from(value: ProcessPriorityClass) -> PROCESS_CREATION_FLAGS {
        PROCESS_CREATION_FLAGS(value.as_type())
    }
}

impl TryFrom<PROCESS_CREATION_FLAGS> for ProcessPriorityClass {
    type Error = ConversionError;
    fn try_from(value: PROCESS_CREATION_FLAGS) -> ::core::result::Result<Self, Self::Error> {
        match value {
            NORMAL_PRIORITY_CLASS => Ok(Self::Normal),
            IDLE_PRIORITY_CLASS => Ok(Self::Idle),
            BELOW_NORMAL_PRIORITY_CLASS => Ok(Self::BelowNormal),
            ABOVE_NORMAL_PRIORITY_CLASS => Ok(Self::AboveNormal),
            HIGH_PRIORITY_CLASS => Ok(Self::High),
            REALTIME_PRIORITY_CLASS => Ok(Self::Realtime),
            PROCESS_MODE_BACKGROUND_BEGIN => Ok(Self::BackgroundModeBegin),
            PROCESS_MODE_BACKGROUND_END => Ok(Self::BackgroundModeEnd),
            _ => Err(ConversionError::OutOfRange),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ProtectionLevel {
    WinTcbLight,
    Windows,
    WindowsLight,
    AntimalwareLight,
    LsaLight,
    #[default]
    WinTcb,
    CodeGenLight,
    AuthentiCode,
    PplApp,
}

impl ProtectionLevel {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::WinTcbLight),
            1 => Some(Self::Windows),
            2 => Some(Self::WindowsLight),
            3 => Some(Self::AntimalwareLight),
            4 => Some(Self::LsaLight),
            5 => Some(Self::WinTcb),
            6 => Some(Self::CodeGenLight),
            7 => Some(Self::AuthentiCode),
            8 => Some(Self::PplApp),
            _ => None,
        }
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    fn try_from_win32(value: PROCESS_PROTECTION_LEVEL_INFORMATION) -> Result<Option<Self>> {
        match value.ProtectionLevel {
            PROTECTION_LEVEL_WINTCB_LIGHT => Ok(Some(ProtectionLevel::WinTcbLight)),
            PROTECTION_LEVEL_WINDOWS => Ok(Some(ProtectionLevel::Windows)),
            PROTECTION_LEVEL_WINDOWS_LIGHT => Ok(Some(ProtectionLevel::WindowsLight)),
            PROTECTION_LEVEL_ANTIMALWARE_LIGHT => Ok(Some(ProtectionLevel::AntimalwareLight)),
            PROTECTION_LEVEL_LSA_LIGHT => Ok(Some(ProtectionLevel::LsaLight)),
            PROTECTION_LEVEL_CODEGEN_LIGHT => Ok(Some(ProtectionLevel::CodeGenLight)),
            PROTECTION_LEVEL_WINTCB => Ok(Some(ProtectionLevel::WinTcb)),
            PROTECTION_LEVEL_AUTHENTICODE => Ok(Some(ProtectionLevel::AuthentiCode)),
            PROTECTION_LEVEL_PPL_APP => Ok(Some(ProtectionLevel::PplApp)),
            PROTECTION_LEVEL_NONE => Ok(None),
            _ => Err(ConversionError::OutOfRange.into()),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PathFormat {
    #[default]
    Win32,
    Native,
}

impl From<PathFormat> for PROCESS_NAME_FORMAT {
    fn from(value: PathFormat) -> PROCESS_NAME_FORMAT {
        match value {
            PathFormat::Win32 => PROCESS_NAME_WIN32,
            PathFormat::Native => PROCESS_NAME_NATIVE,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: usize,
    minor: usize,
    build: usize,
    patch: usize,
}

impl Version {
    pub fn new(major: usize, minor: usize, build: usize, patch: usize) -> Self {
        Version {
            major,
            minor,
            build,
            patch,
        }
    }

    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }

    pub fn build(&self) -> usize {
        self.build
    }

    pub fn patch(&self) -> usize {
        self.patch
    }
}

impl ::core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.patch
        )
    }
}

impl From<ShortVersion> for Version {
    fn from(value: ShortVersion) -> Self {
        Version::new(value.major, value.minor, 0, 0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ShortVersion {
    major: usize,
    minor: usize,
}

impl ShortVersion {
    pub fn new(major: usize, minor: usize) -> Self {
        ShortVersion { major, minor }
    }

    pub fn new_u32(major: u32, minor: u32) -> Self {
        ShortVersion::new(major as usize, minor as usize)
    }

    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }
}

impl ::core::fmt::Display for ShortVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl From<Version> for ShortVersion {
    fn from(value: Version) -> Self {
        ShortVersion::new(value.major, value.minor)
    }
}

impl From<u32> for ShortVersion {
    fn from(value: u32) -> Self {
        ShortVersion::new_u32(value >> (u32::BITS / 2), value << (u32::BITS / 2))
    }
}

#[derive(Debug)]
pub struct TimingInfo {
    creation_time: SystemTime,
    exit_time: SystemTime,
    kernel_time: SystemTime,
    user_time: SystemTime,
}

impl TimingInfo {
    pub fn new(
        creation_time: SystemTime,
        exit_time: SystemTime,
        kernel_time: SystemTime,
        user_time: SystemTime,
    ) -> Self {
        TimingInfo {
            creation_time,
            exit_time,
            kernel_time,
            user_time,
        }
    }

    pub fn creation_time(&self) -> SystemTime {
        self.creation_time
    }

    pub fn exit_time(&self) -> SystemTime {
        self.exit_time
    }

    pub fn kernel_time(&self) -> SystemTime {
        self.kernel_time
    }

    pub fn user_time(&self) -> SystemTime {
        self.user_time
    }
}

impl From<[FILETIME; 4]> for TimingInfo {
    fn from(value: [FILETIME; 4]) -> Self {
        TimingInfo::new(
            file_time_to_std_system_time(value[0]),
            file_time_to_std_system_time(value[1]),
            file_time_to_std_system_time(value[2]),
            file_time_to_std_system_time(value[3]),
        )
    }
}

pub struct ProcessHandle {
    id: isize,
}

impl ProcessHandle {
    fn try_open(
        pid: u32,
        access_rights: ProcessAccessRights,
        inherit_handle: bool,
    ) -> Result<Self> {
        Ok(ProcessHandle {
            id: process::open(pid, access_rights, inherit_handle)?,
        })
    }

    fn try_with_access(pid: u32, access_rights: ProcessAccessRights) -> Result<Self> {
        ProcessHandle::try_open(pid, access_rights, false)
    }

    pub fn current() -> Self {
        ProcessHandle {
            id: process::get_current_handle(),
        }
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if let Err(error) = foundation::close_handle(self.id) {
            println!(
                "Error occured while closing process handle: {}!",
                error.message().to_string_lossy(),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    id: u32,
    parent_id: u32,
    name: Box<str>,
}

impl Process {
    pub(crate) fn new(id: u32, parent_id: u32, name: Box<str>) -> Self {
        Process {
            id,
            parent_id,
            name,
        }
    }

    // non-priviliged access for every process except for 0 - System (can't be accessed)
    fn try_open_handle_with_default_access(&self) -> Result<ProcessHandle> {
        ProcessHandle::try_with_access(self.id, PROCESS_QUERY_LIMITED_INFORMATION)
    }

    fn try_open_handle_with_set_limited_info_access(&self) -> Result<ProcessHandle> {
        ProcessHandle::try_with_access(self.id, PROCESS_SET_LIMITED_INFORMATION)
    }

    fn try_open_handle_with_set_info_access(&self) -> Result<ProcessHandle> {
        ProcessHandle::try_with_access(self.id, PROCESS_SET_INFORMATION)
    }

    fn try_open_handle_with_set_quota_access(&self) -> Result<ProcessHandle> {
        ProcessHandle::try_with_access(self.id, PROCESS_SET_QUOTA)
    }

    fn try_open_handle_with_terminate_access(&self) -> Result<ProcessHandle> {
        ProcessHandle::try_with_access(self.id, PROCESS_TERMINATE)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn parent_id(&self) -> u32 {
        self.parent_id
    }

    pub fn path(&self) -> Result<Box<Path>> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_full_image_name(handle.id, PathFormat::default().into())
            .map_err(ProcessError::from)
    }

    pub fn path_with_format(&self, path_format: PathFormat) -> Result<Box<Path>> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_full_image_name(handle.id, path_format.into()).map_err(ProcessError::from)
    }

    pub fn priority_class(&self) -> Result<ProcessPriorityClass> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_priority_class(handle.id)?
            .try_into()
            .map_err(ProcessError::from)
    }

    pub fn set_priority_class(&self, priority_class: ProcessPriorityClass) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::set_priority_class(handle.id, priority_class.into()).map_err(ProcessError::from)
    }

    pub fn has_priority_boost(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_default_access()?;
        process::has_priority_boost(handle.id).map_err(ProcessError::from)
    }

    pub fn set_priority_boost(&self, enable: bool) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::set_priority_boost(handle.id, enable).map_err(ProcessError::from)
    }

    pub fn is_efficient(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_set_info_access()?;
        Ok(
            ProcessPriorityClass::try_from(process::get_priority_class(handle.id)?)?
                == ProcessPriorityClass::Idle,
        )
    }

    pub fn set_efficiency_mode(&self, enable: bool) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        let (priority_class, state) = if enable {
            (
                ProcessPriorityClass::Idle,
                PowerThrottlingState::ENABLE_EXECUTION_SPEED_THROTTLING,
            )
        } else {
            (
                ProcessPriorityClass::Normal,
                PowerThrottlingState::DISABLE_EXECUTION_SPEED_THROTTLING,
            )
        };
        process::set_priority_class(handle.id, priority_class.into())?;
        process::set_information::<PROCESS_POWER_THROTTLING_STATE>(handle.id, state.into())?;
        Ok(())
    }

    pub fn affinity_mask(&self) -> Result<AffinityInfo> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_affinity_mask(handle.id)
            .map(AffinityInfo::from)
            .map_err(ProcessError::from)
    }

    pub fn set_affinity_mask(&self, process_affinity_mask: usize) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::set_affinity_mask(handle.id, process_affinity_mask).map_err(ProcessError::from)
    }

    pub fn default_cpu_set_count(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_default_cpu_set_count(handle.id).map_err(ProcessError::from)
    }

    pub fn default_cpu_sets(&self) -> Result<Box<[u32]>> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_default_cpu_sets(handle.id).map_err(ProcessError::from)
    }

    pub fn clear_default_cpu_sets(&self) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        process::clear_default_cpu_sets(handle.id).map_err(ProcessError::from)
    }

    pub fn set_default_cpu_sets(&self, cpu_sets: &[u32]) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        process::set_default_cpu_sets(handle.id, cpu_sets).map_err(ProcessError::from)
    }

    pub fn working_set_size(&self) -> Result<(usize, usize)> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_working_set_size(handle.id).map_err(ProcessError::from)
    }

    pub fn set_working_set_size(&self, min_size: usize, max_size: usize) -> Result<()> {
        let handle = self.try_open_handle_with_set_quota_access()?;
        process::set_working_set_size(handle.id, min_size, max_size).map_err(ProcessError::from)
    }

    pub fn exit_code(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_exit_code(handle.id).map_err(ProcessError::from)
    }

    pub fn is_alive(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_default_access()?;
        Ok(process::get_exit_code(handle.id)? == STILL_ACTIVE.0 as u32)
    }

    pub fn is_elevated(&self) -> bool {
        match self.try_open_handle_with_default_access() {
            Ok(handle) => process::is_elevated(handle.id),
            Err(_) => true, // Protected system processes are elevated.
        }
    }

    pub fn is_critical(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_default_access()?;
        process::is_critical(handle.id).map_err(ProcessError::from)
    }

    pub fn memory_priority(&self) -> Result<MemoryPriority> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::get_information::<MEMORY_PRIORITY_INFORMATION>(handle.id)?
            .try_into()
            .map_err(ProcessError::from)
    }

    pub fn protection_level(&self) -> Result<Option<ProtectionLevel>> {
        let handle = self.try_open_handle_with_set_info_access()?;
        ProtectionLevel::try_from_win32(process::get_information::<
            PROCESS_PROTECTION_LEVEL_INFORMATION,
        >(handle.id)?)
    }

    pub fn leap_second_info(&self) -> Result<LeapSecondInfo> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::get_information::<PROCESS_LEAP_SECOND_INFO>(handle.id)
            .map(LeapSecondInfo::from)
            .map_err(ProcessError::from)
    }

    pub fn set_leap_second_info(&self, info: LeapSecondInfo) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        Ok(process::set_information::<PROCESS_LEAP_SECOND_INFO>(
            handle.id,
            info.into(),
        )?)
    }

    /*
    /// POWER_THROTTLING_STATE can only be set as of now, which means
    /// there's no way to determine if a process is currently in EcoQoS mode.

    pub fn power_throttling_state(&self) -> Result<PowerThrottlingState> {
        process::get_information::<PROCESS_POWER_THROTTLING_STATE>(self.handle.id)?.try_into()
    }
    */

    pub fn set_power_throttling_state(&self, state: PowerThrottlingState) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::set_information::<PROCESS_POWER_THROTTLING_STATE>(handle.id, state.into())
            .map_err(ProcessError::from)
    }

    pub fn set_memory_priority(&self, priority: MemoryPriority) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::set_information::<MEMORY_PRIORITY_INFORMATION>(handle.id, priority.into())
            .map_err(ProcessError::from)
    }

    // TODO: doctorWTF
    pub fn app_memory_usage(&self) -> Result<MemoryUsage> {
        let handle = self.try_open_handle_with_set_info_access()?;
        process::get_information::<APP_MEMORY_INFORMATION>(handle.id)
            .map(MemoryUsage::from)
            .map_err(ProcessError::from)
    }

    pub fn handle_count(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_handle_count(handle.id).map_err(ProcessError::from)
    }

    pub fn io_counters(&self) -> Result<IoCounters> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_io_counters(handle.id)
            .map(IoCounters::from)
            .map_err(ProcessError::from)
    }

    /// The system version that the process is expected to run on.
    pub fn system_version(&self) -> Result<ShortVersion> {
        process::get_version(self.id)
            .map(ShortVersion::from)
            .map_err(ProcessError::from)
    }

    pub fn times(&self) -> Result<TimingInfo> {
        let handle = self.try_open_handle_with_default_access()?;
        process::get_times(handle.id)
            .map(TimingInfo::from)
            .map_err(ProcessError::from)
    }

    pub fn terminate(&self, exit_code: u32) -> Result<()> {
        let handle = self.try_open_handle_with_terminate_access()?;
        process::terminate(handle.id, exit_code).map_err(ProcessError::from)
    }

    pub fn wait_for_input_idle(&self, timeout_ms: u32) -> Result<()> {
        let handle = ProcessHandle::try_with_access(
            self.id,
            PROCESS_QUERY_INFORMATION | PROCESS_SYNCHRONIZE,
        )?;
        process::wait_for_input_idle(handle.id, timeout_ms).map_err(ProcessError::from)
    }
}

// TODO: impl Display for Process and Thread
/*
impl ::core::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Process id: {}", self.id)
    }
}
*/

pub fn exit(exit_code: u32) {
    process::exit_current(exit_code)
}

pub fn current_id() -> u32 {
    process::get_current_id()
}

pub fn pids_with_buffer(buffer: &mut [u32]) -> Result<u32> {
    Ok(process_status::get_pids_with_buffer(buffer)?)
}

pub fn pids() -> Result<Box<[u32]>> {
    process_status::get_pids().map_err(ProcessError::from)
}

pub fn find_by_id(id: u32) -> Result<Option<Process>> {
    Snapshot::<Process>::try_create()
        .map(|mut snap| snap.find(|proc_info| proc_info.id() == id))
        .map_err(ProcessError::from)
}

pub fn processes() -> Result<impl Iterator<Item = Process>> {
    Snapshot::<Process>::try_create().map_err(ProcessError::from)
}
