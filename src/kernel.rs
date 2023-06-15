use crate::common::{As, ConversionError};
use win_api_wrapper::win32::system::kernel::PROCESSOR_NUMBER;

// TODO: move and improve macro
macro_rules! define_cpu_indices {
    ($visibility:vis enum $enum_name:ident : $type_repr:ty { $first_name:ident, $($name:ident),* }) => {
        #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
        #[repr($type_repr)]
        $visibility enum $enum_name {
            #[default]
            $first_name,
            $($name),*,
        }

        impl $enum_name {
            #[inline]
            pub fn mask(&self) -> usize { 1 << self.as_type() }
        }

        impl As<u8> for $enum_name {
            #[inline]
            fn as_type(&self) -> u8 {
                *self as u8
            }
        }

        impl ::core::convert::TryFrom<u8> for $enum_name {
            type Error = crate::common::ConversionError;
            fn try_from(value: u8) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    x if x == Self::$first_name.as_type() => Ok(Self::$first_name),
                    $(x if x == Self::$name.as_type() => Ok(Self::$name),)*
                    _ => Err(crate::common::ConversionError::InvalidValue),
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Self::$first_name => write!(f, "{}", stringify!($first_name)),
                    $(Self::$name => write!(f, "{}", stringify!($name)),)*
                }
            }
        }
    };
    (enum $enum_name:ident : $type_repr:ty { $first_name:ident, $($name:ident),* }) => {
        #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
        #[repr($type_repr)]
        $visibility enum $enum_name {
            #[default]
            $first_name,
            $($name),*,
        }

        impl $enum_name {
            #[inline]
            pub fn mask(&self) -> usize { 1 << self.as_type() }
        }

        impl As<u8> for $enum_name {
            #[inline]
            fn as_type(&self) -> u8 {
                *self as u8
            }
        }

        impl ::core::convert::TryFrom<u8> for $enum_name {
            type Error = crate::common::ConversionError;
            fn try_from(value: u8) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    x if x == Self::$first_name.as_type() => Ok(Self::$first_name),
                    $(x if x == Self::$name.as_type() => Ok(Self::$name),)*
                    _ => Err(crate::common::ConversionError::InvalidValue),
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Self::$first_name => write!(f, "{}", stringify!($first_name)),
                    $(Self::$name => write!(f, "{}", stringify!($name)),)*
                }
            }
        }
    };
}

macro_rules! my_macro {
    ($($ident:ident),+ $($last:ident)? ) => {
        // macro body goes here
    };
}

my_macro!(a);

define_cpu_indices!(
    pub enum ProcessorIndex: u8
    {
        CPU0, CPU1, CPU2, CPU3, CPU4, CPU5, CPU6, CPU7, CPU8, CPU9, CPU10, CPU11, CPU12, CPU13, CPU14, CPU15,
        CPU16, CPU17, CPU18, CPU19, CPU20, CPU21, CPU22, CPU23, CPU24, CPU25, CPU26, CPU27, CPU28, CPU29, CPU30, CPU31,
        CPU32, CPU33, CPU34, CPU35, CPU36, CPU37, CPU38, CPU39, CPU40, CPU41, CPU42, CPU43, CPU44, CPU45, CPU46, CPU47,
        CPU48, CPU49, CPU50, CPU51, CPU52, CPU53, CPU54, CPU55, CPU56, CPU57, CPU58, CPU59, CPU60, CPU61, CPU62, CPU63
    }
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Processor {
    index: ProcessorIndex,
    group_index: u16,
}

impl Processor {
    fn new(index: ProcessorIndex, group_index: u16) -> Self {
        Processor { index, group_index }
    }

    pub fn index(&self) -> ProcessorIndex {
        self.index
    }

    pub fn group_index(&self) -> u16 {
        self.group_index
    }
}

impl From<Processor> for PROCESSOR_NUMBER {
    fn from(value: Processor) -> PROCESSOR_NUMBER {
        PROCESSOR_NUMBER {
            Group: value.group_index,
            Number: value.index.as_type(),
            Reserved: 0,
        }
    }
}

impl TryFrom<PROCESSOR_NUMBER> for Processor {
    type Error = ConversionError;
    fn try_from(value: PROCESSOR_NUMBER) -> ::core::result::Result<Self, Self::Error> {
        Ok(Processor::new(value.Number.try_into()?, value.Group))
    }
}
