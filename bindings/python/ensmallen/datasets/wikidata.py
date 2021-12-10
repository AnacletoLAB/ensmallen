"""Module providing graphs available from WikiData.

References
----------
Please cite:

```bib
@article{vrandevcic2014wikidata,
  title={Wikidata: a free collaborative knowledgebase},
  author={Vrande{\v{c}}i{\'c}, Denny and Kr{\"o}tzsch, Markus},
  journal={Communications of the ACM},
  volume={57},
  number={10},
  pages={78--85},
  year={2014},
  publisher={ACM New York, NY, USA}
}
```
"""
from ..ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def WikiData(
    directed = False, preprocess = True, load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest-truthy", **kwargs
) -> Graph:
    """Return Wikidata graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = True
        Preprocess for optimal load time & memory peak
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
    version = "latest-truthy"
        Version to retrieve	
		The available versions are:
			- wikidata-20211025-all-BETA
			- wikidata-20211027-truthy-BETA
			- wikidata-20211029-lexemes-BETA
			- wikidata-20211101-all-BETA
			- wikidata-20211103-truthy-BETA
			- wikidata-20211105-lexemes-BETA
			- wikidata-20211108-all-BETA
			- wikidata-20211110-truthy-BETA
			- wikidata-20211112-lexemes-BETA
			- wikidata-20211115-all-BETA
			- wikidata-20211117-truthy-BETA
			- wikidata-20211119-lexemes-BETA
			- wikidata-20211122-all-BETA
			- wikidata-20211124-truthy-BETA
			- wikidata-20211126-lexemes-BETA
			- wikidata-20211129-all-BETA
			- wikidata-20211201-truthy-BETA
			- wikidata-20211203-lexemes-BETA
			- wikidata-20211206-all-BETA
			- latest-all
			- latest-lexemes
			- latest-truthy
    """
    return AutomaticallyRetrievedGraph(
        "WikiData", version, "wikidata", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

