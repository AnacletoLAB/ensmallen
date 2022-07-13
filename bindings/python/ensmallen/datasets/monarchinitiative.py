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
from .graph_retrieval import RetrievedGraph

def Monarch(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-06-30", **kwargs
) -> Graph:
    """Return Monarch graph	

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
    version = "2022-06-30"
        Version to retrieve	
		The available versions are:
			- 202012
			- 202103
			- 2022-02-10
			- 2022-03-07
			- 2022-03-15
			- 2022-03-30
			- 2022-04-11
			- 2022-04-13
			- 2022-04-20
			- 2022-04-29
			- 2022-05-05
			- 2022-05-07
			- 2022-05-10
			- 2022-05-17
			- 2022-05-23
			- 2022-05-24
			- 2022-05-26
			- 2022-05-27
			- 2022-06-02
			- 2022-06-07
			- 2022-06-08
			- 2022-06-28
			- 2022-06-30
    """
    return RetrievedGraph(
        "Monarch", version, "monarchinitiative", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

