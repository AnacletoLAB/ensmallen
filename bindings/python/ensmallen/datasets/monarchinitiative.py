"""Module providing graphs available from MonarchInitiative.

References
----------
Please cite:

```bib
@article{mungall2017monarch,
  title={The Monarch Initiative: an integrative data and analytic platform connecting phenotypes to genotypes across species},
  author={Mungall, Christopher J and McMurry, Julie A and K{\"o}hler, Sebastian and Balhoff, James P and Borromeo, Charles and Brush, Matthew and Carbon, Seth and Conlin, Tom and Dunn, Nathan and Engelstad, Mark and others},
  journal={Nucleic acids research},
  volume={45},
  number={D1},
  pages={D712--D722},
  year={2017},
  publisher={Oxford University Press}
}
```
"""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def Monarch(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "202103", **kwargs
) -> Graph:
    """Return Monarch graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
        Load node types
    load_edge_weights = True
        Load edge weights
    auto_enable_tradeoffs = True
        Enable tradeoffs when graph has < 50M edges
    sort_tmp_dir = None
        Path to sorting tmp folder
    verbose = 2
    cache = True
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
        Sys var with cache directory
    version = "202103"
        Version to retrieve	
		The available versions are:
			- 202012
			- 202103
    """
    return AutomaticallyRetrievedGraph(
        "Monarch", version, "monarchinitiative", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

