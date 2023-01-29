[![Build Status](https://github.com/alexkazik/lzss/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/alexkazik/lzss/actions?query=workflow%3ACI+branch%3Amaster+event%3Apush)
[![Dependency status](https://deps.rs/repo/github/alexkazik/lzss/status.svg)](https://deps.rs/repo/github/alexkazik/lzss)
[![crates.io](https://img.shields.io/crates/v/lzss.svg)](https://crates.io/crates/lzss)
[![Downloads](https://img.shields.io/crates/d/lzss.svg)](https://crates.io/crates/lzss)
[![Github stars](https://img.shields.io/github/stars/alexkazik/lzss.svg?logo=github)](https://github.com/alexkazik/lzss/stargazers)
[![License](https://img.shields.io/crates/l/lzss.svg)](./LICENSE)

# crate lzss

<!-- cargo-rdme start -->

## Lempel–Ziv–Storer–Szymanski de-/compression

`lzss` is a lossless data compression algorithm in pure Rust.
This crate is built for embedded systems:

* Small code size
* Uses little RAM and CPU
* `no_std` feature
* All parameters can be compile-time only

## Generic vs. dynamic

This crate comes in two flavors: generic (`Lzss`) and dynamic (`LzssDyn`).

The dynamic one has one compress function and all parameters are passed to
it at runtime, making it very adaptive.

The generic one has compile-time parameters will produce a function for each
different sets of parameters. This function will be more optimized by the
compiler than the dynamic one, the downside is that multiple functions are
generated when multiple parameter sets are used.

(The same applies for decompress and other functions, only used function will
be in the generated program.)

## Lack of a header

This algorithm has by design no header at all. Please be aware that it is not
possible to check if the contents is correct, or even the length matches.
It is recommended to add a header based on the requirements.

## Origin
This code is based on the [LZSS encoder-decoder by Haruhiko Okumura, public domain](https://oku.edu.mie-u.ac.jp/~okumura/compression/lzss.c).

In order to create an encoder-decoder which is compatible to the program above
the following is required: `C = 0x20` in this library and `P = (1+EI+EJ) / 9` in Okumuras program.

## Features
* `alloc`       - Allows de-/compression with buffer on the heap and the `VecWriter`.
* `safe`        - Only use safe code (see Safety below).
* `std`         - Enables `alloc` and additional `IOSimpleReader`, `IOSimpleWriter`,
                  and the `Error` instance for `LzssError` and `LzssDynError`.

`std` and `safe` are enabled by default.

### Usage
With defaults (`std` and `safe`):
```toml
[dependencies]
lzss = "0.8"
```

With `no_std` (and without `safe`):
```toml
[dependencies]
lzss = { version = "0.8", default-features = false }
```

## Example
```rust
type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
let input = b"Example Data";
let mut output = [0; 30];
let result = MyLzss::compress(
  SliceReader::new(input),
  SliceWriter::new(&mut output),
);
assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
```

## Safety

With the `safe` feature the code is not using any unsafe code (`forbid(unsafe_code)`), but at
the cost of performance and size - though on modern systems that is not to mention.

But on smaller systems (like microcontrollers, where `no_std` is needed) it may be noticeable.
Which is the reason wht it can be switched on/off.

<!-- cargo-rdme end -->

# Command-Line-Interface

In oder to de-/compress files in the cli, install lzss-cli:

```shell
cargo install lzss-cli
```

Example:
```shell
lzss e 10,4,0x20 <input >outout
```
