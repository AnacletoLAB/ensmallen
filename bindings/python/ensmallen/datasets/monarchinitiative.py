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
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2025-09-17", **kwargs
) -> Graph:
    """Return Monarch graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "2025-09-17"
        Version to retrieve	
		The available versions are:
			- 202012
			- 202103
			- 2024-01-09
			- 2024-01-12
			- 2024-01-13
			- 2024-01-27
			- 2024-02-13
			- 2024-03-07
			- 2024-03-08
			- 2024-03-13
			- 2024-03-17
			- 2024-03-18
			- 2024-04-18
			- 2024-05-17
			- 2024-05-22
			- 2024-06-07
			- 2024-06-13
			- 2024-07-02
			- 2024-07-03
			- 2024-07-04
			- 2024-07-12
			- 2024-08-12
			- 2024-09-12
			- 2024-10-09
			- 2024-10-16
			- 2024-11-19
			- 2024-11-28
			- 2024-12-13
			- 2024-12-14
			- 2024-12-17
			- 2025-01-14
			- 2025-01-15
			- 2025-02-04
			- 2025-02-08
			- 2025-02-11
			- 2025-02-17
			- 2025-03-10
			- 2025-04-09
			- 2025-04-15
			- 2025-05-23
			- 2025-06-05
			- 2025-06-19
			- 2025-07-05
			- 2025-07-09
			- 2025-07-15
			- 2025-08-01
			- 2025-08-12
			- 2025-08-15
			- 2025-09-12
			- 2025-09-17
    """
    return RetrievedGraph(
        "Monarch", version, "monarchinitiative", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
