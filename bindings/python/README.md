# Python Bindings

## How to install this
This project is currently work in progress, and is to be considered for all
intents and porposes an **alpha** version.

To install the **latest (alpha) release**, run the following:

```bash
pip install ensmallen_graph
```

## Compile the bindings
The project uses PyO3 and maturin.
You need nightly rust in order to compile PyO3.
This could be done by running:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh --default-toolchain nightly --profile default -y
```

Then we need to install maturin with:
```
cargo install maturin
```

And finally we can create the Wheels with:
```
maturin build --release
```

The wheels will be located in `./target/wheels/`.

### Compile the bindings for multiple Python versions

By default maturin will build for the installed python version.

To install other Python versions just run:
```bash
wget https://www.python.org/ftp/python/${PYTHON_VERSION}/Python-${PYTHON_VERSION}.tar.xz
tar -xf Python-${PYTHON_VERSION}.tar.xz
(cd Python-${PYTHON_VERSION}; ./configure --enable-optimizations; make -j$(nproc); make -j$(nproc) install)
```
Where `${PYTHON_VERSION}` is the choosen version, such as `3.8.3` or `3.6.10`.

## Solving the GLIBC__2.29__ not found error
This means that the linux package were built on a system with a really new Libc version.
This can be solved by compiling your own bindings.
In order to be compatible with as many systems as possible, we suggest to build them on 
Ubuntu 16.04 .