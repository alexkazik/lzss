use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use lzss::{Lzss, VecWriter, SliceReader, ResultLzssErrorVoidExt};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encompress heap example", |b| b.iter_batched(
        || (SliceReader::new(EXAMPLE_DATA), VecWriter::with_capacity(EXAMPLE_DATA.len())),
        |(r, w)| {
            MyLzss::compress_heap(r, w).void_unwrap()
        },
        BatchSize::SmallInput,
    ));
    c.bench_function("encompress in-place example", |b| b.iter_batched(
        || {
            let mut v = vec![0; MyLzss::MIN_OFFSET + EXAMPLE_DATA.len() / 8];
            v.extend(EXAMPLE_DATA);
            v
        },
        |mut v| {
            let (_, end) = MyLzss::compress_in_place(black_box(&mut v), MyLzss::MIN_OFFSET + EXAMPLE_DATA.len() / 8);
            assert!(end.is_none());
        },
        BatchSize::SmallInput,
    ));
    c.bench_function("decompress heap example", |b| b.iter_batched(
        || {
            let compressed = MyLzss::compress_heap(
                SliceReader::new(EXAMPLE_DATA),
                VecWriter::with_capacity(EXAMPLE_DATA.len()),
            ).void_unwrap();
            (compressed, VecWriter::with_capacity(EXAMPLE_DATA.len()))
        },
        |(r, w)| {
            MyLzss::decompress_heap(SliceReader::new(&r), w).void_unwrap()
        },
        BatchSize::SmallInput,
    ));
}

const EI: usize = 10;
const EJ: usize = 4;
type MyLzss = Lzss<EI, EJ, 0x20, {1 << EI}, {2 << EI}>;

const EXAMPLE_DATA: &[u8; 665] = br#"
/* LZSS encoder-decoder (Haruhiko Okumura; public domain) */

void decode(void)
{
    int i, j, k, r, c;

    for (i = 0; i < N - F; i++) buffer[i] = ' ';
    r = N - F;
    while ((c = getbit(1)) != EOF) {
        if (c) {
            if ((c = getbit(8)) == EOF) break;
            fputc(c, outfile);
            buffer[r++] = c;  r &= (N - 1);
        } else {
            if ((i = getbit(EI)) == EOF) break;
            if ((j = getbit(EJ)) == EOF) break;
            for (k = 0; k <= j + 1; k++) {
                c = buffer[(i + k) & (N - 1)];
                fputc(c, outfile);
                buffer[r++] = c;  r &= (N - 1);
            }
        }
    }
}
"#;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
