# Python Bindings

## How to install this
This project is currently work in progress, and is to be considered for all
intents and porposes an **alpha** version.

To install the **latest (alpha) release**, run the following:

```bash
pip install ensmallen
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
In order to be compatible with as many systems as possible, we suggest to build them on the official Manylinux1 docker.

# Compiling the bindings for Linux to push them to Pipy
In order to make the wheels usable by any linux system we must follow the `manylinux2010` standard. We used to support `manylinux1` but rust no longer support Glibc 2.5 so we had to bump it to the next standard.

To specify for which versions you wish to build the bindings just modify the folliowing line inside of `./ensmallen/setup/DockerFileManyLinux2010`:
```docker
ENV PATH /opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:$PATH
```

This can be done by building the bindings inside a centos5 docker with:
```bash
git clone https://github.com/LucaCappelletti95/ensmallen
cd ensmallen
make build_manylinux2010 python_manylinux2010
```

The result wheels will be in `./ensmallen/bindings/python/target/wheels/`.

A wheel will be created for each version of python, this is configurable in the Dockerfile `./ensmallen/setup/DockerFileManylinux2010` in a ENV var.

A tutorial on how the internals of Python's Cffi can be found [here](https://blog.schuetze.link/2018/07/21/a-dive-into-packaging-native-python-extensions.html)
