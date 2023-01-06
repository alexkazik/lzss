/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::generic::Lzss;
use crate::macros::{get, search_loop, set};

impl<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize>
    Lzss<EI, EJ, C, N, N2>
{
    // Allow many single char names, this is done to copy the original code as close as possible.
    #![allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn compress_in_place_internal(
        io: &mut [u8],
        offset: usize,
    ) -> (usize, Option<usize>) {
        // It is already ensured that EI+EJ are "reasonable", 1<<EI == N and 2*N == N2

        if offset >= io.len() {
            return (0, None);
        }
        if offset < Self::MIN_OFFSET {
            return (0, Some(offset));
        }
        io[offset - (N - Self::F)..offset].fill(C);
        let mut out_buf = 0;
        let mut out_len = 0;
        let mut out_ptr = 0;
        let mut s = offset - (N - Self::F);
        let mut r = offset;
        let offset2 = N * (1 + (offset + Self::F) / N) - (offset + Self::F);

        while r < io.len() {
            let f1 = Self::F.min(io.len() - r);
            let c = get!(io, r);
            let mut x = 0;
            let mut y = 1;
            for (i, &ci) in search_loop!(s, r, io) {
                if ci == c {
                    let mut j = 1;
                    while j < f1 {
                        if get!(io, i + j) != get!(io, r + j) {
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
                out_buf = (out_buf << 9) | 0x100 | usize::from(c);
                out_len += 9;
                y = 1;
            } else {
                out_buf = (out_buf << (1 + EI + EJ))
                    | (((x + offset2) & (N - 1)) << EJ)
                    | (y - (Self::P + 1));
                out_len += 1 + EI + EJ;
            }
            while out_len > 8 {
                out_len -= 8;
                set!(io, out_ptr, (out_buf >> out_len) as u8);
                out_ptr += 1;
            }

            r += y;
            s += y;

            if out_ptr + Self::MIN_GAP_SIZE > s {
                if out_len > 0 {
                    set!(io, out_ptr, (out_buf << (8 - out_len)) as u8);
                    out_ptr += 1;
                }
                return (out_ptr, Some(r));
            }
        }

        if out_len > 0 {
            set!(io, out_ptr, (out_buf << (8 - out_len)) as u8);
            out_ptr += 1;
        }
        (out_ptr, None)
    }
}
