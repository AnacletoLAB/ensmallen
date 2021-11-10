"""
This file offers the methods to automatically retrieve the graph WikiSQ.

The graph is automatically retrieved from the Wikipedia repository. 



"""
from ...ensmallen import Graph  # pylint: disable=import-error
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from typing import Dict, Optional
from ensmallen.edge_list_utils import parse_wikipedia_graph


def WikiSQ(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    load_node_types: bool = True,
    load_edge_weights: bool = True,
    automatically_enable_speedups_for_small_graphs: bool = True,
    sort_temporary_directory: Optional[str] = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the WikiSQ graph.

    The graph is automatically retrieved from the Wikipedia repository.	

    Parameters
    -------------------
    directed: bool = False
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    load_nodes: bool = True
        Whether to load the nodes vocabulary or treat the nodes
        simply as a numeric range.
    load_node_types: bool = True
        Whether to load the node types or skip them entirely.
        This feature is only available when the preprocessing is enabled.
    load_edge_weights: bool = True
        Whether to load the edge weights if available or skip them entirely.
        This feature is only available when the preprocessing is enabled.
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
    Instace of WikiSQ graph.

	
    """
    return AutomaticallyRetrievedGraph(
        graph_name="WikiSQ",
        repository="wikipedia",
        version=version,
        directed=directed,
        preprocess=preprocess,
        load_nodes=load_nodes,
        load_node_types=load_node_types,
        load_edge_weights=load_edge_weights,
        automatically_enable_speedups_for_small_graphs=automatically_enable_speedups_for_small_graphs,
        sort_temporary_directory=sort_temporary_directory,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
			parse_wikipedia_graph
		],
		callbacks_arguments=[
		    {
		        "source": "sqwiki-latest-pages-articles-multistream.xml.bz2",
		        "edge_path": "edge_list.tsv",
		        "node_path": "node_list.tsv",
		        "node_type_path": "node_type_list.tsv",
		        "node_types_separator": "|",
		        "nodes_column": "id",
		        "node_list_node_types_column": "category",
		        "sources_column": "sources",
		        "destinations_column": "destinations"
		    }
		]
    )()
