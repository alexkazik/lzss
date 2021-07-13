# Lempel–Ziv–Storer–Szymanski de-/compression

`lzss` is a lossless data compression algorithm in pure Rust.
This crate is built for embedded systems:

* Small code size
* Uses little RAM and CPU
* `no_std` feature
* All parameters can be compile-time only

# Generic vs. dynamic

This crate comes in two flavors: generic (Lzss) and dynamic (LzssDyn).

The dynamic one has one compress function and all parameters are passed to
it at runtime, making it very adaptive.

The generic one has compile-time parameters will produce a function for each
different sets of parameters. This function will be more optimized by the
compiler than the dynamic one, the downside is that multiple functions are
generated when multiple parameter sets are used.

(The same applies for decompress and other functions, only used function will
be in the generated program.)

# Lack of a header

This algorithm has by design no header at all. Please be aware that it is not
possible to check if the contents is correct, or even the length matches.
It is recommended to add a header based on the requirements.


# Origin
This code is based on the [LZSS encoder-decoder by Haruhiko Okumura, public domain](https://oku.edu.mie-u.ac.jp/~okumura/compression/lzss.c).

# Features
* `std`         - Enables everything marked with `std`
                  and the `Error` instance for `LzssError` and `LzssDynError`.
* `const_panic` - Requires nightly and enables compile-time
                  checks of the parameters for Lzss.

# Example
```rust
use lzss::{Lzss, SliceReader, SliceWriter};

type MyLzss = Lzss<10, 4, 0x20, {1 << 10}, {2 << 10}>;
let input = b"Example Data";
let mut output = [0; 30];
let result = MyLzss::compress(
  SliceReader::new(input),
  SliceWriter::new(&mut output),
);
assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
```

# Command-Line-Interface

In oder to de-/compress files in the cli, install lzss-cli:

```shell
cargo install lzss-cli
```

Example:
```shell
lzss e 10,4,0x20 <input >outout
```
