//! Provides traits that allow types to be converted into more general and more specific forms.

/// Allows types to be generalized.
pub trait AsGeneral<T>: Sized
where
    T: TryAsSpecific<Self> + From<Self>,
{
    /// Converts `self` into a more general version.
    fn as_general(self) -> T;
}

/// Allows types to be fallibly generalized.
pub trait TryAsGeneral<T>: Sized
where
    T: TryAsSpecific<Self>,
{
    /// Converts `self` into a more general version, returning [`None`] if it isn't possible.
    fn try_as_general(self) -> Option<T>;
}

/// Allows types to be narrowed down.
pub trait AsSpecific<T>: Sized {
    /// Converts `self` into a more specific version.
    fn as_specific(self) -> T;
}

/// Allows types to be fallibly narrowed down.
pub trait TryAsSpecific<T>: Sized {
    /// Converts `self` into a more specific version, returning [`None`] if it isn't possible.
    fn try_as_specific(self) -> Option<T>;
}

impl<T, This> TryAsGeneral<T> for This
where
    This: AsGeneral<T>,
    T: TryAsSpecific<Self> + From<Self>,
{
    fn try_as_general(self) -> Option<T> {
        Some(self.as_general())
    }
}

impl<T, This> TryAsSpecific<T> for This
where
    This: AsSpecific<T>,
{
    fn try_as_specific(self) -> Option<T> {
        Some(self.as_specific())
    }
}
