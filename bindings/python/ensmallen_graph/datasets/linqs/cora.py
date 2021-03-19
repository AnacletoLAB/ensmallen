"""
This file offers the methods to automatically retrieve the graph Cora.

The graph is automatically retrieved from the LINQS repository. 

The Cora dataset consists of 2708 scientific publications classified into
one of seven classes. The citation network consists of 5429 links. Each
publication in the dataset is described by a 0/1-valued word vector indicating
the absence/presence of the corresponding word from the dictionary. The
dictionary consists of 1433 unique words.

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-13 10:41:51.046804

The undirected graph Cora has 4141 nodes with 8 different node types: 
the 5 most common are Word (nodes number 1433), Neural_Networks (nodes
number 818), Probabilistic_Methods (nodes number 426), Genetic_Algorithms
(nodes number 418) and Theory (nodes number 351), of which 1 are singletons,
and 54494 unweighted edges with 2 different edge types: Paper2Word and
Paper2Paper, of which none are self-loops. The graph is sparse as it has
a density of 0.00636 and has 2 connected components, where the component
with most nodes has 4140 nodes and the component with the least nodes has
a single node. The graph median node degree is 22, the mean node degree
is 26.32, and the node degree mode is 24. The top 5 most central nodes
are word_1177 (degree 1083), word_1263 (degree 980), word_507 (degree 676),
word_1209 (degree 584) and word_19 (degree 560).


References
---------------------
Please cite the following if you use the data:

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


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.linqs import Cora

    # Then load the graph
    graph = Cora()

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
from .parse_linqs import parse_linqs_incidence_matrix
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def Cora(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/linqs",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the Cora graph.

    The graph is automatically retrieved from the LINQS repository. 

	The Cora dataset consists of 2708 scientific publications classified into
	one of seven classes. The citation network consists of 5429 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 1433 unique words.

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
    Instace of Cora graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-13 10:41:51.046804
	
	The undirected graph Cora has 4141 nodes with 8 different node types: 
	the 5 most common are Word (nodes number 1433), Neural_Networks (nodes
	number 818), Probabilistic_Methods (nodes number 426), Genetic_Algorithms
	(nodes number 418) and Theory (nodes number 351), of which 1 are singletons,
	and 54494 unweighted edges with 2 different edge types: Paper2Word and
	Paper2Paper, of which none are self-loops. The graph is sparse as it has
	a density of 0.00636 and has 2 connected components, where the component
	with most nodes has 4140 nodes and the component with the least nodes has
	a single node. The graph median node degree is 22, the mean node degree
	is 26.32, and the node degree mode is 24. The top 5 most central nodes
	are word_1177 (degree 1083), word_1263 (degree 980), word_507 (degree 676),
	word_1209 (degree 584) and word_19 (degree 560).
	

	References
	---------------------
	Please cite the following if you use the data:
	
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
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.linqs import Cora
	
	    # Then load the graph
	    graph = Cora()
	
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
        graph_name="Cora",
        dataset="linqs",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
			parse_linqs_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "linqs/cora/cora/cora.cites",
		        "content_path": "linqs/cora/cora/cora.content",
		        "node_list_path": "linqs/cora/nodes.tsv",
		        "edge_list_path": "linqs/cora/edges.tsv"
		    }
		]
    )()
