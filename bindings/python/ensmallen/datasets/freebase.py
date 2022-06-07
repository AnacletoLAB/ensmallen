"""Module providing graphs available from FreeBase.

References
----------
Please cite:

```bib
@inproceedings{bollacker2008freebase,
  title={Freebase: a collaboratively created graph database for structuring human knowledge},
  author={Bollacker, Kurt and Evans, Colin and Paritosh, Praveen and Sturge, Tim and Taylor, Jamie},
  booktitle={Proceedings of the 2008 ACM SIGMOD international conference on Management of data},
  pages={1247--1250},
  year={2008}
}

@misc{freebase:datadumps,
  title = "Freebase Data Dumps"
  author = "Google",
  howpublished = "\\url{https://developers.google.com/freebase/data}",
}
```
"""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def FreeBase(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return FreeBase graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "FreeBase", version, "freebase", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FreeBase2WikiData(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return FreeBase2WikiData graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "FreeBase2WikiData", version, "freebase", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

