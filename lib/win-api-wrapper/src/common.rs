/// This trait defines a `to` method for `Borrowed` to `Owned` conversion between two types.
pub trait To<T> {
    /// Converts the borrowed type to an owned type of `T`.
    fn to(&self) -> T;
}

/// Defines a method for `Borrowed` to debug [`String`] conversion.
pub trait ToDebugString {
    /// Creates a debug [`String`] from `self`.
    fn to_debug_string(&self) -> String;
}
