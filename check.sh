#!/bin/bash

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (safe+std+alloc, std+alloc, alloc, nothing) (std implies alloc, tests require alloc)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features std --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features alloc --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

# clippy (safe+std+alloc, std+alloc, alloc, nothing) (std implies alloc, tests require alloc)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features --features std --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features --features alloc --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
( cd lzss && cargo rdme --force ) || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p lzss || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

# tests (safe+std+alloc, alloc) (std implies alloc, tests require alloc)
cargo $TOOLCHAIN test --release --all-features -- --include-ignored || exit 1
cargo $TOOLCHAIN test --release --no-default-features --features alloc -- --include-ignored || exit 1
