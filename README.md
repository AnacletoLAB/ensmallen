# EnsmallenGraph
[![Build Status on Travis](https://travis-ci.org/LucaCappelletti94/ensmallen_graph.svg?branch=master)](https://travis-ci.org/github/LucaCappelletti94/ensmallen_graph)
Rust library to run weighted random walks on very big graphs.

## Project coverage
Since some software handling coverages sometime get slightly different results, here's two of them:

[![Coverage Status](https://coveralls.io/repos/github/LucaCappelletti94/ensmallen_graph/badge.svg?branch=master)](https://coveralls.io/github/LucaCappelletti94/ensmallen_graph)
[![codecov](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph/branch/master/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph)

## Build the python library
In order to build the optimized version of the bindings we need [maturin](https://github.com/PyO3/maturin).
```bash
maturin build --release
```
The `.whl` file will be inside of `./target/wheels`.
This file can be installed with pip as:
```bash
pip install --upgrade --user ./target/wheels/*.whl
```

## Setup for Google cloud
Run the following to setup a google cloud machine.
In the future, this should be moved into a docker.

```bash
sudo apt install wget git-all -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

## Build inside of Docker
Build the Dockerfile 
```bash
sudo docker build -t ensmallen-env .
sudo docker run -it -v "${PWD}:/build" ensmallen-env
```
