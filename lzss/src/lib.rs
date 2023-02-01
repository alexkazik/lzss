#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::inline_always)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::manual_assert)]

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
//! This crate comes in two flavors: generic ([`Lzss`](crate::Lzss)) and dynamic ([`LzssDyn`](crate::LzssDyn)).
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
//! * `alloc`       - Allows de-/compression with buffer on the heap and the [`VecWriter`](crate::VecWriter).
//! * `safe`        - Only use safe code (see Safety below).
//! * `std`         - Enables `alloc` and additional [`IOSimpleReader`](crate::IOSimpleReader), [`IOSimpleWriter`](crate::IOSimpleWriter),
//!                   and the [`Error`](::std::error::Error) instance for [`LzssError`](crate::LzssError) and [`LzssDynError`](crate::LzssDynError).
//!
//! `std` and `safe` are enabled by default.
//!
//! ## Usage
//! With defaults (`std` and `safe`):
//! ```toml
//! [dependencies]
//! lzss = "0.9"
//! ```
//!
//! With `no_std` (and without `safe`):
//! ```toml
//! [dependencies]
//! lzss = { version = "0.9", default-features = false }
//! ```
//!
//! # Example
//! ```rust
//! # use lzss::{Lzss, SliceReader, SliceWriter};
//! type MyLzss = Lzss<10, 4, 0x20>;
//! let input = b"Example Data";
//! let mut output = [0; 30];
//! let result = MyLzss::compress(
//!   SliceReader::new(input),
//!   SliceWriter::new(&mut output),
//! );
//! assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
//! ```
//!
//! # Safety
//!
//! With the `safe` feature the code is not using any unsafe code (`forbid(unsafe_code)`), but at
//! the cost of performance and size - though on modern systems that is not to mention.
//!
//! But on smaller systems (like microcontrollers, where `no_std` is needed) it may be noticeable.
//! Which is the reason wht it can be switched on/off.

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

pub use crate::dynamic::{LzssDyn, LzssDynError};
pub use crate::error::LzssError;
pub use crate::generic::Lzss;
#[cfg(any(test, feature = "std"))]
pub use crate::io_simple::{IOSimpleReader, IOSimpleWriter};
pub use crate::read_write::{Read, Write};
pub use crate::slice::{SliceReader, SliceWriteError, SliceWriter, SliceWriterExact};
#[cfg(any(test, feature = "alloc"))]
pub use crate::vec::VecWriter;
pub use crate::void::{
    ResultLzssErrorVoidExt, ResultLzssErrorVoidReadExt, ResultLzssErrorVoidWriteExt,
};

mod bits;
mod dynamic;
mod error;
mod generic;
#[cfg(any(test, feature = "std"))]
mod io_simple;
mod macros;
mod read_write;
#[cfg_attr(feature = "safe", path = "slice_safe.rs")]
mod slice;
#[cfg(any(test, feature = "alloc"))]
mod vec;
mod void;
