use core::fmt::Debug;
use win_api_wrapper::win32::{
    foundation::POINT,
    system::threading::{
        process::{
            MEMORY_PRIORITY, MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_INFORMATION,
            MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL,
            MEMORY_PRIORITY_VERY_LOW, PROCESS_POWER_THROTTLING_CURRENT_VERSION,
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
            PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION, PROCESS_POWER_THROTTLING_STATE,
        },
        thread::THREAD_POWER_THROTTLING_STATE,
    },
};

#[derive(Debug)]
pub enum ConversionError {
    OutOfRange,
    InvalidValue,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Point<T>
where
    T: Default + Copy,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Default + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn from_x(x: T) -> Self {
        Point {
            x,
            ..Default::default()
        }
    }

    pub fn from_y(y: T) -> Self {
        Point {
            y,
            ..Default::default()
        }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> From<T> for Point<T>
where
    T: Default + Copy,
{
    fn from(value: T) -> Self {
        Point::from_x(value)
    }
}

impl From<POINT> for Point<i32> {
    fn from(value: POINT) -> Self {
        Point::new(value.x, value.y)
    }
}

impl From<Point<i32>> for POINT {
    fn from(value: Point<i32>) -> Self {
        POINT {
            x: value.x,
            y: value.y,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemoryPriority {
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}

impl MemoryPriority {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::VeryLow),
            1 => Some(Self::Low),
            2 => Some(Self::Medium),
            3 => Some(Self::BelowNormal),
            4 => Some(Self::Normal),
            _ => None,
        }
    }
}

impl As<u8> for MemoryPriority {
    #[inline]
    fn as_type(&self) -> u8 {
        *self as u8
    }
}

impl As<u32> for MemoryPriority {
    #[inline]
    fn as_type(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<MEMORY_PRIORITY_INFORMATION> for MemoryPriority {
    type Error = ConversionError;
    fn try_from(info: MEMORY_PRIORITY_INFORMATION) -> ::core::result::Result<Self, Self::Error> {
        match info.MemoryPriority {
            MEMORY_PRIORITY_VERY_LOW => Ok(MemoryPriority::VeryLow),
            MEMORY_PRIORITY_LOW => Ok(MemoryPriority::Low),
            MEMORY_PRIORITY_MEDIUM => Ok(MemoryPriority::Medium),
            MEMORY_PRIORITY_BELOW_NORMAL => Ok(MemoryPriority::BelowNormal),
            MEMORY_PRIORITY_NORMAL => Ok(MemoryPriority::Normal),
            _ => Err(ConversionError::OutOfRange),
        }
    }
}

impl From<MemoryPriority> for MEMORY_PRIORITY_INFORMATION {
    fn from(value: MemoryPriority) -> Self {
        MEMORY_PRIORITY_INFORMATION {
            MemoryPriority: MEMORY_PRIORITY(value.as_type()),
        }
    }
}

impl ::core::fmt::Display for MemoryPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VeryLow => write!(f, "Very low{}", self.clone()),
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::BelowNormal => write!(f, "Below normal"),
            Self::Normal => write!(f, "Normal"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PowerThrottlingControlMask {
    #[default]
    SystemManaged = 0,
    ThrottleExecutionSpeed = PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
    IgnoreTimerResolution = PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION,
}

impl ::core::fmt::Display for PowerThrottlingControlMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemManaged => write!(f, "System managed mask: {:b}", self.as_type()),
            Self::ThrottleExecutionSpeed => {
                write!(f, "Throttle execution speed mask: {:b}", self.as_type())
            }
            Self::IgnoreTimerResolution => {
                write!(f, "Ignore timer resulution mask: {:b}", self.as_type())
            }
        }
    }
}

impl As<u32> for PowerThrottlingControlMask {
    #[inline]
    fn as_type(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<u32> for PowerThrottlingControlMask {
    type Error = ConversionError;
    fn try_from(value: u32) -> ::core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerThrottlingControlMask::SystemManaged),
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED => {
                Ok(PowerThrottlingControlMask::ThrottleExecutionSpeed)
            }
            PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION => {
                Ok(PowerThrottlingControlMask::IgnoreTimerResolution)
            }
            _ => Err(ConversionError::InvalidValue),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct PowerThrottlingState {
    mask: PowerThrottlingControlMask,
    enable_state: bool,
    version: u32,
}

impl PowerThrottlingState {
    pub const SYSTEM_MANAGED: PowerThrottlingState =
        Self::new(PowerThrottlingControlMask::SystemManaged, false);
    pub const ENABLE_EXECUTION_SPEED_THROTTLING: PowerThrottlingState =
        Self::new(PowerThrottlingControlMask::ThrottleExecutionSpeed, true);
    pub const DISABLE_EXECUTION_SPEED_THROTTLING: PowerThrottlingState =
        Self::new(PowerThrottlingControlMask::ThrottleExecutionSpeed, false);
    pub const IGNORE_TIMER_RESOLUTION: PowerThrottlingState =
        Self::new(PowerThrottlingControlMask::IgnoreTimerResolution, true);
    pub const USE_TIMER_RESOLUTION: PowerThrottlingState =
        Self::new(PowerThrottlingControlMask::IgnoreTimerResolution, false);

    pub const fn new(mask: PowerThrottlingControlMask, enable_state: bool) -> Self {
        PowerThrottlingState {
            mask,
            enable_state,
            version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        }
    }

    pub const fn version(&self) -> u32 {
        self.version
    }
}

impl From<PowerThrottlingState> for PROCESS_POWER_THROTTLING_STATE {
    fn from(value: PowerThrottlingState) -> PROCESS_POWER_THROTTLING_STATE {
        PROCESS_POWER_THROTTLING_STATE {
            Version: value.version,
            ControlMask: value.mask.as_type(),
            StateMask: if value.enable_state {
                value.mask.as_type()
            } else {
                0
            },
        }
    }
}

impl TryFrom<PowerThrottlingState> for THREAD_POWER_THROTTLING_STATE {
    type Error = ConversionError;
    fn try_from(
        state: PowerThrottlingState,
    ) -> ::core::result::Result<THREAD_POWER_THROTTLING_STATE, Self::Error> {
        if state.mask == PowerThrottlingControlMask::IgnoreTimerResolution {
            return Err(Self::Error::InvalidValue);
        }

        Ok(THREAD_POWER_THROTTLING_STATE {
            Version: state.version,
            ControlMask: state.mask.as_type(),
            StateMask: if state.enable_state {
                state.mask.as_type()
            } else {
                0
            },
        })
    }
}

impl TryFrom<PROCESS_POWER_THROTTLING_STATE> for PowerThrottlingState {
    type Error = ConversionError;
    fn try_from(
        state: PROCESS_POWER_THROTTLING_STATE,
    ) -> ::core::result::Result<Self, Self::Error> {
        Ok(Self::new(
            state.ControlMask.try_into()?,
            state.StateMask > 0,
        ))
    }
}
