# EnsmallenGraph
[![Build Status on Travis](https://travis-ci.org/LucaCappelletti94/ensmallen_graph.svg?branch=master)](https://travis-ci.org/github/LucaCappelletti94/ensmallen_graph) [![](https://img.shields.io/badge/rust-nightly-orange)](https://github.com/LucaCappelletti94/ensmallen_graph/tree/master/graph) [![](https://img.shields.io/badge/python-3.5%20%7C%203.6%20%7C%203.7%20%7C%203.8%20%7C%203.9-blue)](https://github.com/LucaCappelletti94/ensmallen_graph/tree/master/bindings/python) ![](https://img.shields.io/badge/platform-linux--64%20%7C%20osx--64%20%7C%20win--64-lightgrey) [![](https://img.shields.io/badge/fuzz-libfuzzer%20%7C%20honggfuzz-blueviolet)](https://github.com/LucaCappelletti94/ensmallen_graph/tree/master/fuzzing) ![](https://img.shields.io/badge/license-MIT-green)

Rust library to run node2vec-like weighted random walks on very big graphs (~50M nodes and ~150M edges).
Based on our benchmarks, our walk is ~600 times faster than Python's [Networkx](https://networkx.github.io/).

## Project coverage
Since some software handling coverages sometime get slightly different results, here's two of them:

[![Coverage Status](https://coveralls.io/repos/github/LucaCappelletti94/ensmallen_graph/badge.svg?branch=master)](https://coveralls.io/github/LucaCappelletti94/ensmallen_graph)
[![codecov](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph/branch/master/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph)

## How to install the Python bindings
This project is currently work in progress, and is to be considered for all
intents and porposes an **alpha** version.

To install the **latest (alpha) release**, run the following:

```bash
pip install ensmallen_graph
```

The pre-compiled wheels needs glibc >= 2.12.

See [this page](https://github.com/LucaCappelletti94/ensmallen_graph/blob/master/bindings/python/README.md) to compile the bindings yourself.