"""
This file offers the methods to automatically retrieve the graph Cora.

The graph is automatically retrieved from the LINQS repository. 
The Cora dataset consists of 2708 scientific publications classified into
one of seven classes. The citation network consists of 5429 links. Each
publication in the dataset is described by a 0/1-valued word vector indicating
the absence/presence of the corresponding word from the dictionary. The
dictionary consists of 1433 unique words.

References
---------------------
Please cite the following if you use the data:

```bib
@incollection{getoor2005link,
  title={Link-based classification},
  author={Getoor, Lise},
  booktitle={Advanced methods for knowledge discovery from complex data},
  pages={189--207},
  year={2005},
  publisher={Springer}
}

@article{sen2008collective,
  title={Collective classification in network data},
  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
  journal={AI magazine},
  volume={29},
  number={3},
  pages={93--93},
  year={2008}
}
```
"""
from ...ensmallen import Graph  # pylint: disable=import-error
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from typing import Dict, Optional
from .parse_linqs import parse_linqs_incidence_matrix


def Cora(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    load_node_types: bool = True,
    automatically_enable_speedups_for_small_graphs: bool = True,
    sort_temporary_directory: Optional[str] = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the Cora graph.

    The graph is automatically retrieved from the LINQS repository.	The Cora dataset consists of 2708 scientific publications classified into
	one of seven classes. The citation network consists of 5429 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 1433 unique words.

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
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of Cora graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```bib
	@incollection{getoor2005link,
	  title={Link-based classification},
	  author={Getoor, Lise},
	  booktitle={Advanced methods for knowledge discovery from complex data},
	  pages={189--207},
	  year={2005},
	  publisher={Springer}
	}
	
	@article{sen2008collective,
	  title={Collective classification in network data},
	  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
	  journal={AI magazine},
	  volume={29},
	  number={3},
	  pages={93--93},
	  year={2008}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="Cora",
        repository="linqs",
        version=version,
        directed=directed,
        preprocess=preprocess,
        load_nodes=load_nodes,
        load_node_types=load_node_types,
        automatically_enable_speedups_for_small_graphs=automatically_enable_speedups_for_small_graphs,
        sort_temporary_directory=sort_temporary_directory,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
			parse_linqs_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "cora/cora/cora.cites",
		        "content_path": "cora/cora/cora.content",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
