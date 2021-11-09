"""
This file offers the methods to automatically retrieve the graph socfb-MU78.

The graph is automatically retrieved from the NetworkRepository repository. 


References
---------------------
Please cite the following if you use the data:

```bib
@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}

@article{traud2012social,
        title={Social structure of {F}acebook networks},
        author={Traud, Amanda L and Mucha, Peter J and Porter, Mason A},
        journal={Phys. A},
        month={Aug},
        number={16},
        pages={4165--4180},
        volume={391},
        year={2012}
}

@article{Traud:2011fs,
        title={Comparing Community Structure to Characteristics in Online Collegiate Social Networks},
        author={Traud, Amanda L and Kelsic, Eric D and Mucha, Peter J and Porter, Mason A},
        journal={SIAM Rev.},
        number={3},
        pages={526--543},
        volume={53},
        year={2011}
}
```
"""
from ...ensmallen import Graph  # pylint: disable=import-error
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from typing import Dict, Optional



def SocfbMu78(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    load_node_types: bool = True,
    load_edge_types: bool = True,
    automatically_enable_speedups_for_small_graphs: bool = True,
    sort_temporary_directory: Optional[str] = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the socfb-MU78 graph.

    The graph is automatically retrieved from the NetworkRepository repository.	

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
    load_node_types: bool = True,
        Whether to load the node types or skip them entirely.
        This may be useful for graphs that we currently do not support,
        where the number of node types is higher than max(u16), roughly 65000.
        This feature is only available when the preprocessing is enabled.
        TODO: add support for graphs with more than 65000 node types.
    load_edge_types: bool = True,
        Whether to load the edge types or skip them entirely.
        This may be useful for graphs that we currently do not support,
        where the number of edge types is higher than max(u16), roughly 65000.
        This feature is only available when the preprocessing is enabled.
        TODO: add support for graphs with more than 65000 edge types.
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
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of socfb-MU78 graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```bib
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	
	@article{traud2012social,
	        title={Social structure of {F}acebook networks},
	        author={Traud, Amanda L and Mucha, Peter J and Porter, Mason A},
	        journal={Phys. A},
	        month={Aug},
	        number={16},
	        pages={4165--4180},
	        volume={391},
	        year={2012}
	}
	
	@article{Traud:2011fs,
	        title={Comparing Community Structure to Characteristics in Online Collegiate Social Networks},
	        author={Traud, Amanda L and Kelsic, Eric D and Mucha, Peter J and Porter, Mason A},
	        journal={SIAM Rev.},
	        number={3},
	        pages={526--543},
	        volume={53},
	        year={2011}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="SocfbMu78",
        repository="networkrepository",
        version=version,
        directed=directed,
        preprocess=preprocess,
        load_nodes=load_nodes,
        load_node_types=load_node_types,
        load_edge_types=load_edge_types,
        automatically_enable_speedups_for_small_graphs=automatically_enable_speedups_for_small_graphs,
        sort_temporary_directory=sort_temporary_directory,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs
    )()
