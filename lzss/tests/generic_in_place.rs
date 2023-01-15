use common::{EXAMPLE_DATA, INIT_BYTE};
use lzss::{Lzss, ResultLzssErrorVoidReadExt, SliceReader, SliceWriter};

mod common;

#[test]
fn compress_in_place() {
    const EI: usize = 8;
    const EJ: usize = 4;
    type MyLzss = Lzss<EI, EJ, INIT_BYTE, { 1 << EI }, { 2 << EI }>;

    let mut decompressed = vec![0; EXAMPLE_DATA.len()];
    for offset in MyLzss::MIN_OFFSET - 1..=2 << EI {
        let mut buffer = vec![!INIT_BYTE; (2 << EI) + EXAMPLE_DATA.len()];
        let compressed = &mut buffer[0..offset + EXAMPLE_DATA.len()];
        // copy input into the buffer
        compressed[offset..].copy_from_slice(EXAMPLE_DATA);
        // compress
        let (compressed_len, new_offset) = MyLzss::compress_in_place(compressed, offset);
        // decompress the successfully compressed data
        match MyLzss::decompress(
            SliceReader::new(&compressed[0..compressed_len]),
            SliceWriter::new(&mut decompressed),
        )
        .void_read_unwrap()
        {
            Ok(decompressed_len) => {
                // if there was a remaining not compressed data then check the size and copy it
                if let Some(new_offset) = new_offset {
                    assert_eq!(
                        decompressed_len + (compressed.len() - new_offset),
                        EXAMPLE_DATA.len(),
                        "LZSS::compress_in_place(offset={offset}): decompressed size error"
                    );
                    decompressed[decompressed_len..].copy_from_slice(&compressed[new_offset..]);
                }
                // check if the data matches
                assert_eq!(
                    &decompressed[..],
                    EXAMPLE_DATA,
                    "LZSS::compress_in_place(offset={offset}): decompressed data does not match"
                );
            }
            Err(error) => {
                panic!("LZSS::compress_in_place(offset={offset}): decompress failed: {error}")
            }
        }
    }
}
