use crate::{Read, Write};
use std::io::Error;

// As already denoted this is a very simplistic implementation,
// everybody is welcome to improve it.

/// `std` Read from a stream, this is a inefficient exemplary implementation.
pub struct IOSimpleReader<'a, R: std::io::Read>(&'a mut R);
impl<'a, R: std::io::Read> IOSimpleReader<'a, R> {
  /// Constructs a new reader.
  #[inline(always)]
  #[must_use]
  pub fn new(stream: &'a mut R) -> IOSimpleReader<'a, R> {
    IOSimpleReader(stream)
  }
}
impl<'a, R: std::io::Read> Read for IOSimpleReader<'a, R> {
  type Error = Error;
  fn read(&mut self) -> Result<Option<u8>, Self::Error> {
    let mut buf = [0; 1];
    if self.0.read(&mut buf)? == 0 {
      Ok(None)
    } else {
      Ok(Some(buf[0]))
    }
  }
}

/// `std` Write to a stream, this is a inefficient exemplary implementation.
pub struct IOSimpleWriter<'a, W: std::io::Write>(&'a mut W);
impl<'a, W: std::io::Write> IOSimpleWriter<'a, W> {
  /// Constructs a new writer.
  #[inline(always)]
  #[must_use]
  pub fn new(stream: &'a mut W) -> IOSimpleWriter<'a, W> {
    IOSimpleWriter(stream)
  }
}
impl<'a, W: std::io::Write> Write for IOSimpleWriter<'a, W> {
  type Output = ();
  type Error = Error;
  fn write(&mut self, data: u8) -> Result<(), Self::Error> {
    let buf = [data];
    self.0.write_all(&buf)
  }
  #[inline(always)]
  fn finish(self) -> Result<Self::Output, Self::Error> {
    self.0.flush()
  }
}

#[cfg(test)]
mod tests {
  use crate::{IOSimpleReader, IOSimpleWriter, Lzss, LzssError};
  use std::io::{Cursor, ErrorKind};

  type TestLZSS = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
  const TEST_DATA: &[u8; 27] = b"Sample   Data   11221233123";

  #[test]
  fn test_simple_io() {
    let mut output = [0u8; 30];
    let mut output_cursor = Cursor::new(&mut output[..]);
    let output_result = TestLZSS::compress(
      IOSimpleReader::new(&mut Cursor::new(TEST_DATA)),
      IOSimpleWriter::new(&mut output_cursor),
    );
    assert_eq!(
      output_result.map_err(|x| x.map_read_error(|x| x.kind()).map_write_error(|x| x.kind())),
      Ok(())
    );
    assert_eq!(output_cursor.position(), 26);
  }
  #[test]
  fn test_simple_io_fail() {
    let mut output = [0u8; 10];
    let mut output_cursor = Cursor::new(&mut output[..]);
    let output_result = TestLZSS::compress(
      IOSimpleReader::new(&mut Cursor::new(TEST_DATA)),
      IOSimpleWriter::new(&mut output_cursor),
    );
    assert_eq!(
      output_result.map_err(|x| x.map_read_error(|x| x.kind()).map_write_error(|x| x.kind())),
      Err(LzssError::WriteError(ErrorKind::WriteZero))
    );
  }
}
