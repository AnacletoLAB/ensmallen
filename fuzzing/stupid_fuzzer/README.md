# Stupid Fuzzer
This is an implementation of the simplest bit-flipper fuzzer.
This has no coverage or feedback mechanism.
The goal of this fuzzer is to be used on machines where we cannot install libraries
or compiler easily honggfuzz and libfuzz.

### Usage
```bash
cargo run --release --bin fuzz
```

# Fuzzer Debugger
A small binary that takes one of the inputs generated from the other fuzers,
prints it and run the from_vec test on it multiple times.

```bash
cargo run --release --bin debug ../corpus/from_vec/b96dac44a8cfc85e309288ea2909a7ff.00000010.honggfuzz.cov 10
```

# Sig abort test:

Usage example:
```bash
cargo run --release --bin sigabrttest ../corpus/from_vec/b96dac44a8cfc85e309288ea2909a7ff.00000010.honggfuzz.cov 1000000000
```