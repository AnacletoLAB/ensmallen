# Contributing Guide
Here we will explain the overall structure of the repository and how you can 
contribute to it.

For each big feature (in this case big means that it takes more than 3 commits to finish and debug),
create a new branch, and then submit a pull-request for approval.

The respository is structured as follows:
- `./bindings` This folder contains all the bindings. Currently there are only Python's bindings but in the futures we might expand them to other languages. In general this folder contains a directory for each language.
- `./bindings/python` Home for the Python's bindings for more details see its [`README.md`](https://github.com/AnacletoLAB/ensmallen/blob/master/bindings/python/README.md).
- `./code_analysis` Our static Rust code analyzer, this is used to both automatically generate bindings and enforce extra rules on the code, such as naming conventions and proper documentation of methods.
- `./drawio` Old (outdated) images we use to explain concepts of the library.
- [`./fuzzing`](https://github.com/AnacletoLAB/ensmallen/tree/master/fuzzing) Where all the Rust fuzzing related things are.
- `./fuzzing/graph_harness` The general harnesses we use with the different fuzzers (libFuzz, Honggfuzz). 
- `./fuzzing/graph_harness/fuzz` Contains libFuzzer targets, so that from its parent directory you can run it.
- `./fuzzing/honggfuzz` Folder setted up to fuzz ensmallen with hongufzz, for more details see the
- `./fuzzing/stupid_fuzzer` This is the most basic fuzzer, it takes random values and feed them to the
harness without any sort of coverage. This can also be used to reproduce bugs found by other tests and thus is mostly just to better inspect bugs found by other fuzzers.
- `./fuzzing/unit_tests` Our harnesses catch panics and some signals and dump informations about the errors in a folder with a random name here. When possible we provvide the panic / signal informations, backtrace, original data, graph report, and any other information we can find.
- `./graph` Core of the library, this is where most algorithms and data-structures are.
- `./graph/tags` Procedural macro we use to "tag" methods to instruct our bindings generator or harness generator.
- `./notebooks_and_scripts` This folder contains jupyter notebooks and scripts that we use mainly to update the graph automatic-retrival pipeline.
- `./perf` Little rust executables we use to profile the library and find the bottlenecks.
- `./setup` Dockerfiles and bash scripts that we use to build ensmallen on different systems.

In general we try to have a file `README.md` in every folder that contains informations and details and
a `Makefile` that contains recipies for the most common operations.

# Core
The core of the library is inside the folder `./graph`.
Most of the code are methods fo the `Graph` struct. 
To keep things usable, we try to divide methods into different files and
the self-contained helper functions and structs in sub-modules. 

To test it you can use the command:
```bash
# Quick check that the code compiles
$ cargo check
# Check that there are no code-smells (THIS IS STILL A WORK IN PROGRESS)
$ cargo clippy -- -D clippy::pedantic
# Run the tests (--release is suggested because they take a while)
$ cargo test --release
# Run our custom set of rules and validations based on our static code analyzer
$ (cd ../code_analysis/; cargo run --release --bin check)
```

To gather coverage over the tests, run:
```bash
# Setup flags to gather better coverage (THIS WILL SLOW DOWN SIGNIFICANTLY the tests)
$ export CARGO_INCREMENTAL=0
$ export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
$ export RUSTDOCFLAGS="-Cpanic=abort"
# Remove the target file so that we force re-compilation
$ rm -rfd target;
# Update the libraries (mostly the ones from git need this)
$ cargo update;
# Run the now instrumented test (due to -Zprofile)
$ cargo test;
# Create a folder where we will save the coverage
$ mkdir ./target/debug/coverage;
# Convert the coverage gathered during the test to a human-readable satic website.
$ grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/;
```
The resulting coverage can be found at `./target/debug/coverage/src/index.html`.

Most of the test suite is contained in `./src/test_utilities.rs`, this file contains
tests that check the general consistency of the graph. Thesere are in the main sources
because we use them both in tests and during fuzzing.

# Fuzzing the Core
To help check if there are bugs, we heavely fuzz the code. We have two main harnesses.
An harness is a piece of code that takes a random vector of bytes and use it as 
input for the code you want to test. The simple harness (found at `./fuzzing/graph_harness/src/from_strings.rs`)
create a random graph using the `from_strings` constructor and then run the default test-suite on it.

More informations can be found in the `README.md` inside the `./fuzzing` folder, but 
to start fuzzing you can just run:
```bash
$ cd fuzzing
$ make hfuzz_from_strings
```
If it finds bugs, a report about them, useful for debugging, will be generated inside the `./fuzzing/unit_tests` folder.

# Code analysis
Our hand-written recursive descent rust parser can be found in `./code_analysis/rust_parser`.
We built several utilities: 
- `bindgen` It analyze all the methods and functions in the core library and automatically create the 
python bindings creating the file `auto_generated_bindings.rs`. Also this create the data we use for
the TF-IDF raccomander system we have for the `__get_attr__` magic method, in particular
it pre-process and store, for each struct, a list of all the methods, and the pre-computed TFIDF weights
of each term.
- `metatest` It analyze all the methods of the `Graph` struct and create a fuzzing harness that 
create a random graph and calls 10 random methods with random arguments.
- `check` It's our static analysis code quality checks. It check that every public
method has a documentation formatted in our standard way, that, if the files has a 
naming convention, it's respected, and other checks.

# Python Bindings
Most of the python bindings are automatically generated throught our `code_analysis` tools.
So it's a good practice to re-generate the bindings each time you compile them:
```bash
$ cd ./code_analysis
$ cargo run --release --bin bindgen
```

The bindings use the [`PyO3`](https://github.com/PyO3/pyo3) library and [`maturin`](https://github.com/PyO3/maturin) for the compilation and packaging steps.

To quiclky build and install the bindings for you current python environment use:
```bash
$ cd ./bindings/python
$ maturin develop --release --strip
```

To compile them for publishing use:
```bash
$ cd ./bindings/python
$ RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000" maturin build --release --strip
```
The compiled bindins will be in `./bindings/python/target/wheels/`.

# Fuzzing Python Bindings
Using the new shiny Google's toy, [Atheris](https://github.com/google/atheris) we can fuzz the bindings.
This way pairing the Rust and the Python fuzzers we can test all the code in the library.

A little example of an harness is:
```python
#!/usr/bin/python
import sys
import atheris
import ensmallen_graph    

def fuzz_me(input_bytes: bytes):
    fdp = atheris.FuzzedDataProvider(input_bytes)
    try:
        ensmallen_graph.preprocessing.okapi_bm25_tfidf_int(
            [   
                [
                    fdp.ConsumeUInt(4)
                    for _ in range(fdp.ConsumeUInt(1))
                ]
                for _ in range(fdp.ConsumeUInt(1))
            ],
            fdp.ConsumeUInt(4),
            fdp.ConsumeUInt(4),
            False,
        )
    except ValueError:
        pass

atheris.instrument_all()
atheris.Setup(sys.argv, fuzz_me)
atheris.Fuzz()
```

To have coverage over the rust code we must instrumen it, so we need to compile the bindings as follows:
```bash
$ cd ./bindings/python
$ maturin develop --release --rustc-extra-args='-Ctarget-cpu=native -Zinstrument-coverage -Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=4  -Cllvm-args=-sanitizer-coverage-trace-compares  -Cllvm-args=-sanitizer-coverage-inline-8bit-counters  -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-stack-depth --verbose -Zsanitizer=address'
```

Finally you can run the fuzzer as:
```bash
LD_PRELOAD="$(python -c "import atheris; print(atheris.path())")/asan_with_fuzzer.so" python my_atheris_script.py
```

This does not yet supports panics / signals catching so you must implmement it yourself in the python harness.

# Perf
To profile our code just create a new binary target (e.g `my_bin`) in the folder,
and create a program that loads a graph and call the method you want to profile.
It's better if the program takes from 10 seconds to 1 minute.

Then you should compile the code:
```bash
cargo build --release
```

And run the profiling:
```bash
$ perf record --call-graph=dwarf ./target/release/my_bin
```
This will create a file called `perf.data` which can be analyzed using [`hotspot`](https://github.com/KDAB/hotspot) or directly `perf report` and `perf annotate`.
For more info, see the `perf` documentation.
