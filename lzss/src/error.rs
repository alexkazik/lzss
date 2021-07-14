/// This represents either an read or write error.
#[derive(Debug, Eq, PartialEq)]
pub enum LzssError<R, W> {
  /// Contains the read error value.
  ReadError(R),
  /// Contains the write error value.
  WriteError(W),
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
