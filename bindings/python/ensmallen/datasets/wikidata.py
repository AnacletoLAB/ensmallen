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
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def WikiData(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest-truthy", **kwargs
) -> Graph:
    """Return Wikidata graph	

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
    version = "latest-truthy"
        Version to retrieve	
		The available versions are:
			- wikidata-20220404-all-BETA
			- wikidata-20220406-truthy-BETA
			- wikidata-20220408-lexemes-BETA
			- wikidata-20220411-all-BETA
			- wikidata-20220413-truthy-BETA
			- wikidata-20220415-lexemes-BETA
			- wikidata-20220418-all-BETA
			- wikidata-20220420-truthy-BETA
			- wikidata-20220422-lexemes-BETA
			- wikidata-20220425-all-BETA
			- wikidata-20220427-truthy-BETA
			- wikidata-20220429-lexemes-BETA
			- wikidata-20220502-all-BETA
			- wikidata-20220504-truthy-BETA
			- wikidata-20220506-lexemes-BETA
			- wikidata-20220509-all-BETA
			- wikidata-20220511-truthy-BETA
			- wikidata-20220513-lexemes-BETA
			- latest-all
			- latest-lexemes
			- latest-truthy
    """
    return AutomaticallyRetrievedGraph(
        "WikiData", version, "wikidata", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

