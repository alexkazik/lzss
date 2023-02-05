use crate::error::LzssError;
use core::convert::Infallible;

/// Conversion from `Result<T, LzssError<Infallible, Infallible>>` to `T`.
pub trait UnwrapReadWriteExt<T>: Sized {
    /// Get the value out of a wrapper.
    fn unwrap_read_write(self) -> T;
}

impl<T> UnwrapReadWriteExt<T> for Result<T, LzssError<Infallible, Infallible>> {
    /// Get the value out of an always-ok Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline(always)]
    fn unwrap_read_write(self) -> T {
        match self {
            Ok(val) => val,
        }
    }
}

/// Conversion from `Result<T, LzssError<Infallible, E>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzssError`] layer from the Result.
pub trait UnwrapReadExt<E, T>: Sized {
    /// Remove the [`LzssError`] layer from the Result.
    fn unwrap_read(self) -> Result<T, E>;
}

impl<E, T> UnwrapReadExt<E, T> for Result<T, LzssError<Infallible, E>> {
    /// Remove the [`LzssError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn unwrap_read(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzssError::WriteError(e)) => Err(e),
        }
    }
}

/// Conversion from `Result<T, LzssError<E, Infallible>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzssError`] layer from the Result.
pub trait UnwrapWriteExt<E, T>: Sized {
    /// Remove the [`LzssError`] layer from the Result.
    fn unwrap_write(self) -> Result<T, E>;
}

impl<E, T> UnwrapWriteExt<E, T> for Result<T, LzssError<E, Infallible>> {
    /// Remove the [`LzssError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn unwrap_write(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzssError::ReadError(e)) => Err(e),
        }
    }
}
