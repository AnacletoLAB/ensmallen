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
    cache_sys_var="GRAPH_CACHE_DIR", version="2023-10-17", **kwargs
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
    version = "2023-10-17"
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
			- 2022-04-18
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
			- 2022-07-27
			- 2022-09-27
			- 2022-10-05
			- 2022-10-07
			- 2022-10-11
			- 2022-10-14
			- 2022-10-16
			- 2022-11-16
			- 2022-12-06
			- 2022-12-11
			- 2023-01-06
			- 2023-01-08
			- 2023-01-15
			- 2023-01-22
			- 2023-01-31
			- 2023-02-05
			- 2023-02-12
			- 2023-02-19
			- 2023-02-28
			- 2023-03-05
			- 2023-03-08
			- 2023-03-09
			- 2023-03-10
			- 2023-03-12
			- 2023-03-14
			- 2023-03-16
			- 2023-03-17
			- 2023-03-22
			- 2023-04-02
			- 2023-04-15
			- 2023-04-16
			- 2023-04-25
			- 2023-04-27
			- 2023-05-03
			- 2023-05-14
			- 2023-05-21
			- 2023-05-25
			- 2023-05-31
			- 2023-06-01
			- 2023-06-04
			- 2023-06-08
			- 2023-06-11
			- 2023-06-18
			- 2023-06-25
			- 2023-06-27
			- 2023-07-08
			- 2023-07-09
			- 2023-07-13
			- 2023-08-24
			- 2023-09-15
			- 2023-09-28
			- 2023-10-17
    """
    return RetrievedGraph(
        "Monarch", version, "monarchinitiative", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
