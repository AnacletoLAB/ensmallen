# EnsmallenGraph
[![Build Status on Travis](https://travis-ci.org/LucaCappelletti94/ensmallen_graph.svg?branch=master)](https://travis-ci.org/github/LucaCappelletti94/ensmallen_graph)

Rust library to run node2vec-like weighted random walks on very big graphs.

## Project coverage
Since some software handling coverages sometime get slightly different results, here's two of them:

[![Coverage Status](https://coveralls.io/repos/github/LucaCappelletti94/ensmallen_graph/badge.svg?branch=master)](https://coveralls.io/github/LucaCappelletti94/ensmallen_graph)
[![codecov](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph/branch/master/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/ensmallen_graph)

## How to install this
This project is currently work in progress, and is to be considered for all
intents and porposes an alpha version.

To install the latest (alpha) release, run the following:

```bash
pip install ensmallen_graph
```

## Example code:

Graph should be in KGX TSV format described here:
https://github.com/NCATS-Tangerine/kgx/blob/master/data-preparation.md

```
import ensmallen_graph

graph = ensmallen_graph.EnsmallenGraph(
    edge_path=f"nodes.tsv",
    node_path=f"edges.tsv",
    sources_column="subject",
    destinations_column="object",
    edge_types_column="edge_label",
    default_node_type="biolink:NamedThing",
    default_edge_type="biolink:interacts_with",
    nodes_column='id',
    node_types_column='category',
    directed=False,
)
```