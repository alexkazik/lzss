use crate::error::LzssError;
use void::{unreachable, Void};

/// Conversion from `Result<T, LzssError<Void, Void>>` to `T`.
#[deprecated(since = "TBD", note = "renamed to UnwrapReadWriteExt")]
pub trait ResultLzssErrorVoidExt<T>: Sized {
    /// Get the value out of a wrapper.
    #[deprecated(since = "TBD", note = "renamed to unwrap_read_write")]
    fn void_unwrap(self) -> T;
}

#[allow(deprecated)]
impl<T> ResultLzssErrorVoidExt<T> for Result<T, LzssError<Void, Void>> {
    /// Get the value out of an always-ok Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline(always)]
    fn void_unwrap(self) -> T {
        match self {
            Ok(val) => val,
            Err(LzssError::ReadError(e)) => unreachable(e),
            Err(LzssError::WriteError(e)) => unreachable(e),
        }
    }
}

/// Conversion from `Result<T, LzssError<Void, E>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzssError`] layer from the Result.
#[deprecated(since = "TBD", note = "renamed to UnwrapReadExt")]
pub trait ResultLzssErrorVoidReadExt<E, T>: Sized {
    /// Remove the [`LzssError`] layer from the Result.
    #[deprecated(since = "TBD", note = "renamed to unwrap_read")]
    fn void_read_unwrap(self) -> Result<T, E>;
}

#[allow(deprecated)]
impl<E, T> ResultLzssErrorVoidReadExt<E, T> for Result<T, LzssError<Void, E>> {
    /// Remove the [`LzssError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn void_read_unwrap(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzssError::ReadError(e)) => unreachable(e),
            Err(LzssError::WriteError(e)) => Err(e),
        }
    }
}

/// Conversion from `Result<T, LzssError<E, Void>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzssError`] layer from the Result.
#[deprecated(since = "TBD", note = "renamed to UnwrapWriteExt")]
pub trait ResultLzssErrorVoidWriteExt<E, T>: Sized {
    /// Remove the [`LzssError`] layer from the Result.
    #[deprecated(since = "TBD", note = "renamed to unwrap_write")]
    fn void_write_unwrap(self) -> Result<T, E>;
}

#[allow(deprecated)]
impl<E, T> ResultLzssErrorVoidWriteExt<E, T> for Result<T, LzssError<E, Void>> {
    /// Remove the [`LzssError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn void_write_unwrap(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzssError::ReadError(e)) => Err(e),
            Err(LzssError::WriteError(e)) => unreachable(e),
        }
    }
}
