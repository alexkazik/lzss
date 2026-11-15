/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

// Notice: generic/compress.rs is generated from this file, see build.rs.

use crate::bits::BitReader;
use crate::dynamic::LzssDyn;
use crate::error::LzssError;
use crate::macros::{get, set};
use crate::read_write::{Read, Write};

impl LzssDyn {
    // Allow many single char names, this is done to copy the original code as close as possible.
    #[allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn decompress_internal<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
        buffer: &mut [u8],
    ) -> Result<(), LzssError<R::Error, W::Error>> {
        // It is already ensured that EI+EJ are "reasonable"
        // And for dynamic: the buffer has the correct size

        let mut bit_reader = BitReader::new(reader);

        let mut r = self.n() - self.f();
        loop {
            if let Some(inp) = bit_reader.read_bits(9).map_err(LzssError::ReadError)? {
                if (inp & 0x100) != 0 {
                    writer.write(inp as u8).map_err(LzssError::WriteError)?;
                    set!(buffer, r, inp as u8);
                    r = (r + 1) & (self.n() - 1);
                } else if let Some(inp2) = bit_reader
                    .read_bits(self.ei + self.ej - 8)
                    .map_err(LzssError::ReadError)?
                {
                    let inp = (inp << (self.ei + self.ej - 8)) | inp2;
                    let i = (inp >> self.ej) as usize;
                    let j = (inp & ((1 << self.ej) - 1)) as usize;
                    for k in 0..=j + self.p() {
                        let b = get!(buffer, (i + k) & (self.n() - 1));
                        writer.write(b).map_err(LzssError::WriteError)?;
                        set!(buffer, r, b);
                        r = (r + 1) & (self.n() - 1);
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
