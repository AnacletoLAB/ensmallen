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
    version = "latest-truthy"
        Version to retrieve	
		The available versions are:
			- wikidata-20220211-lexemes-BETA
			- wikidata-20220214-all-BETA
			- wikidata-20220216-truthy-BETA
			- wikidata-20220218-lexemes-BETA
			- wikidata-20220221-all-BETA
			- wikidata-20220223-truthy-BETA
			- wikidata-20220225-lexemes-BETA
			- wikidata-20220228-all-BETA
			- wikidata-20220304-lexemes-BETA
			- wikidata-20220307-all-BETA
			- wikidata-20220309-truthy-BETA
			- wikidata-20220311-lexemes-BETA
			- wikidata-20220314-all-BETA
			- wikidata-20220316-truthy-BETA
			- wikidata-20220318-lexemes-BETA
			- wikidata-20220321-all-BETA
			- wikidata-20220323-truthy-BETA
			- wikidata-20220325-lexemes-BETA
			- latest-all
			- latest-lexemes
			- latest-truthy
    """
    return AutomaticallyRetrievedGraph(
        "WikiData", version, "wikidata", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

