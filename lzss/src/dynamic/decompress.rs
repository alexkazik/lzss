/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::bits::BitReader;
use crate::dynamic::LzssDyn;
use crate::{LzssError, Read, Write};

impl LzssDyn {
  #[inline(always)]
  pub(crate) fn decompress_internal<R: Read, W: Write>(
    &self,
    reader: &mut R,
    writer: &mut W,
    buffer: &mut [u8],
  ) -> Result<(), LzssError<R::Error, W::Error>> {
    let n = 1 << self.ei;
    let index_mask = n - 1;
    let buffer = &mut buffer[..n];
    let mut bit_reader = BitReader::new(reader);

    let mut r = n - self.f();
    loop {
      if let Some(c) = bit_reader.read_bits(1).map_err(LzssError::ReadError)? {
        if c != 0 {
          if let Some(b) = bit_reader.read_bits(8).map_err(LzssError::ReadError)? {
            writer.write(b as u8).map_err(LzssError::WriteError)?;
            buffer[r & index_mask] = b as u8;
            r = r.wrapping_add(1);
          } else {
            return Ok(());
          }
        } else if let Some(ij) = bit_reader
          .read_bits(self.ei + self.ej)
          .map_err(LzssError::ReadError)?
        {
          let i = (ij >> self.ej) as usize;
          let j = (ij & ((1 << self.ej) - 1)) as usize;
          for k in 0..=j + self.p() {
            let b = buffer[i.wrapping_add(k) & index_mask];
            writer.write(b).map_err(LzssError::WriteError)?;
            buffer[r & index_mask] = b;
            r = r.wrapping_add(1);
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
