"""
This file offers the methods to automatically retrieve the graph Nitrospina sp. AB629B18.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 22:55:24.447279

The undirected graph Nitrospina sp. AB629B18 has 2545 nodes and 216470 weighted edges,
of which none are self-loops. The graph is dense as it has a density of 0.06687 and
has 44 connected components, where the component with most nodes has 2440 nodes and
the component with the least nodes has 2 nodes. The graph median node degree is 137,
the mean node degree is 170.11, and the node degree mode is 1. The top 5 most central
nodes are 1131269.AQVV01000005_gene372 (degree 945), 1131269.AQVV01000004_gene551
(degree 900), 1131269.AQVV01000041_gene1655 (degree 872), 1131269.AQVV01000008_gene914
(degree 868) and 1131269.AQVV01000007_gene1025 (degree 843).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import NitrospinaSp.Ab629b18

    # Then load the graph
    graph = NitrospinaSp.Ab629b18()

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
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def NitrospinaSp.Ab629b18(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Nitrospina sp. AB629B18 graph.

    The graph is automatically retrieved from the STRING repository. 

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
    Instace of Nitrospina sp. AB629B18 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 22:55:24.447279
	
	The undirected graph Nitrospina sp. AB629B18 has 2545 nodes and 216470 weighted edges,
	of which none are self-loops. The graph is dense as it has a density of 0.06687 and
	has 44 connected components, where the component with most nodes has 2440 nodes and
	the component with the least nodes has 2 nodes. The graph median node degree is 137,
	the mean node degree is 170.11, and the node degree mode is 1. The top 5 most central
	nodes are 1131269.AQVV01000005_gene372 (degree 945), 1131269.AQVV01000004_gene551
	(degree 900), 1131269.AQVV01000041_gene1655 (degree 872), 1131269.AQVV01000008_gene914
	(degree 868) and 1131269.AQVV01000007_gene1025 (degree 843).
	


	References
	---------------------
	Please cite the following if you use the data:
	
	@article{szklarczyk2019string,
	    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
	    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
	    journal={Nucleic acids research},
	    volume={47},
	    number={D1},
	    pages={D607--D613},
	    year={2019},
	    publisher={Oxford University Press}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.string import NitrospinaSp.Ab629b18
	
	    # Then load the graph
	    graph = NitrospinaSp.Ab629b18()
	
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
        "NitrospinaSp.Ab629b18",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
