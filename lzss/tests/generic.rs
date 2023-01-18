use common::{EXAMPLE_DATA, INIT_BYTE};
use lzss::{Lzss, ResultLzssErrorVoidExt, SliceReader, VecWriter};

mod common;

macro_rules! test_generic {
    ($name:ident, $ei:expr, $ej:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            debug_assert!(false, "Disabled in debug mode");

            type MyLzss = Lzss<$ei, $ej, INIT_BYTE, { 1 << $ei }, { 2 << $ei }>;

            // compress and decompress
            let compressed = MyLzss::compress_heap(
                SliceReader::new(EXAMPLE_DATA),
                VecWriter::with_capacity(EXAMPLE_DATA.len()),
            )
            .void_unwrap();
            let decompressed = MyLzss::decompress_heap(
                SliceReader::new(&compressed),
                VecWriter::with_capacity(EXAMPLE_DATA.len()),
            )
            .void_unwrap();

            // check if the decompressed matches the original
            assert_eq!(EXAMPLE_DATA, &decompressed[..]);

            // compress via dyn and check if the compressed is identical
            let compressed_dyn = MyLzss::as_dyn()
                .compress(
                    SliceReader::new(EXAMPLE_DATA),
                    VecWriter::with_capacity(EXAMPLE_DATA.len()),
                )
                .void_unwrap();
            assert_eq!(compressed, compressed_dyn);
        }
    };
}

// "regular"
test_generic!(generic_10_4, 10, 4);
test_generic!(generic_11_5, 11, 5);
test_generic!(generic_12_4, 12, 4);
test_generic!(generic_13_5, 13, 5);
// "extremes"
test_generic!(generic_5_3, 5, 3);
test_generic!(generic_6_2, 6, 2);
test_generic!(generic_13_11, 13, 11);
test_generic!(generic_23_1, 23, 1);
