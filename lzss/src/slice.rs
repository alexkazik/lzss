use crate::read_write::{Read, Write};
use core::marker::PhantomData;

/// Read from a slice.
///
/// ```
/// # use lzss::*;
/// let input_data = [10, 42];
/// let mut input = SliceReader::new(&input_data);
/// assert_eq!(input.read(), Ok(Some(10)));
/// assert_eq!(input.read(), Ok(Some(42)));
/// assert_eq!(input.read(), Ok(None));
/// ```
///
/// Use [`unwrap_read`](crate::UnwrapReadExt::unwrap_read) to remove the `!` from the result.
/// Or [`unwrap_read_write`](crate::UnwrapReadWriteExt::unwrap_read_write) if also the writer produces `!`.
pub struct SliceReader<'a> {
    pos: *const u8,
    end: *const u8,
    phantom_data: PhantomData<&'a ()>,
}
impl<'a> SliceReader<'a> {
    /// Constructs a new reader.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a [u8]) -> SliceReader<'a> {
        let ptr = data.as_ptr();
        SliceReader {
            pos: ptr,
            end: unsafe { ptr.add(data.len()) },
            phantom_data: PhantomData,
        }
    }
}
impl<'a> Read for SliceReader<'a> {
    /// No error can occur.
    type Error = !;
    #[inline(always)]
    fn read(&mut self) -> Result<Option<u8>, Self::Error> {
        if self.pos == self.end {
            // reached eof
            Ok(None)
        } else {
            let result = unsafe { self.pos.read() };
            self.pos = unsafe { self.pos.add(1) };
            Ok(Some(result))
        }
    }
}

/// A zero-sized type, will be returned in case of an error.
#[derive(Debug, Eq, PartialEq)]
pub struct SliceWriteError;

impl core::fmt::Display for SliceWriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Over- or underflow happened")
    }
}

#[cfg(any(test, feature = "std"))]
impl std::error::Error for SliceWriteError {}

/// Write into a slice.
///
/// Returns amount of written bytes.
///
/// ```
/// # use lzss::*;
/// let mut buf = [0; 2];
/// // underfull
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(1));
/// // full
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(2));
/// // overflow
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Err(SliceWriteError));
/// ```
pub struct SliceWriter<'a> {
    start: *mut u8,
    pos: *mut u8,
    end: *mut u8,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> SliceWriter<'a> {
    /// Constructs a new writer.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> SliceWriter<'a> {
        let ptr = data.as_mut_ptr();
        SliceWriter {
            start: ptr,
            pos: ptr,
            end: unsafe { ptr.add(data.len()) },
            phantom_data: PhantomData,
        }
    }
}

impl<'a> Write for SliceWriter<'a> {
    /// Returns written bytes.
    type Output = usize;
    /// In case of an overflow this error is returned.
    type Error = SliceWriteError;
    #[inline(always)]
    fn write(&mut self, data: u8) -> Result<(), Self::Error> {
        if self.pos == self.end {
            Err(SliceWriteError)
        } else {
            unsafe { self.pos.write(data) };
            self.pos = unsafe { self.pos.add(1) };
            Ok(())
        }
    }
    #[inline(always)]
    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok((unsafe { self.pos.offset_from(self.start) }) as usize)
    }
}

/// Write into a slice which has the exact size of the result.
///
/// ```
/// # use lzss::*;
/// let mut buf = [0; 2];
/// // underfull
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Err(SliceWriteError));
/// // full
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(()));
/// // overflow
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Err(SliceWriteError));
/// ```
pub struct SliceWriterExact<'a> {
    pos: *mut u8,
    end: *mut u8,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> SliceWriterExact<'a> {
    /// Constructs a new writer.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> SliceWriterExact<'a> {
        let ptr = data.as_mut_ptr();
        SliceWriterExact {
            pos: ptr,
            end: unsafe { ptr.add(data.len()) },
            phantom_data: PhantomData,
        }
    }
}

impl<'a> Write for SliceWriterExact<'a> {
    /// Returns always `()`.
    type Output = ();
    /// In case of an under- or overflow this error is returned.
    type Error = SliceWriteError;
    #[inline(always)]
    fn write(&mut self, data: u8) -> Result<(), Self::Error> {
        if self.pos == self.end {
            Err(SliceWriteError)
        } else {
            unsafe { self.pos.write(data) };
            self.pos = unsafe { self.pos.add(1) };
            Ok(())
        }
    }
    #[inline(always)]
    fn finish(self) -> Result<Self::Output, Self::Error> {
        if self.pos == self.end {
            Ok(())
        } else {
            Err(SliceWriteError)
        }
    }
}
