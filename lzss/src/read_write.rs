/// Trait for reading bytes.
pub trait Read {
    /// The error which can happen during a read operation.
    ///
    /// Use [`!`](never) when no error can be emitted.
    type Error;
    /// Read a byte.
    ///
    /// Return `Ok(None)` in case of eof.
    ///
    /// Please be aware that even after reading an eof it may be tried again
    /// (which then also has to result in an eof).
    fn read(&mut self) -> Result<Option<u8>, Self::Error>;
}

/// Trait for writing bytes.
pub trait Write {
    /// The final output.
    ///
    /// This will be often `()`, but for example the [`VecWriter`](crate::VecWriter) returns the [Vec].
    ///
    /// Please see the example implementations.
    type Output;
    /// The error which can happen during a write or finish operation.
    ///
    /// Use [`!`](never) when no error can be emitted.
    type Error;
    /// Write a byte.
    fn write(&mut self, data: u8) -> Result<(), Self::Error>;
    /// Convert the writer into the output.
    ///
    /// When the underlying structure requires a flush, call it in this routine.
    ///
    /// Be aware that `finish` is not called when an error occurred.
    fn finish(self) -> Result<Self::Output, Self::Error>;
}
