#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]
#![cfg_attr(feature = "const_panic", feature(const_panic))]
// Allow many single char names, this is done to copy the original code as close as possible.
#![allow(clippy::many_single_char_names)]
#![warn(missing_docs)]

//! # Lempel–Ziv–Storer–Szymanski de-/compression
//!
//! `lzss` is a lossless data compression algorithm in pure Rust.
//! This crate is built for embedded systems:
//!
//! * Small code size
//! * Uses little RAM and CPU
//! * `no_std` feature
//! * All parameters can be compile-time only
//!
//! # Generic vs. dynamic
//!
//! This crate comes in two flavors: generic ([Lzss]) and dynamic ([LzssDyn]).
//!
//! The dynamic one has one compress function and all parameters are passed to
//! it at runtime, making it very adaptive.
//!
//! The generic one has compile-time parameters will produce a function for each
//! different sets of parameters. This function will be more optimized by the
//! compiler than the dynamic one, the downside is that multiple functions are
//! generated when multiple parameter sets are used.
//!
//! (The same applies for decompress and other functions, only used function will
//! be in the generated program.)
//!
//! # Lack of a header
//!
//! This algorithm has by design no header at all. Please be aware that it is not
//! possible to check if the contents is correct, or even the length matches.
//! It is recommended to add a header based on the requirements.
//!
//! # Origin
//! This code is based on the [LZSS encoder-decoder by Haruhiko Okumura, public domain](https://oku.edu.mie-u.ac.jp/~okumura/compression/lzss.c).
//!
//! In order to create an encoder-decoder which is compatible to the program above
//! the following is required: `C = 0x20` in this library and `P = (1+EI+EJ) / 9` in Okumuras program.
//!
//! # Features
//! * `std`         - Enables everything marked with `std`.
//! * `const_panic` - Requires nightly and enables compile-time
//!                   checks of the parameters, see [Lzss].
//!
//! ## Usage
//! With std:
//! ```toml
//! [dependencies]
//! lzss = "0.8"
//! ```
//!
//! With no_std:
//! ```toml
//! [dependencies]
//! lzss = { version = "0.8", default-features = false }
//! ```
//!
//! # Example
//! ```rust
//! # use lzss::{Lzss, SliceReader, SliceWriter};
//! type MyLzss = Lzss<10, 4, 0x20, 1024, 2048>;
//! let input = b"Example Data";
//! let mut output = [0; 30];
//! let result = MyLzss::compress(
//!   SliceReader::new(input),
//!   SliceWriter::new(&mut output),
//! );
//! assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
//! ```

#[cfg(doctest)]
mod test_readme_md {
  macro_rules! external_doc_test {
    ($x:expr) => {
      #[doc = $x]
      extern "C" {}
    };
  }

  external_doc_test!(include_str!("../README.md"));
}

mod bits;
mod dynamic;
mod error;
mod generic;
#[cfg(any(doc, test, feature = "std"))]
mod io_simple;
mod read_write;
mod slice;
#[cfg(any(doc, test, feature = "std"))]
mod vec;
mod void;

pub use crate::dynamic::{LzssDyn, LzssDynError};
pub use crate::error::LzssError;
pub use crate::generic::Lzss;
#[cfg(any(doc, test, feature = "std"))]
pub use crate::io_simple::{IOSimpleReader, IOSimpleWriter};
pub use crate::read_write::{Read, Write};
pub use crate::slice::{SliceReader, SliceWriteError, SliceWriter, SliceWriterExact};
#[cfg(any(doc, test, feature = "std"))]
pub use crate::vec::VecWriter;
pub use crate::void::{
  ResultLzssErrorVoidExt, ResultLzssErrorVoidReadExt, ResultLzssErrorVoidWriteExt,
};
