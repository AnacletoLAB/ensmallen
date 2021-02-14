"""
This file offers the methods to automatically retrieve the graph PubMedDiabetes.

The graph is automatically retrieved from the LINQS repository. 

The Pubmed Diabetes dataset consists of 19717 scientific publications from
PubMed database pertaining to diabetes classified into one of three classes.
The citation network consists of 44338 links. Each publication in the dataset
is described by a TF/IDF weighted word vector from a dictionary which consists
of 500 unique words.

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-13 10:42:02.115490

The undirected graph PubMedDiabetes has 20217 nodes with 4 different node
types: Diabetes Mellitus Type 1 (nodes number 7875), Diabetes Mellitus
Type 2 (nodes number 7739), Diabetes Mellitus, Experimental (nodes number
4103) and Word (nodes number 500) and 1032358 weighted edges with 2 different
edge types: Paper2Word and Paper2Paper, of which 3 are self-loops. The
graph is sparse as it has a density of 0.00505 and is connected, as it
has a single component. The graph median node degree is 54, the mean node
degree is 102.13, and the node degree mode is 54. The top 5 most central
nodes are studi (degree 11875), 2 (degree 10867), 1 (degree 10588), type
(degree 10572) and result (degree 10262).


References
---------------------
Please cite the following if you use the data:

@inproceedings{namata2012query,
  title={Query-driven active surveying for collective classification},
  author={Namata, Galileo and London, Ben and Getoor, Lise and Huang, Bert and EDU, UMD},
  booktitle={10th International Workshop on Mining and Learning with Graphs},
  volume={8},
  year={2012}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.linqs import PubMedDiabetes

    # Then load the graph
    graph = PubMedDiabetes()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Wether to show a loading bar.
        verbose=True
    )

    # Remember that, if you need, you can enable the memory-time trade-offs:
    train_graph.enable(
        vector_sources=True,
        vector_destinations=True,
        vector_outbounds=True
    )

    # Consider using the methods made available in the Embiggen package
    # to run graph embedding or link prediction tasks.
"""
from typing import Dict
from .parse_linqs import parse_linqs_pubmed_incidence_matrix
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def PubMedDiabetes(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/linqs",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the PubMedDiabetes graph.

    The graph is automatically retrieved from the LINQS repository. 

	The Pubmed Diabetes dataset consists of 19717 scientific publications from
	PubMed database pertaining to diabetes classified into one of three classes.
	The citation network consists of 44338 links. Each publication in the dataset
	is described by a TF/IDF weighted word vector from a dictionary which consists
	of 500 unique words.

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of PubMedDiabetes graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-13 10:42:02.115490
	
	The undirected graph PubMedDiabetes has 20217 nodes with 4 different node
	types: Diabetes Mellitus Type 1 (nodes number 7875), Diabetes Mellitus
	Type 2 (nodes number 7739), Diabetes Mellitus, Experimental (nodes number
	4103) and Word (nodes number 500) and 1032358 weighted edges with 2 different
	edge types: Paper2Word and Paper2Paper, of which 3 are self-loops. The
	graph is sparse as it has a density of 0.00505 and is connected, as it
	has a single component. The graph median node degree is 54, the mean node
	degree is 102.13, and the node degree mode is 54. The top 5 most central
	nodes are studi (degree 11875), 2 (degree 10867), 1 (degree 10588), type
	(degree 10572) and result (degree 10262).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@inproceedings{namata2012query,
	  title={Query-driven active surveying for collective classification},
	  author={Namata, Galileo and London, Ben and Getoor, Lise and Huang, Bert and EDU, UMD},
	  booktitle={10th International Workshop on Mining and Learning with Graphs},
	  volume={8},
	  year={2012}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.linqs import PubMedDiabetes
	
	    # Then load the graph
	    graph = PubMedDiabetes()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Wether to show a loading bar.
	        verbose=True
	    )
	
	    # Remember that, if you need, you can enable the memory-time trade-offs:
	    train_graph.enable(
	        vector_sources=True,
	        vector_destinations=True,
	        vector_outbounds=True
	    )
	
	    # Consider using the methods made available in the Embiggen package
	    # to run graph embedding or link prediction tasks.
    """
    return AutomaticallyRetrievedGraph(
        graph_name="PubMedDiabetes",
        dataset="linqs",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
		    "parse_linqs_pubmed_incidence_matrix"
		]
		callbacks_arguments={
		    "cites_path": "linqs/Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.DIRECTED.cites.tab",
		    "content_path": "linqs/Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.NODE.paper.tab",
		    "node_list_path": "linqs/pubmeddiabetes/nodes.tsv",
		    "edge_list_path": "linqs/pubmeddiabetes/edges.tsv"
		}
    )()
