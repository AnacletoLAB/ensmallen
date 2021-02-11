"""
This file offers the methods to automatically retrieve the graph Rhizobium sp. OK494.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:15:47.866899

The undirected graph Rhizobium sp. OK494 has 6540 nodes and 1047065 weighted
edges, of which none are self-loops. The graph is dense as it has a density
of 0.04897 and has 15 connected components, where the component with most
nodes has 6509 nodes and the component with the least nodes has 2 nodes.
The graph median node degree is 257, the mean node degree is 320.20, and
the node degree mode is 1. The top 5 most central nodes are 1500306.JQLA01000040_gene2104
(degree 2611), 1500306.JQLA01000008_gene4207 (degree 2512), 1500306.JQLA01000042_gene4977
(degree 2369), 1500306.JQLA01000004_gene5466 (degree 2264) and 1500306.JQLA01000006_gene578
(degree 2152).


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
    from ensmallen_graph.datasets.string import RhizobiumSp.Ok494

    # Then load the graph
    graph = RhizobiumSp.Ok494()

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


def RhizobiumSp.Ok494(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Rhizobium sp. OK494 graph.

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
    Instace of Rhizobium sp. OK494 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:15:47.866899
	
	The undirected graph Rhizobium sp. OK494 has 6540 nodes and 1047065 weighted
	edges, of which none are self-loops. The graph is dense as it has a density
	of 0.04897 and has 15 connected components, where the component with most
	nodes has 6509 nodes and the component with the least nodes has 2 nodes.
	The graph median node degree is 257, the mean node degree is 320.20, and
	the node degree mode is 1. The top 5 most central nodes are 1500306.JQLA01000040_gene2104
	(degree 2611), 1500306.JQLA01000008_gene4207 (degree 2512), 1500306.JQLA01000042_gene4977
	(degree 2369), 1500306.JQLA01000004_gene5466 (degree 2264) and 1500306.JQLA01000006_gene578
	(degree 2152).
	

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
	    from ensmallen_graph.datasets.string import RhizobiumSp.Ok494
	
	    # Then load the graph
	    graph = RhizobiumSp.Ok494()
	
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
        "RhizobiumSp.Ok494",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
