"""
This file offers the methods to automatically retrieve the graph CiteSeer.

The graph is automatically retrieved from the LINQS repository. 

The CiteSeer dataset consists of 3312 scientific publications classified
into one of six classes. The citation network consists of 4732 links. Each
publication in the dataset is described by a 0/1-valued word vector indicating
the absence/presence of the corresponding word from the dictionary. The
dictionary consists of 3703 unique words.

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-13 09:45:27.015162

The undirected graph CiteSeer has 7030 nodes with 8 different node types:
 the 5 most common are Word (nodes number 3703), DB (nodes number 701),
IR (nodes number 668), Agents (nodes number 596) and ML (nodes number 590)
and 109841 unweighted edges with 2 different edge types: Paper2Word and
Paper2Paper, of which 124 are self-loops. The graph is sparse as it has
a density of 0.00444 and is connected, as it has a single component. The
graph median node degree is 28, the mean node degree is 31.23, and the
node degree mode is 5. The top 5 most central nodes are word_2568 (degree
704), word_65 (degree 670), word_729 (degree 651), word_601 (degree 627)
and word_2615 (degree 607).


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
    from ensmallen_graph.datasets.linqs import CiteSeer

    # Then load the graph
    graph = CiteSeer()

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
from .parse_linqs import parse_linqs_incidence_matrix
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def CiteSeer(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/linqs"
) -> EnsmallenGraph:
    """Return new instance of the CiteSeer graph.

    The graph is automatically retrieved from the LINQS repository. 

	The CiteSeer dataset consists of 3312 scientific publications classified
	into one of six classes. The citation network consists of 4732 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 3703 unique words.

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

    Returns
    -----------------------
    Instace of CiteSeer graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-13 09:45:27.015162
	
	The undirected graph CiteSeer has 7030 nodes with 8 different node types:
	 the 5 most common are Word (nodes number 3703), DB (nodes number 701),
	IR (nodes number 668), Agents (nodes number 596) and ML (nodes number 590)
	and 109841 unweighted edges with 2 different edge types: Paper2Word and
	Paper2Paper, of which 124 are self-loops. The graph is sparse as it has
	a density of 0.00444 and is connected, as it has a single component. The
	graph median node degree is 28, the mean node degree is 31.23, and the
	node degree mode is 5. The top 5 most central nodes are word_2568 (degree
	704), word_65 (degree 670), word_729 (degree 651), word_601 (degree 627)
	and word_2615 (degree 607).
	

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
	    from ensmallen_graph.datasets.linqs import CiteSeer
	
	    # Then load the graph
	    graph = CiteSeer()
	
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
        "CiteSeer",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[parse_linqs_incidence_matrix]
        dataset="linqs"
    )()
