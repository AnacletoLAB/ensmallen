"""
This file offers the methods to automatically retrieve the graph KGMicrobe.

The graph is automatically retrieved from the KGHub repository. 



"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def KGMicrobe(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/kghub",
    version: str = "current",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the KGMicrobe graph.

    The graph is automatically retrieved from the KGHub repository.	

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
    version: str = "current",
        The version of the graph to retrieve.		
	The available versions are:
			- 20210422
			- 20210517
			- 20210608
			- 20210615
			- 20210617
			- 20210622
			- 20210715
			- current
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of KGMicrobe graph.

	
    """
    return AutomaticallyRetrievedGraph(
        graph_name="KGMicrobe",
        dataset="kghub",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
