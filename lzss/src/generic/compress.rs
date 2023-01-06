/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::bits::BitWriter;
use crate::generic::Lzss;
use crate::read_write::{Read, Write};
use crate::LzssError;

impl<
    const EI: usize,
    const EJ: usize,
    const C: u8,
    const UNSAFE_N: usize,
    const UNSAFE_N2: usize,
  > Lzss<EI, EJ, C, UNSAFE_N, UNSAFE_N2>
{
  #[inline(always)]
  pub(crate) fn compress_internal<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    buffer: &mut [u8; UNSAFE_N2],
  ) -> Result<(), LzssError<R::Error, W::Error>> {
    let mut bit_writer = BitWriter::new(writer);

    let mut buffer_end = Self::N - Self::F;
    while buffer_end < 2 * Self::N {
      match reader.read().map_err(LzssError::ReadError)? {
        None => break,
        Some(data) => {
          buffer[buffer_end] = data;
          buffer_end += 1;
        }
      }
    }

    let mut r = Self::N - Self::F;
    let mut s = 0;
    while r < buffer_end {
      let f1 = Self::F.min(buffer_end - r);
      let mut x = 0;
      let mut y = 1;
      let c = buffer[r];
      for (i, &ci) in (s..r).zip(&buffer[s..r]).rev() {
        if ci == c {
          let mut j = 1;
          while j < f1 {
            if buffer[i + j] != buffer[r + j] {
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
            (((x & (Self::N - 1)) as u32) << EJ) | ((y - (Self::P + 1)) as u32),
            1 + EI + EJ,
          )
          .map_err(LzssError::WriteError)?;
      }
      r += y;
      s += y;
      if r >= Self::N * 2 - Self::F {
        buffer.copy_within(Self::N..2 * Self::N, 0);
        buffer_end -= Self::N;
        r -= Self::N;
        s -= Self::N;
        while buffer_end < 2 * Self::N {
          match reader.read().map_err(LzssError::ReadError)? {
            None => break,
            Some(data) => {
              buffer[buffer_end] = data;
              buffer_end += 1;
            }
          }
        }
      }
    }
    bit_writer.flush().map_err(LzssError::WriteError)
  }
}
