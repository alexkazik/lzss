/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::generic::Lzss;

impl<
    const EI: usize,
    const EJ: usize,
    const C: u8,
    const UNSAFE_N: usize,
    const UNSAFE_N2: usize,
  > Lzss<EI, EJ, C, UNSAFE_N, UNSAFE_N2>
{
  #[inline(always)]
  pub(crate) fn compress_in_place_internal(io: &mut [u8], offset: usize) -> (usize, Option<usize>) {
    #[cfg(not(feature = "const_panic"))]
    Self::assert_parameters();
    if offset >= io.len() {
      return (0, None);
    }
    if offset < Self::MIN_OFFSET {
      return (0, Some(offset));
    }

    io[offset - (Self::N - Self::F)..offset].fill(C);

    let mut out_buf = 0;
    let mut out_len = 0;
    let mut out_ptr = 0;
    let mut s = offset - (Self::N - Self::F);
    let mut r = offset;
    let offset2 = Self::N * (1 + (offset + Self::F) / Self::N) - (offset + Self::F);

    while r < io.len() {
      let f1 = Self::F.min(io.len() - r);
      let c = io[r];
      let mut x = 0;
      let mut y = 1;
      for (i, &ci) in (s..r).zip(&io[s..r]).rev() {
        if ci == c {
          let mut j = 1;
          while j < f1 {
            if io[i + j] != io[r + j] {
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
          | (((x + offset2) & (Self::N - 1)) << EJ)
          | (y - (Self::P + 1));
        out_len += 1 + EI + EJ;
      }
      while out_len > 8 {
        out_len -= 8;
        io[out_ptr] = (out_buf >> out_len) as u8;
        out_ptr += 1;
      }

      r += y;
      s += y;

      if out_ptr + Self::MIN_GAP_SIZE > s {
        if out_len > 0 {
          io[out_ptr] = (out_buf << (8 - out_len)) as u8;
          out_ptr += 1;
        }
        return (out_ptr, Some(r));
      }
    }

    if out_len > 0 {
      io[out_ptr] = (out_buf << (8 - out_len)) as u8;
      out_ptr += 1;
    }
    (out_ptr, None)
  }
}
