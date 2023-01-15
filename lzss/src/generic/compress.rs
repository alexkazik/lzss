/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::bits::BitWriter;
use crate::generic::Lzss;
use crate::read_write::{Read, Write};
use crate::LzssError;

impl<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize>
    Lzss<EI, EJ, C, N, N2>
{
    // Allow many single char names, this is done to copy the original code as close as possible.
    #![allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn compress_internal<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W,
        buffer: &mut [u8; N2],
    ) -> Result<(), LzssError<R::Error, W::Error>> {
        // It is already ensured that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        let mut bit_writer = BitWriter::new(writer);

        let mut buffer_end = N - Self::F;
        while buffer_end < 2 * N {
            match reader.read().map_err(LzssError::ReadError)? {
                None => break,
                Some(data) => {
                    *unsafe { buffer.get_unchecked_mut(buffer_end) } = data;
                    buffer_end += 1;
                }
            }
        }

        let mut r = N - Self::F;
        let mut s = 0;
        while r < buffer_end {
            let f1 = Self::F.min(buffer_end - r);
            let mut x = 0;
            let mut y = 1;
            let c = *unsafe { buffer.get_unchecked(r) };
            for i in (s..r).rev() {
                if *unsafe { buffer.get_unchecked(i) } == c {
                    let mut j = 1;
                    while j < f1 {
                        if *unsafe { buffer.get_unchecked(i + j) }
                            != *unsafe { buffer.get_unchecked(r + j) }
                        {
                            break;
                        }
                        j += 1;
                    }
                    if j > y {
                        x = i;
                        y = j;
                    }
                }
            }
            if y <= Self::P {
                bit_writer
                    .write_bits(0x100 | u32::from(c), 9)
                    .map_err(LzssError::WriteError)?;
                y = 1;
            } else {
                bit_writer
                    .write_bits(
                        (((x & (N - 1)) as u32) << EJ) | ((y - (Self::P + 1)) as u32),
                        1 + EI + EJ,
                    )
                    .map_err(LzssError::WriteError)?;
            }
            r += y;
            s += y;
            if r >= N * 2 - Self::F {
                buffer.copy_within(N..2 * N, 0);
                buffer_end -= N;
                r -= N;
                s -= N;
                while buffer_end < 2 * N {
                    match reader.read().map_err(LzssError::ReadError)? {
                        None => break,
                        Some(data) => {
                            *unsafe { buffer.get_unchecked_mut(buffer_end) } = data;
                            buffer_end += 1;
                        }
                    }
                }
            }
        }
        bit_writer.flush().map_err(LzssError::WriteError)
    }
}
