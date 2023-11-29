/// Allows types to be infallibly generalized.
pub trait AsGeneral<T>: Sized
// TODO: remove this where clause
where
    T: TryAsSpecific<Self> + From<Self>,
{
    /// Converts `self` into a generalized version.
    fn as_general(self) -> T;
}

/// Allows types to be fallibly generalized.
pub trait TryAsGeneral<T>: Sized
// TODO: remove this where clause
where
    T: TryAsSpecific<Self>,
{
    /// Converts `self` into a generalized version.
    fn try_as_general(self) -> Option<T>;
}

impl<T, This> TryAsGeneral<T> for This
// TODO: remove this where clause
where
    This: AsGeneral<T>,
    T: TryAsSpecific<Self> + From<Self>,
{
    fn try_as_general(self) -> Option<T> {
        Some(self.as_general())
    }
}

/// Allows types to be infallibly narrowed down.
pub trait AsSpecific<T>: Sized {
    /// Converts `self` into a more specific version, returning [`None`] if it isn't possible.
    fn as_specific(self) -> T;
}

/// Allows types to be fallibly narrowed down.
pub trait TryAsSpecific<T>: Sized {
    /// Converts `self` into a more specific version, returning [`None`] if it isn't possible.
    fn try_as_specific(self) -> Option<T>;
}

impl<T, This> TryAsSpecific<T> for This
where
    This: AsSpecific<T>,
{
    fn try_as_specific(self) -> Option<T> {
        Some(self.as_specific())
    }
}
