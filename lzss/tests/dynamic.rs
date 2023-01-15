use common::{EXAMPLE_DATA, INIT_BYTE};
use lzss::{LzssDyn, ResultLzssErrorVoidExt, SliceReader, VecWriter};

mod common;

fn combinations() -> Vec<(usize, usize, bool)> {
    let mut result = Vec::new();

    // The Rules:
    // * `ej` must be larger than `0`
    // * `ei` must be larger than `ej`
    // * `ei + ej` must be at least 8
    // * `ei + ej` must be 24 or less

    for ej in 0..=12 {
        for ei in 0..=24 {
            result.push((ei, ej, ej > 0 && ei > ej && ei + ej >= 8 && ei + ej <= 24));
        }
    }

    result
}

#[test]
#[ignore]
fn dynamic() {
    debug_assert!(false, "Disabled in debug mode");
    for (ei, ej, is_valid) in combinations() {
        let params = LzssDyn::new(ei, ej, INIT_BYTE);
        assert_eq!(
            is_valid,
            params.is_ok(),
            "LzssDyn<{ei},{ej},0x{INIT_BYTE:02x}>::new returned the wrong case"
        );
        if let Ok(lzss) = params {
            let encoded = lzss
                .compress(
                    SliceReader::new(EXAMPLE_DATA),
                    VecWriter::with_capacity(EXAMPLE_DATA.len()),
                )
                .void_unwrap();
            let decoded = lzss
                .decompress(
                    SliceReader::new(&encoded),
                    VecWriter::with_capacity(EXAMPLE_DATA.len()),
                )
                .void_unwrap();
            assert_eq!(
                EXAMPLE_DATA,
                &decoded[..],
                "LzssDyn<{ei},{ej},0x{INIT_BYTE:02x}> Data mismatch"
            );
        }
    }
}
