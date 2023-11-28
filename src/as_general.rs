/// Allows types to be generalized. This trait uses associated types to ensure that it is only
/// implemented once on a given type; a type should not have multiple generalizations.
///
/// Requires that `Self::Output: From<Self>` to make sure that implementations of `From` are kept
/// up to date. Also requires that `TryAsSpecific` is implemented on `Self::Output` to ensure that
/// items can properly be converted back and forth.
///
/// Also provides a blanket implementation of [`TryAsGeneral`].
pub trait AsGeneral: Sized
where
    Self::Output: From<Self> + TryAsSpecific<Self>,
{
    /// The generalized version of this type.
    type Output;

    /// Converts `self` into a generalized version.
    fn as_general(self) -> Self::Output;
}

/// Allows types to be fallibly generalized. This trait uses associated types to ensure that it is
/// only implemented once on a given type; a type should not have multiple generalizations.
///
/// Requires that `TryAsSpecific` is implemented on `Self::Output` to ensure that items can properly
/// be converted back and forth.
pub trait TryAsGeneral: Sized
where
    Self::Output: TryAsSpecific<Self>,
{
    /// The generalized version of this type.
    type Output;

    /// Converts `self` into a generalized version.
    fn try_as_general(self) -> Option<Self::Output>;
}

impl<T> TryAsGeneral for T
where
    T: AsGeneral,
{
    type Output = <T as AsGeneral>::Output;

    fn try_as_general(self) -> Option<Self::Output> {
        Some(self.as_general())
    }
}

/// Allows types to be narrowed down. This trait uses generics to allow for multiple specific
/// versions of a given type.
pub trait TryAsSpecific<T>: Sized {
    /// Converts `self` into a more specific version, returning [`None`] if it isn't possible.
    fn try_as_specific(self) -> Option<T>;
}
