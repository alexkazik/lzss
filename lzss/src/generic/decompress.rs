/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::bits::BitReader;
use crate::generic::Lzss;
use crate::read_write::{Read, Write};
use crate::LzssError;

impl<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize>
    Lzss<EI, EJ, C, N, N2>
{
    // Allow many single char names, this is done to copy the original code as close as possible.
    #![allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn decompress_internal<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W,
        buffer: &mut [u8; N],
    ) -> Result<(), LzssError<R::Error, W::Error>> {
        // It is already ensured that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut bit_reader = BitReader::new(reader);

        let mut r = N - Self::F;
        loop {
            if let Some(inp) = bit_reader.read_bits(9).map_err(LzssError::ReadError)? {
                if (inp & 0x100) != 0 {
                    writer.write(inp as u8).map_err(LzssError::WriteError)?;
                    *unsafe { buffer.get_unchecked_mut(r) } = inp as u8;
                    r = (r + 1) & (N - 1);
                } else if let Some(inp2) = bit_reader
                    .read_bits(EI + EJ - 8)
                    .map_err(LzssError::ReadError)?
                {
                    let inp = (inp << (EI + EJ - 8)) | inp2;
                    let i = (inp >> EJ) as usize;
                    let j = (inp & ((1 << EJ) - 1)) as usize;
                    for k in 0..=j + Self::P {
                        let b = *unsafe { buffer.get_unchecked((i + k) & (N - 1)) };
                        writer.write(b).map_err(LzssError::WriteError)?;
                        *unsafe { buffer.get_unchecked_mut(r) } = b;
                        r = (r + 1) & (N - 1);
                    }
                } else {
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        }
    }
}
