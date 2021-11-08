"""
This file offers the methods to automatically retrieve the graph PubMedDiabetes.

The graph is automatically retrieved from the LINQS repository. 
The Pubmed Diabetes dataset consists of 19717 scientific publications from
PubMed database pertaining to diabetes classified into one of three classes.
The citation network consists of 44338 links. Each publication in the dataset
is described by a TF/IDF weighted word vector from a dictionary which consists
of 500 unique words.

References
---------------------
Please cite the following if you use the data:

```bib
@inproceedings{namata2012query,
  title={Query-driven active surveying for collective classification},
  author={Namata, Galileo and London, Ben and Getoor, Lise and Huang, Bert and EDU, UMD},
  booktitle={10th International Workshop on Mining and Learning with Graphs},
  volume={8},
  year={2012}
}
```
"""
from typing import Dict, Optional
from .parse_linqs import parse_linqs_pubmed_incidence_matrix
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def PubMedDiabetes(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    automatically_enable_speedups_for_small_graphs: bool = True,
    sort_temporary_directory: Optional[str] = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the PubMedDiabetes graph.

    The graph is automatically retrieved from the LINQS repository.	The Pubmed Diabetes dataset consists of 19717 scientific publications from
	PubMed database pertaining to diabetes classified into one of three classes.
	The citation network consists of 44338 links. Each publication in the dataset
	is described by a TF/IDF weighted word vector from a dictionary which consists
	of 500 unique words.

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
    Instace of PubMedDiabetes graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```bib
	@inproceedings{namata2012query,
	  title={Query-driven active surveying for collective classification},
	  author={Namata, Galileo and London, Ben and Getoor, Lise and Huang, Bert and EDU, UMD},
	  booktitle={10th International Workshop on Mining and Learning with Graphs},
	  volume={8},
	  year={2012}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="PubMedDiabetes",
        repository="linqs",
        version=version,
        directed=directed,
        preprocess=preprocess,
        load_nodes=load_nodes,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
			parse_linqs_pubmed_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.DIRECTED.cites.tab",
		        "content_path": "Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.NODE.paper.tab",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
