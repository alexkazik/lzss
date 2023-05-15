# Changelog for lzss

## 0.9.0 -- 2023-02-01

* Check if the buffer fits into usize
* Fix typos
* Fix clippy warnings

## 0.9.0 -- 2023-02-01

* Use safe rust thanks to @cbiffle
* Use const_panic always
* Rename generic's de-/compress to de-/compress_stack
* Improve dynamic decompression a bit
* Generate the generic de-/compression from the dynamic code
* Improve docs and readme
* Fix clippy warnings
* Add benchmark thanks to @cbiffle

## 0.8.2 -- 2021-09-26

* Do not require `std` when building docs #1 thanks to @Cryptjar

## 0.8.1 -- 2021-07-15

* Fixed a potential buffer overflow
* Several small improvements
* Support alloc instead of std where possible 
* Tests

## 0.8.0 -- 2021-06-07

* Initial release
