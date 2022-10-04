"""Module providing graphs available from PheKnowLatorKG.

References
----------
Please cite:

```bib
@article{callahan2020framework,
  title={A Framework for Automated Construction of Heterogeneous Large-Scale Biomedical Knowledge Graphs},
  author={Callahan, Tiffany J and Tripodi, Ignacio J and Hunter, Lawrence E and Baumgartner, William A},
  journal={bioRxiv},
  year={2020},
  publisher={Cold Spring Harbor Laboratory}
}
```
"""
from ensmallen import Graph  # pylint: disable=import-error
from .graph_retrieval import RetrievedGraph

def PheKnowLator(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="v3.0.2-2021-10-18.subclass-relationsOnly-owlnets-purified", **kwargs
) -> Graph:
    """Return PheKnowLator graph	

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
    version = "v3.0.2-2021-10-18.subclass-relationsOnly-owlnets-purified"
        Version to retrieve	
		The available versions are:
			- v2.0.0-2020-5-10.instance-inverseRelations-owl
			- v2.0.0-2020-5-10.instance-inverseRelations-owlnets
			- v2.0.0-2020-5-10.instance-relationsOnly-owl
			- v2.0.0-2020-5-10.instance-relationsOnly-owlnets
			- v2.0.0-2020-5-10.subclass-inverseRelations-owl
			- v2.0.0-2020-5-10.subclass-inverseRelations-owlnets
			- v2.0.0-2020-5-10.subclass-relationsOnly-owl
			- v2.0.0-2020-5-10.subclass-relationsOnly-owlnets
			- v2.0.0-2021-1-25.instance-inverseRelations-owl
			- v2.0.0-2021-1-25.instance-inverseRelations-owlnets
			- v2.0.0-2021-1-25.instance-inverseRelations-owlnets-purified
			- v2.0.0-2021-1-25.instance-relationsOnly-owl
			- v2.0.0-2021-1-25.instance-relationsOnly-owlnets
			- v2.0.0-2021-1-25.instance-relationsOnly-owlnets-purified
			- v2.0.0-2021-1-25.subclass-inverseRelations-owl
			- v2.0.0-2021-1-25.subclass-inverseRelations-owlnets
			- v2.0.0-2021-1-25.subclass-inverseRelations-owlnets-purified
			- v2.0.0-2021-1-25.subclass-relationsOnly-owl
			- v2.0.0-2021-1-25.subclass-relationsOnly-owlnets
			- v2.0.0-2021-1-25.subclass-relationsOnly-owlnets-purified
			- v2.0.0-2021-2-11.instance-inverseRelations-owl
			- v2.0.0-2021-2-11.instance-inverseRelations-owlnets
			- v2.0.0-2021-2-11.instance-inverseRelations-owlnets-purified
			- v2.0.0-2021-2-11.instance-relationsOnly-owl
			- v2.0.0-2021-2-11.instance-relationsOnly-owlnets
			- v2.0.0-2021-2-11.instance-relationsOnly-owlnets-purified
			- v2.0.0-2021-2-11.subclass-inverseRelations-owl
			- v2.0.0-2021-2-11.subclass-inverseRelations-owlnets
			- v2.0.0-2021-2-11.subclass-inverseRelations-owlnets-purified
			- v2.0.0-2021-2-11.subclass-relationsOnly-owl
			- v2.0.0-2021-2-11.subclass-relationsOnly-owlnets
			- v2.0.0-2021-2-11.subclass-relationsOnly-owlnets-purified
			- v2.1.0-2021-5-01.instance-inverseRelations-owl
			- v2.1.0-2021-5-01.instance-inverseRelations-owlnets
			- v2.1.0-2021-5-01.instance-inverseRelations-owlnets-purified
			- v2.1.0-2021-5-01.instance-relationsOnly-owl
			- v2.1.0-2021-5-01.instance-relationsOnly-owlnets
			- v2.1.0-2021-5-01.instance-relationsOnly-owlnets-purified
			- v2.1.0-2021-5-01.subclass-inverseRelations-owl
			- v2.1.0-2021-5-01.subclass-inverseRelations-owlnets
			- v2.1.0-2021-5-01.subclass-inverseRelations-owlnets-purified
			- v2.1.0-2021-5-01.subclass-relationsOnly-owl
			- v2.1.0-2021-5-01.subclass-relationsOnly-owlnets
			- v2.1.0-2021-5-01.subclass-relationsOnly-owlnets-purified
			- v2.1.0-2021-6-01.instance-inverseRelations-owl
			- v2.1.0-2021-6-01.instance-inverseRelations-owlnets
			- v2.1.0-2021-6-01.instance-inverseRelations-owlnets-purified
			- v2.1.0-2021-6-01.instance-relationsOnly-owl
			- v2.1.0-2021-6-01.instance-relationsOnly-owlnets
			- v2.1.0-2021-6-01.instance-relationsOnly-owlnets-purified
			- v2.1.0-2021-6-01.subclass-inverseRelations-owl
			- v2.1.0-2021-6-01.subclass-inverseRelations-owlnets
			- v2.1.0-2021-6-01.subclass-inverseRelations-owlnets-purified
			- v2.1.0-2021-6-01.subclass-relationsOnly-owl
			- v2.1.0-2021-6-01.subclass-relationsOnly-owlnets
			- v2.1.0-2021-6-01.subclass-relationsOnly-owlnets-purified
			- v2.1.0-2021-7-06.instance-inverseRelations-owl
			- v2.1.0-2021-7-06.instance-inverseRelations-owlnets
			- v2.1.0-2021-7-06.instance-inverseRelations-owlnets-purified
			- v2.1.0-2021-7-06.instance-relationsOnly-owl
			- v2.1.0-2021-7-06.instance-relationsOnly-owlnets
			- v2.1.0-2021-7-06.instance-relationsOnly-owlnets-purified
			- v2.1.0-2021-7-06.subclass-inverseRelations-owl
			- v2.1.0-2021-7-06.subclass-inverseRelations-owlnets
			- v2.1.0-2021-7-06.subclass-inverseRelations-owlnets-purified
			- v2.1.0-2021-7-06.subclass-relationsOnly-owl
			- v2.1.0-2021-7-06.subclass-relationsOnly-owlnets
			- v2.1.0-2021-7-06.subclass-relationsOnly-owlnets-purified
			- v2.1.0-2021-8-01.instance-inverseRelations-owl
			- v2.1.0-2021-8-01.instance-inverseRelations-owlnets
			- v2.1.0-2021-8-01.instance-inverseRelations-owlnets-purified
			- v2.1.0-2021-8-01.instance-relationsOnly-owl
			- v2.1.0-2021-8-01.instance-relationsOnly-owlnets
			- v2.1.0-2021-8-01.instance-relationsOnly-owlnets-purified
			- v2.1.0-2021-8-01.subclass-inverseRelations-owl
			- v2.1.0-2021-8-01.subclass-inverseRelations-owlnets
			- v2.1.0-2021-8-01.subclass-inverseRelations-owlnets-purified
			- v2.1.0-2021-8-01.subclass-relationsOnly-owl
			- v2.1.0-2021-8-01.subclass-relationsOnly-owlnets
			- v2.1.0-2021-8-01.subclass-relationsOnly-owlnets-purified
			- v2.1.0-2021-9-01.instance-inverseRelations-owl
			- v2.1.0-2021-9-01.instance-inverseRelations-owlnets
			- v2.1.0-2021-9-01.instance-inverseRelations-owlnets-purified
			- v2.1.0-2021-9-01.instance-relationsOnly-owl
			- v2.1.0-2021-9-01.instance-relationsOnly-owlnets
			- v2.1.0-2021-9-01.instance-relationsOnly-owlnets-purified
			- v2.1.0-2021-9-01.subclass-inverseRelations-owl
			- v2.1.0-2021-9-01.subclass-inverseRelations-owlnets
			- v2.1.0-2021-9-01.subclass-inverseRelations-owlnets-purified
			- v2.1.0-2021-9-01.subclass-relationsOnly-owl
			- v2.1.0-2021-9-01.subclass-relationsOnly-owlnets
			- v2.1.0-2021-9-01.subclass-relationsOnly-owlnets-purified
			- v3.0.2-2021-1-01.instance-inverseRelations-owl
			- v3.0.2-2021-1-01.instance-inverseRelations-owlnets
			- v3.0.2-2021-1-01.instance-inverseRelations-owlnets-purified
			- v3.0.2-2021-1-01.instance-relationsOnly-owl
			- v3.0.2-2021-1-01.instance-relationsOnly-owlnets
			- v3.0.2-2021-1-01.instance-relationsOnly-owlnets-purified
			- v3.0.2-2021-1-01.subclass-inverseRelations-owl
			- v3.0.2-2021-1-01.subclass-inverseRelations-owlnets
			- v3.0.2-2021-1-01.subclass-inverseRelations-owlnets-purified
			- v3.0.2-2021-1-01.subclass-relationsOnly-owl
			- v3.0.2-2021-1-01.subclass-relationsOnly-owlnets
			- v3.0.2-2021-1-01.subclass-relationsOnly-owlnets-purified
			- v3.0.2-2021-10-18.instance-inverseRelations-owl
			- v3.0.2-2021-10-18.instance-inverseRelations-owlnets
			- v3.0.2-2021-10-18.instance-inverseRelations-owlnets-purified
			- v3.0.2-2021-10-18.instance-relationsOnly-owl
			- v3.0.2-2021-10-18.instance-relationsOnly-owlnets
			- v3.0.2-2021-10-18.instance-relationsOnly-owlnets-purified
			- v3.0.2-2021-10-18.subclass-inverseRelations-owl
			- v3.0.2-2021-10-18.subclass-inverseRelations-owlnets
			- v3.0.2-2021-10-18.subclass-inverseRelations-owlnets-purified
			- v3.0.2-2021-10-18.subclass-relationsOnly-owl
			- v3.0.2-2021-10-18.subclass-relationsOnly-owlnets
			- v3.0.2-2021-10-18.subclass-relationsOnly-owlnets-purified
    """
    return RetrievedGraph(
        "PheKnowLator", version, "pheknowlatorkg", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
