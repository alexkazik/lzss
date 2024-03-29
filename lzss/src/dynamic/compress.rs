/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

// Notice: generic/compress.rs is generated from this file, see build.rs.

use crate::bits::BitWriter;
use crate::dynamic::LzssDyn;
use crate::error::LzssError;
use crate::macros::{get, search_loop, set};
use crate::read_write::{Read, Write};

impl LzssDyn {
    // Allow many single char names, this is done to copy the original code as close as possible.
    #![allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn compress_internal<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
        buffer: &mut [u8],
    ) -> Result<(), LzssError<R::Error, W::Error>> {
        // It is already ensured that EI+EJ are "reasonable"
        // And for generic: 1<<EI == N and 2*N == N2
        // And for dynamic: the buffer has the correct size

        let mut bit_writer = BitWriter::new(writer);

        let mut buffer_end = self.n() - self.f();
        while buffer_end < 2 * self.n() {
            match reader.read().map_err(LzssError::ReadError)? {
                None => break,
                Some(data) => {
                    set!(buffer, buffer_end, data);
                    buffer_end += 1;
                }
            }
        }

        let mut r = self.n() - self.f();
        let mut s = 0;
        while r < buffer_end {
            let f1 = self.f().min(buffer_end - r);
            let mut x = 0;
            let mut y = 1;
            let c = get!(buffer, r);
            for (i, &ci) in search_loop!(s, r, buffer) {
                if ci == c {
                    let mut j = 1;
                    while j < f1 {
                        if get!(buffer, i + j) != get!(buffer, r + j) {
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
            if y <= self.p() {
                bit_writer
                    .write_bits(0x100 | u32::from(c), 9)
                    .map_err(LzssError::WriteError)?;
                y = 1;
            } else {
                bit_writer
                    .write_bits(
                        (((x & (self.n() - 1)) as u32) << self.ej) | ((y - (self.p() + 1)) as u32),
                        1 + self.ei + self.ej,
                    )
                    .map_err(LzssError::WriteError)?;
            }
            r += y;
            s += y;
            if r >= self.n() * 2 - self.f() {
                buffer.copy_within(self.n()..2 * self.n(), 0);
                buffer_end -= self.n();
                r -= self.n();
                s -= self.n();
                while buffer_end < 2 * self.n() {
                    match reader.read().map_err(LzssError::ReadError)? {
                        None => break,
                        Some(data) => {
                            set!(buffer, buffer_end, data);
                            buffer_end += 1;
                        }
                    }
                }
            }
        }
        bit_writer.flush().map_err(LzssError::WriteError)
    }
}
