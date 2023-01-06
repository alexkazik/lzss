use honggfuzz::fuzz;
use lzss::{SliceReader, VecWriter, ResultLzssErrorVoidExt, LzssDyn};

const EI: usize = 10;
const EJ: usize = 4;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            LzssDyn::new(EI, EJ, 0x20).unwrap()
                .compress(
                    SliceReader::new(data),
                    VecWriter::with_capacity(data.len()),
                )
                .void_unwrap();
        });
    }
}
