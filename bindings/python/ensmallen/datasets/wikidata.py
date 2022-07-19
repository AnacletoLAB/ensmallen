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
from .graph_retrieval import RetrievedGraph

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
			- wikidata-20220603-lexemes-BETA
			- wikidata-20220606-all-BETA
			- wikidata-20220608-truthy-BETA
			- wikidata-20220610-lexemes-BETA
			- wikidata-20220613-all-BETA
			- wikidata-20220615-truthy-BETA
			- wikidata-20220617-lexemes-BETA
			- wikidata-20220620-all-BETA
			- wikidata-20220622-truthy-BETA
			- wikidata-20220624-lexemes-BETA
			- wikidata-20220627-all-BETA
			- wikidata-20220629-truthy-BETA
			- wikidata-20220701-lexemes-BETA
			- wikidata-20220704-all-BETA
			- wikidata-20220706-truthy-BETA
			- wikidata-20220708-lexemes-BETA
			- wikidata-20220711-all-BETA
			- wikidata-20220713-truthy-BETA
			- wikidata-20220715-lexemes-BETA
			- latest-all
			- latest-lexemes
			- latest-truthy
    """
    return RetrievedGraph(
        "WikiData", version, "wikidata", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

