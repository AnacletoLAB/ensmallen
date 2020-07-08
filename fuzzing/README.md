# Fuzzing Guide
The module expose an harness that supports the `arbitrary` crate so it's really easy to fuzz.

More over, 2 fuzzers are already setted up, `honggfuzz` and `libFuzzer`.
They both share the corpus.

# libfuzzer
[Libfuzzer](https://github.com/google/fuzzing/blob/master/tutorial/libFuzzerTutorial.md) it's the LLVMs fuzzer (https://llvm.org/docs/LibFuzzer.html).
LibFuzzer is in-process, coverage-guided, evolutionary fuzzing engine.

LibFuzzer is linked with the library under test, and feeds fuzzed inputs to the library via a specific fuzzing entrypoint (aka “target function”); the fuzzer then tracks which areas of the code are reached, and generates mutations on the corpus of input data in order to maximize the code coverage. The code coverage information for libFuzzer is provided by LLVM’s SanitizerCoverage instrumentation.

### Install libfuzzer:
```bash
cargo install cargo-fuzz
```

### Run libfuzzer
```bash
cd fuzzing/graph_harness
cargo fuzz run from_csv
```

# honggfuzz

### Install honggfuzz:
[Honggfuzz](https://honggfuzz.dev/) it's mantained by google (https://github.com/google/honggfuzz).

```bash
cargo install honggfuzz
```
On ubuntu it also needs the following packages (even though they are usually already installed):
```bash
sudo apt install build-essential binutils-dev libunwind-dev libblocksruntime-dev liblzma-dev
```

### Compile and install honggfuzz:
To build and install honggfuzz without cargo just run:
```bash
git clone https://github.com/google/honggfuzz
(cd honggfuzz; make -j$(nproc); sudo make -j$(nproc) install)
```

### Run honggfuzz:
```bash
cd fuzzing/honggfuzz
cargo hfuzz run honggfuzz
```

Alternatively you can run it without cargo as:
```bash
cd fuzzing/honggfuzz
cargo hfuzz build
honggfuzz -P -i ./hfuzz_workspace/honggfuzz/input -- ./hfuzz_target/honggfuzz
```

Useful parameters are `-n` which is the number of threads and `-t` which is the timeout.
An example of the fuzzer using 12 threads and having a timeout of 30 seconds is:
```bash
cd fuzzing/honggfuzz
cargo hfuzz build
honggfuzz -n 12 -t 30 -P -i ./hfuzz_workspace/honggfuzz/input -- ./hfuzz_target/honggfuzz
```

We can specify which measure to maximize:
- `--linux_perf_instr`  The number of instruction executed.
- `--linux_perf_branch` The number of brench taken.
- `--linux_perf_bts_edge` The number of unique edges taken, counted with Intel BTS.
- `--linux_perf_ipt_block`  Use Intel Processor Trace to count unique blocks.

So if we want to maximze the number of instructions covered we could run:
```bash
cd fuzzing/honggfuzz
cargo hfuzz build
honggfuzz --linux_perf_instr -P -i ./hfuzz_workspace/honggfuzz/input -- ./hfuzz_target/honggfuzz
```

Therefore the tipical command looks like:
```bash
honggfuzz --linux_perf_branch -n $(nproc) -t 30 -P -i ./hfuzz_workspace/honggfuzz/input -- ./hfuzz_target/honggfuzz
```

In the same way we can pass additional arguments to the cargo version with:
```bash
HFUZZ_RUN_ARGS="-n $(nproc) -t 30" cargo hfuzz run honggfuzz
```