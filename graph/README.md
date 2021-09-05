# EnsmallenGraph
[![Build Status on Travis](https://travis-ci.org/LucaCappelletti94/ensmallen.svg?branch=master)](https://travis-ci.org/github/LucaCappelletti94/ensmallen)

Rust library to run node2vec-like weighted random walks on very big graphs.

## Project coverage
Since some software handling coverages sometime get slightly different results, here's two of them:

[![Coverage Status](https://coveralls.io/repos/github/LucaCappelletti94/ensmallen/badge.svg?branch=master)](https://coveralls.io/github/LucaCappelletti94/ensmallen)
[![codecov](https://codecov.io/gh/LucaCappelletti94/ensmallen/branch/master/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/ensmallen)

## How to install this
This project is currently work in progress, and is to be considered for all
intents and porposes an **alpha** version.

To install the **latest (alpha) release**, run the following:

```bash
pip install ensmallen
```

## Fuzzing Guide
The module expose an harness that supports the `arbitrary` crate so it's really easy to fuzz.

More over, 2 fuzzers are already setted up, `honggfuzz` and `libFuzzer`.

Install honggfuzz:
```bash
cargo install honggfuzz
```
On ubuntu it also needs the following packages (even though they are usually already installed):
```bash
sudo apt install build-essential binutils-dev libunwind-dev libblocksruntime-dev liblzma-dev
```

To run honggfuzz:
```bash
cd graph/honggfuzz
cargo hfuzz run honggfuzz
```

Install libfuzzer:
```bash
cargo install cargo-fuzz
```

To run libfuzzer
```bash
cd graph
cargo fuzz run from_csv
```

They both share the corpus.