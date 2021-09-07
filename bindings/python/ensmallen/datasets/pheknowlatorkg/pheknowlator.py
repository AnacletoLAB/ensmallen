"""
This file offers the methods to automatically retrieve the graph PheKnowLator.

The graph is automatically retrieved from the PheKnowLatorKG repository. 


References
---------------------
Please cite the following if you use the data:

```latex
@article{callahan2020framework,
  title={A Framework for Automated Construction of Heterogeneous Large-Scale Biomedical Knowledge Graphs},
  author={Callahan, Tiffany J and Tripodi, Ignacio J and Hunter, Lawrence E and Baumgartner, William A},
  journal={bioRxiv},
  year={2020},
  publisher={Cold Spring Harbor Laboratory}
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def PheKnowLator(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/pheknowlatorkg",
    version: str = "v2.1.0-2021-8-01.subclass-relationsOnly-owlnets-purified",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the PheKnowLator graph.

    The graph is automatically retrieved from the PheKnowLatorKG repository.	

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True,
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True,
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    version: str = "v2.1.0-2021-8-01.subclass-relationsOnly-owlnets-purified",
        The version of the graph to retrieve.		
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
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of PheKnowLator graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```latex
	@article{callahan2020framework,
	  title={A Framework for Automated Construction of Heterogeneous Large-Scale Biomedical Knowledge Graphs},
	  author={Callahan, Tiffany J and Tripodi, Ignacio J and Hunter, Lawrence E and Baumgartner, William A},
	  journal={bioRxiv},
	  year={2020},
	  publisher={Cold Spring Harbor Laboratory}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="PheKnowLator",
        repository="pheknowlatorkg",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
