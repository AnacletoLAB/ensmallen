"""
This file offers the methods to automatically retrieve the wikipedia graph WikiSourceAS.

The graph is automatically retrieved from the Wikipedia repository. 



"""
from ...ensmallen import Graph  # pylint: disable=import-error
from ..wikipedia_automatic_graph_retrieval import WikipediaAutomaticallyRetrievedGraph
from typing import Dict, Optional


def WikiSourceAS(
    directed: bool = False,
    load_nodes: bool = True,
    load_node_types: bool = True,
    compute_node_description: bool = False,
    automatically_enable_speedups_for_small_graphs: bool = True,
    sort_temporary_directory: Optional[str] = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the WikiSourceAS graph.

    The graph is automatically retrieved from the Wikipedia repository.	

    Parameters
    -------------------
    directed: bool = False
        Wether to load the graph as directed or undirected.
        By default false.
    load_nodes: bool = True
        Whether to load the nodes vocabulary or treat the nodes
        simply as a numeric range.
    load_node_types: bool = True
        Whether to load the node types or skip them entirely.
        This feature is only available when the preprocessing is enabled.
    compute_node_description: bool = False
        Whether to compute the node descriptions.
        Note that this will significantly increase the side of the node lists!
    automatically_enable_speedups_for_small_graphs: bool = True
        Whether to enable the Ensmallen time-memory tradeoffs in small graphs
        automatically. By default True, that is, if a graph has less than
        50 million edges. In such use cases the memory expenditure is minimal.
    sort_temporary_directory: Optional[str] = None
        Which folder to use to store the temporary files needed to sort in 
        parallel the edge list when building the optimal preprocessed file.
        This defaults to the same folder of the edge list when no value is 
        provided.
    verbose: int = 2
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: Optional[str] = None
        Where to store the downloaded graphs.
        If no path is provided, first we check the system variable
        provided below is set, otherwise we use the directory `graphs`.
    cache_path_system_variable: str = "GRAPH_CACHE_DIR"
        The system variable with the default graph cache directory.
    version: str = "latest"
        The version of the graph to retrieve.	
		The available versions are:
			- 20210620
			- 20210701
			- 20210720
			- 20210801
			- 20210820
			- 20210901
			- 20210920
			- 20211001
			- 20211020
			- 20211101
			- latest
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of WikiSourceAS graph.

	
    """
    return WikipediaAutomaticallyRetrievedGraph(
        graph_name="WikiSourceAS",
        repository="wikipedia",
        version=version,
        directed=directed,
        load_nodes=load_nodes,
        load_node_types=load_node_types,
        compute_node_description=compute_node_description,
        automatically_enable_speedups_for_small_graphs=automatically_enable_speedups_for_small_graphs,
        sort_temporary_directory=sort_temporary_directory,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs
    )()
