#!/bin/bash
# Get the targets
#TARGETS=$(rustup target list | grep x86_6)
#echo $TARGETS
TARGETS=(
    x86_64-apple-darwin
    x86_64-pc-windows-msvc
    x86_64-unknown-freebsd
    x86_64-unknown-linux-gnu
    x86_64-unknown-netbsd
)
# Setup nightly as default so that we can use Pyo3
rustup default nightly
# Install all the toolchains
for target in $TARGETS; do
    rustup toolchain install "nightly-$target"
done
# Build for each target
for target in $TARGETS; do
    echo $target
    maturin build --release --target $target
done