use honggfuzz::fuzz;
use lzss::{Lzss, SliceReader, VecWriter, ResultLzssErrorVoidExt};

const EI: usize = 10;
const EJ: usize = 4;
type MyLzss = Lzss<EI, EJ, 0x20, {1 << EI}, {2 << EI}>;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            MyLzss::compress_heap(
                SliceReader::new(data),
                VecWriter::with_capacity(data.len()),
            ).void_unwrap();
        });
    }
}
