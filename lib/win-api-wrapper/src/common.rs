/// This trait defines a `to` method for Borrowed -> Owned conversion between two types.
pub trait To<T> {
    /// Converts the borrowed type to an owned type of `T`.
    fn to(&self) -> T;
}
