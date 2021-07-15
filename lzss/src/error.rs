use core::fmt::Display;

/// This represents either an read or write error.
#[derive(Debug, Eq, PartialEq)]
pub enum LzssError<R, W> {
  /// Contains the read error value.
  ReadError(R),
  /// Contains the write error value.
  WriteError(W),
}

impl<R: Display, W: Display> core::fmt::Display for LzssError<R, W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      LzssError::ReadError(error) => write!(f, "Read error: {}", error),
      LzssError::WriteError(error) => write!(f, "Write error: {}", error),
    }
  }
}

/// `std` Implementation of [Error](std::error::Error) for [LzssError]
#[cfg(any(doc, test, feature = "std"))]
impl<R, W> std::error::Error for LzssError<R, W>
where
  R: std::error::Error + 'static,
  W: std::error::Error + 'static,
{
  #[inline]
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      LzssError::ReadError(error) => Some(error),
      LzssError::WriteError(error) => Some(error),
    }
  }
}

impl<R, W> LzssError<R, W> {
  /// Maps a `LzssError<R, W>` to `LzssError<E, W>` by applying a function to a contained read error value, leaving an write error value untouched.
  #[inline]
  pub fn map_read_error<E, O: FnOnce(R) -> E>(self, op: O) -> LzssError<E, W> {
    match self {
      LzssError::ReadError(e) => LzssError::ReadError(op(e)),
      LzssError::WriteError(e) => LzssError::WriteError(e),
    }
  }
  /// Maps a `LzssError<R, W>` to `LzssError<R, E>` by applying a function to a contained write error value, leaving an read error value untouched.
  #[inline]
  pub fn map_write_error<E, O: FnOnce(W) -> E>(self, op: O) -> LzssError<R, E> {
    match self {
      LzssError::ReadError(e) => LzssError::ReadError(e),
      LzssError::WriteError(e) => LzssError::WriteError(op(e)),
    }
  }
}
