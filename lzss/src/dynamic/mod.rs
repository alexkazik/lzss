use crate::{LzssError, Read, Write};

mod compress;
mod decompress;

/// Dynamic parameters for de-/compression (see [Lzss](crate::Lzss) for compile-time parameters).
///
/// # Parameters
/// * `ei` - The number of bits in the offset, usualy `10..13`
/// * `ej` - The number of bits in the length, usually `4..5`
/// * `c` - The initial fill byte of the buffer, usually `0x20` (space)
///
/// # Restrictions
/// * `ej` must be larger than `0`
/// * `ei` must be larger than `ej`
/// * `ei + ej` must be at least 8
/// * `ei + ej` must be 24 or less
///
/// # Example
/// ```rust
/// # use lzss::{LzssDyn, LzssDynError, ResultLzssErrorVoidExt, SliceReader, VecWriter};
/// let my_lzss = LzssDyn::new(10, 4, 0x20)?;
/// let input = b"Example Data";
/// let result = my_lzss.compress(
///   SliceReader::new(input),
///   VecWriter::with_capacity(30),
/// );
/// assert_eq!(result.void_unwrap().len(), 14); // the output is 14 bytes long
/// # Ok::<(), LzssDynError>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LzssDyn {
  ei: usize,
  ej: usize,
  c: u8,
}

impl LzssDyn {
  /// Create new Lzss parameters.
  ///
  /// If the parameter are not valid (see above) an error is returned.
  pub fn new(ei: usize, ej: usize, c: u8) -> Result<Self, LzssDynError> {
    if ej == 0 {
      Err(LzssDynError::EjIsZero)
    } else if ej >= ei {
      Err(LzssDynError::EiNotLargerThanEj)
    } else if ei + ej < 8 {
      Err(LzssDynError::EiEjToSmall)
    } else if ei + ej > 24 {
      Err(LzssDynError::EiEjToLarge)
    } else {
      Ok(LzssDyn { ei, ej, c })
    }
  }

  /// Get the ei parameter.
  #[inline(always)]
  pub const fn ei(&self) -> usize {
    self.ei
  }

  /// Get the ej parameter.
  #[inline(always)]
  pub const fn ej(&self) -> usize {
    self.ej
  }

  /// Get the c parameter.
  #[inline(always)]
  pub const fn c(&self) -> u8 {
    self.c
  }

  #[inline(always)]
  pub(crate) const fn n(&self) -> usize {
    1 << self.ei
  }

  #[inline(always)]
  pub(crate) const fn p(&self) -> usize {
    (1 + self.ei + self.ej) / 9
  }

  #[inline(always)]
  pub(crate) const fn f(&self) -> usize {
    (1 << self.ej) + self.p()
  }

  /// `std` Compress the input data into the output.
  ///
  /// The buffer, with `2 * (1 << EI)` bytes, is allocated on the heap.
  #[cfg(any(doc, test, feature = "std"))]
  pub fn compress<R: Read, W: Write>(
    &self,
    mut reader: R,
    mut writer: W,
  ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
    let mut buffer = vec![self.c; 2 * self.n()];
    self.compress_internal(&mut reader, &mut writer, &mut buffer)?;
    writer.finish().map_err(LzssError::WriteError)
  }

  /// Compress the input data into the output.
  ///
  /// It will be asserted at runtime that the buffer is at least `2 * (1 << EI)`.
  pub fn compress_with_buffer<R: Read, W: Write>(
    &self,
    mut reader: R,
    mut writer: W,
    buffer: &mut [u8],
  ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
    assert!(buffer.len() >= 2 * self.n());
    unsafe { ::core::ptr::write_bytes(buffer.as_mut_ptr(), self.c, self.n() - self.f()) };
    self.compress_internal(&mut reader, &mut writer, buffer)?;
    writer.finish().map_err(LzssError::WriteError)
  }

  /// `std` Decompress the input data into the output.
  ///
  /// The buffer, with `1 << EI` bytes, is allocated on the heap.
  #[cfg(any(doc, test, feature = "std"))]
  pub fn decompress<R: Read, W: Write>(
    &self,
    mut reader: R,
    mut writer: W,
  ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
    let mut buffer = vec![self.c; self.n()];
    self.decompress_internal(&mut reader, &mut writer, &mut buffer)?;
    writer.finish().map_err(LzssError::WriteError)
  }

  /// Decompress the input data into the output.
  ///
  /// It will be asserted at runtime that the buffer is at least `1 << EI`.
  pub fn decompress_with_buffer<R: Read, W: Write>(
    &self,
    mut reader: R,
    mut writer: W,
    buffer: &mut [u8],
  ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
    assert!(buffer.len() >= self.n());
    unsafe { ::core::ptr::write_bytes(buffer.as_mut_ptr(), self.c, self.n()) };
    self.decompress_internal(&mut reader, &mut writer, buffer)?;
    writer.finish().map_err(LzssError::WriteError)
  }
}

/// The error returned by [LzssDyn::new].
#[derive(Debug)]
pub enum LzssDynError {
  /// Invalid EJ, must be larger than 0.
  EjIsZero,
  /// Invalid EI, must be larger than EJ.
  EiNotLargerThanEj,
  /// Invalid EI, EJ, both together must be 8 or more.
  EiEjToSmall,
  /// Invalid EI, EJ, both together must be 24 or less.
  EiEjToLarge,
}

impl core::fmt::Display for LzssDynError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      LzssDynError::EjIsZero => f.write_str("Invalid EJ, must be larger than 0"),
      LzssDynError::EiNotLargerThanEj => f.write_str("Invalid EI, must be larger than EJ"),
      LzssDynError::EiEjToSmall => f.write_str("Invalid EI, EJ, both together must be 8 or more"),
      LzssDynError::EiEjToLarge => f.write_str("Invalid EI, EJ, both together must be 24 or less"),
    }
  }
}

/// `std` Implementation of [Error](std::error::Error) for [LzssDynError]
#[cfg(any(doc, test, feature = "std"))]
impl std::error::Error for LzssDynError {}

#[cfg(test)]
mod tests {
  use crate::dynamic::LzssDyn;
  use crate::slice::SliceReader;
  use crate::vec::VecWriter;
  use crate::ResultLzssErrorVoidExt;

  fn test_lzss() -> LzssDyn {
    LzssDyn::new(10, 4, 0x20).unwrap()
  }

  const TEST_DATA: &[u8; 27] = b"Sample   Data   11221233123";
  const COMPRESSED_DATA: [u8; 26] = [
    169, 216, 109, 183, 11, 101, 149, 246, 13, 18, 195, 116, 176, 191, 81, 152, 204, 102, 83, 32,
    0, 19, 57, 152, 3, 16,
  ];

  #[test]
  fn test_decompress() {
    let output = test_lzss()
      .decompress(
        SliceReader::new(&COMPRESSED_DATA),
        VecWriter::with_capacity(TEST_DATA.len()),
      )
      .void_unwrap();
    assert_eq!(output.as_slice(), TEST_DATA);
  }

  #[test]
  fn test_compress() {
    let output = test_lzss()
      .compress(
        SliceReader::new(TEST_DATA),
        VecWriter::with_capacity(COMPRESSED_DATA.len()),
      )
      .void_unwrap();
    assert_eq!(output.as_slice(), COMPRESSED_DATA);
  }

  #[test]
  fn test_compress_big() {
    let big_test_data = include_bytes!("mod.rs");
    // compress
    let output1 = test_lzss()
      .compress(
        SliceReader::new(big_test_data),
        VecWriter::with_capacity(big_test_data.len()),
      )
      .void_unwrap();
    // decompress
    let output2 = test_lzss()
      .decompress(
        SliceReader::new(&output1),
        VecWriter::with_capacity(big_test_data.len()),
      )
      .void_unwrap();
    assert_eq!(output2.as_slice(), big_test_data);
  }

  #[test]
  fn test_decompress_with_buffer() {
    let mut buffer = [0u8; 1111];
    let output = test_lzss()
      .decompress_with_buffer(
        SliceReader::new(&COMPRESSED_DATA),
        VecWriter::with_capacity(TEST_DATA.len()),
        &mut buffer,
      )
      .void_unwrap();
    assert_eq!(output.as_slice(), TEST_DATA);
  }

  #[test]
  fn test_compress_with_buffer() {
    let mut buffer = [0u8; 2222];
    let output = test_lzss()
      .compress_with_buffer(
        SliceReader::new(TEST_DATA),
        VecWriter::with_capacity(COMPRESSED_DATA.len()),
        &mut buffer,
      )
      .void_unwrap();
    assert_eq!(output.as_slice(), COMPRESSED_DATA);
  }
}
