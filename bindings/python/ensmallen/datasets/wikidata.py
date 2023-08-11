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
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest-truthy", **kwargs
) -> Graph:
    """Return Wikidata graph	

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
    version = "latest-truthy"
        Version to retrieve	
		The available versions are:
			- wikidata-20230626-all-BETA
			- wikidata-20230628-truthy-BETA
			- wikidata-20230630-lexemes-BETA
			- wikidata-20230703-all-BETA
			- wikidata-20230705-truthy-BETA
			- wikidata-20230707-lexemes-BETA
			- wikidata-20230710-all-BETA
			- wikidata-20230712-truthy-BETA
			- wikidata-20230714-lexemes-BETA
			- wikidata-20230717-all-BETA
			- wikidata-20230719-truthy-BETA
			- wikidata-20230721-lexemes-BETA
			- wikidata-20230724-all-BETA
			- wikidata-20230726-truthy-BETA
			- wikidata-20230728-lexemes-BETA
			- wikidata-20230731-all-BETA
			- wikidata-20230802-truthy-BETA
			- wikidata-20230807-all-BETA
			- latest-all
			- latest-lexemes
			- latest-truthy
    """
    return RetrievedGraph(
        "WikiData", version, "wikidata", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
