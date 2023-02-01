use crate::dynamic::LzssDyn;
use crate::error::LzssError;
use crate::read_write::{Read, Write};
use core::convert::Infallible;
#[cfg(feature = "safe")]
use core::convert::TryInto;

mod compress;
mod compress_in_place;
mod decompress;

/// A zero-sized type, the const generics specify the parameters of the compression.
///
/// # Parameters
/// * `EI` - The number of bits in the offset, usually `10..13`
/// * `EJ` - The number of bits in the length, usually `4..5`
/// * `C` - The initial fill byte of the buffer, usually `0x20` (space)
/// * `N` - Equals `1 << EI`, the size of the buffer for [`Lzss::decompress`]
/// * `N2` - Equals `2 << EI` (`N * 2`), the size of the buffer for [`Lzss::compress`]
///
/// # Restrictions
/// * `EJ` must be larger than `0`
/// * `EI` must be larger than `EJ`
/// * `EI + EJ` must be at least 8
/// * `EI + EJ` must be 24 or less
/// * `N` must be equal to `1 << EI`
/// * `N2` must be equal to `2 << EI` (`N * 2`)
///
/// All parameters are checked at compile-time.
///
/// There is no runtime overhead since everything is checked during compile-time.
///
/// # Limitations
/// Since it's not possible to do const calculations on const generics all parameters
/// have to be set.
///
/// # Example
/// ```rust
/// # use lzss::{Lzss, SliceReader, SliceWriterExact};
/// type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
/// let input = b"Example Data";
/// let mut output = [0; 14];
/// let result = MyLzss::compress_stack(
///   SliceReader::new(input),
///   SliceWriterExact::new(&mut output),
/// );
/// assert!(result.is_ok()); // the output is exactly 14 bytes long
/// ```

pub struct Lzss<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize>(
    Infallible,
);

impl<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize>
    Lzss<EI, EJ, C, N, N2>
{
    /// Create a new [`LzssDyn`] with the parameter from this generic type.
    ///
    /// This is mainly useful for creating const [`LzssDyn`].
    ///
    /// ```
    /// # use lzss::{Lzss, LzssDyn};
    /// type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
    /// const MY_DYN1: LzssDyn = MyLzss::as_dyn();
    /// // or
    /// const MY_DYN2: LzssDyn = Lzss::<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>::as_dyn();
    /// ```
    #[must_use]
    pub const fn as_dyn() -> LzssDyn {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        LzssDyn {
            ei: EI,
            ej: EJ,
            c: C,
        }
    }

    /// Compress the input data into the output.
    ///
    /// The buffer, with `N2` bytes, is allocated on the stack.
    #[inline(always)]
    #[deprecated(since = "0.9.0", note = "renamed to compress_stack")]
    pub fn compress<R: Read, W: Write>(
        reader: R,
        writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        Self::compress_stack(reader, writer)
    }

    /// Compress the input data into the output.
    ///
    /// The buffer, with `N2` bytes, is allocated on the stack.
    pub fn compress_stack<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut buffer = [C; N2];
        Self::compress_internal(&mut reader, &mut writer, &mut buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Compress the input data into the output.
    ///
    /// The buffer, with `N2` bytes, is allocated on the heap.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[cfg(feature = "alloc")]
    pub fn compress_heap<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut buffer = vec![C; N2];
        #[cfg(not(feature = "safe"))]
        let buffer = unsafe { &mut *(buffer.as_mut_ptr().cast::<[u8; N2]>()) };
        #[cfg(feature = "safe")]
        let buffer: &mut [u8; N2] = (&mut buffer[..]).try_into().unwrap();
        Self::compress_internal(&mut reader, &mut writer, buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Compress the input data into the output.
    pub fn compress_with_buffer<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
        buffer: &mut [u8; N2],
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        buffer[..N - Self::F].fill(C);
        Self::compress_internal(&mut reader, &mut writer, buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Decompress the input data into the output.
    ///
    /// The buffer, with `N` bytes, is allocated on the stack.
    #[inline(always)]
    #[deprecated(since = "0.9.0", note = "renamed to decompress_stack")]
    pub fn decompress<R: Read, W: Write>(
        reader: R,
        writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        Self::decompress_stack(reader, writer)
    }

    /// Decompress the input data into the output.
    ///
    /// The buffer, with `N` bytes, is allocated on the stack.
    pub fn decompress_stack<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut buffer: [u8; N] = [C; N];
        Self::decompress_internal(&mut reader, &mut writer, &mut buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Decompress the input data into the output.
    ///
    /// The buffer, with `N` bytes, is allocated on the heap.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[cfg(feature = "alloc")]
    pub fn decompress_heap<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut buffer = vec![C; N];
        #[cfg(not(feature = "safe"))]
        let buffer = unsafe { &mut *(buffer.as_mut_ptr().cast::<[u8; N]>()) };
        #[cfg(feature = "safe")]
        let buffer: &mut [u8; N] = (&mut buffer[..]).try_into().unwrap();
        Self::decompress_internal(&mut reader, &mut writer, buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Decompress the input data into the output.
    pub fn decompress_with_buffer<R: Read, W: Write>(
        mut reader: R,
        mut writer: W,
        buffer: &mut [u8; N],
    ) -> Result<W::Output, LzssError<R::Error, W::Error>> {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        buffer[..N].fill(C);
        Self::decompress_internal(&mut reader, &mut writer, buffer)?;
        writer.finish().map_err(LzssError::WriteError)
    }

    /// Compress, the input and output is in the same slice.
    ///
    /// The input is located at `io[offset..]`.
    /// When there is enough space in the slice then the result will be `(size, None)`.
    /// And the output is located at `io[0..size]`.
    ///
    /// If there is not enough space in the slice, i.e. the output (or buffer) would overwrite
    /// the input, then the result will be `(size, Some(new_offset))`, the already compressed
    /// data is in `io[0..size]` and the not yet compressed data is in `io[new_offset..]`.
    ///
    /// Even when the compression fails due to space the data is recoverable.
    ///
    /// The minimum offset is [`Lzss::MIN_OFFSET`], though if the offset is `Lzss::MIN_OFFSET + input_size/8`
    /// then the compression can't fail.
    pub fn compress_in_place(io: &mut [u8], offset: usize) -> (usize, Option<usize>) {
        let _ = Self::ASSERT_PARAMETERS; // This ensures that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        Self::compress_in_place_internal(io, offset)
    }

    /// The minimal offset when using `compress_in_place`.
    ///
    /// It's a little less than `N`.
    pub const MIN_OFFSET: usize = (N - Self::F) + Self::MIN_GAP_SIZE;

    // non-public helpers

    pub(crate) const P: usize = (1 + EI + EJ) / 9; /* If match length <= P then output one character */
    pub(crate) const F: usize = (1 << EJ) + Self::P; /* lookahead buffer size */
    pub(crate) const MIN_GAP_SIZE: usize = Self::P + 4;

    const ASSERT_PARAMETERS: Result<(), ()> = {
        if EJ == 0 {
            panic!("LZSS: Invalid EJ, must be larger than 0")
        }
        if EJ >= EI {
            panic!("LZSS: Invalid EI, must be larger than EJ")
        }
        if EI + EJ < 8 {
            panic!("LZSS: Invalid EI, EJ, both together must be 8 or more")
        }
        if EI + EJ > 24 {
            panic!("LZSS: Invalid EI, EJ, both together must be 24 or less")
        }
        // the conversion to u32 is for the check to work on 16-bit systems
        if (N as u32) != (1u32 << EI) {
            panic!("LZSS: Invalid N, must be exactly 1<<EI")
        }
        // the conversion to u32 is for the check to work on 16-bit systems
        if (N2 as u32) != 2 * (N as u32) {
            panic!("LZSS: Invalid N2, must be exactly 2*N")
        }
        Ok(())
    };
}

#[cfg(test)]
mod tests {
    use crate::generic::Lzss;
    use crate::slice::SliceReader;
    use crate::vec::VecWriter;
    use crate::ResultLzssErrorVoidExt;

    type TestLZSS = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;

    const TEST_DATA: &[u8; 27] = b"Sample   Data   11221233123";
    const COMPRESSED_DATA: [u8; 26] = [
        169, 216, 109, 183, 11, 101, 149, 246, 13, 18, 195, 116, 176, 191, 81, 152, 204, 102, 83,
        32, 0, 19, 57, 152, 3, 16,
    ];

    #[test]
    fn test_decompress() {
        let output = TestLZSS::decompress_stack(
            SliceReader::new(&COMPRESSED_DATA),
            VecWriter::with_capacity(TEST_DATA.len()),
        )
        .void_unwrap();
        assert_eq!(output.as_slice(), TEST_DATA);
    }

    #[test]
    fn test_decompress_with_buffer() {
        let mut buffer = [123; 1024];
        let output = TestLZSS::decompress_with_buffer(
            SliceReader::new(&COMPRESSED_DATA),
            VecWriter::with_capacity(TEST_DATA.len()),
            &mut buffer,
        )
        .void_unwrap();
        assert_eq!(output.as_slice(), TEST_DATA);
    }

    #[test]
    fn test_compress() {
        let output = TestLZSS::compress_stack(
            SliceReader::new(TEST_DATA),
            VecWriter::with_capacity(COMPRESSED_DATA.len()),
        )
        .void_unwrap();
        assert_eq!(output.as_slice(), COMPRESSED_DATA);
    }

    #[test]
    fn test_compress_with_buffer() {
        let mut buffer = [123; 2048];
        let output = TestLZSS::compress_with_buffer(
            SliceReader::new(TEST_DATA),
            VecWriter::with_capacity(COMPRESSED_DATA.len()),
            &mut buffer,
        )
        .void_unwrap();
        assert_eq!(output.as_slice(), COMPRESSED_DATA);
    }

    #[test]
    fn test_compress_in_place() {
        const OFFSET: usize = TestLZSS::MIN_OFFSET + TEST_DATA.len() / 8;
        let mut io = [0u8; OFFSET + TEST_DATA.len()];
        io[OFFSET..].copy_from_slice(TEST_DATA);
        let (c, u) = TestLZSS::compress_in_place(&mut io, OFFSET);
        assert_eq!(c, COMPRESSED_DATA.len());
        assert_eq!(u, None);
        assert_eq!(io[0..c], COMPRESSED_DATA);
    }

    #[test]
    fn test_compress_big() {
        let big_test_data = include_bytes!("mod.rs");
        // compress
        let output1 = TestLZSS::compress_stack(
            SliceReader::new(big_test_data),
            VecWriter::with_capacity(big_test_data.len()),
        )
        .void_unwrap();
        // compress_in_place
        let offset: usize = TestLZSS::MIN_OFFSET + big_test_data.len() / 8;
        let mut io = Vec::new();
        io.resize(offset + big_test_data.len(), 0);
        let io = io.as_mut_slice();
        io[offset..].copy_from_slice(big_test_data);
        let (c, u) = TestLZSS::compress_in_place(io, offset);
        assert_eq!(u, None);
        // compare both
        assert_eq!(output1.as_slice(), &io[0..c]);
        // decompress
        let output2 = TestLZSS::decompress_stack(
            SliceReader::new(&io[0..c]),
            VecWriter::with_capacity(big_test_data.len()),
        )
        .void_unwrap();
        assert_eq!(output2.as_slice(), big_test_data);
    }
}
