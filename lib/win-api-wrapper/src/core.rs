pub mod error;

pub type Result<T> = core::result::Result<T, super::core::error::Error>;
