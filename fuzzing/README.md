# Fuzzing Guide
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
cd fuzzing/honggfuzz
cargo hfuzz run honggfuzz
```

Install libfuzzer:
```bash
cargo install cargo-fuzz
```

To run libfuzzer
```bash
cd fuzzing/graph_harness
cargo fuzz run from_csv
```

They both share the corpus.