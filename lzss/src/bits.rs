use crate::read_write::{Read, Write};

pub(crate) struct BitReader<'a, R> {
    bits_in_buf: u8,
    buf: u32,
    reader: &'a mut R,
}

impl<R: Read> BitReader<'_, R> {
    #[inline(always)]
    pub(crate) fn new(reader: &mut R) -> BitReader<'_, R> {
        BitReader {
            bits_in_buf: 0,
            buf: 0,
            reader,
        }
    }

    #[inline(always)]
    pub(crate) fn read_bits(&mut self, len: usize) -> Result<Option<u32>, R::Error> {
        let len = len as u8; // len is 24 at most anyway
        while self.bits_in_buf < len {
            if let Some(val) = self.reader.read()? {
                self.buf = (self.buf << 8) | (val as u32);
                self.bits_in_buf += 8;
            } else {
                return Ok(None);
            }
        }
        self.bits_in_buf -= len;
        Ok(Some((self.buf >> self.bits_in_buf) & ((1 << len) - 1)))
    }
}

pub(crate) struct BitWriter<'a, W> {
    bits_in_buf: u8,
    buf: u32,
    writer: &'a mut W,
}

impl<W: Write> BitWriter<'_, W> {
    #[inline(always)]
    pub(crate) fn new(writer: &mut W) -> BitWriter<'_, W> {
        BitWriter {
            bits_in_buf: 0,
            buf: 0,
            writer,
        }
    }

    #[inline(always)]
    pub(crate) fn write_bits(&mut self, data: u32, len: usize) -> Result<(), W::Error> {
        let len = len as u8; // len is 24 at most anyway
        self.buf = (self.buf << len) | data;
        self.bits_in_buf += len;

        while self.bits_in_buf >= 8 {
            self.bits_in_buf -= 8;
            self.writer.write((self.buf >> self.bits_in_buf) as u8)?;
        }

        Ok(())
    }

    #[inline(always)]
    pub(crate) fn flush(&mut self) -> Result<(), W::Error> {
        if self.bits_in_buf > 0 {
            self.writer
                .write((self.buf << (8 - self.bits_in_buf)) as u8)
        } else {
            Ok(())
        }
    }
}
