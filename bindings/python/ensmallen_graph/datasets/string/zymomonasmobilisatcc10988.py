"""
This file offers the methods to automatically retrieve the graph Zymomonas mobilis ATCC10988.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 21:10:41.557003

The undirected graph Zymomonas mobilis ATCC10988 has 1704 nodes and 124722
weighted edges, of which none are self-loops. The graph is dense as it
has a density of 0.08596 and has 2 connected components, where the component
with most nodes has 1702 nodes and the component with the least nodes has
2 nodes. The graph median node degree is 129, the mean node degree is 146.39,
and the node degree mode is 2. The top 5 most central nodes are 555217.Zmob_0789
(degree 774), 555217.Zmob_0076 (degree 612), 555217.Zmob_0029 (degree 610),
555217.Zmob_1003 (degree 536) and 555217.Zmob_0620 (degree 531).


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
    from ensmallen_graph.datasets.string import ZymomonasMobilisAtcc10988

    # Then load the graph
    graph = ZymomonasMobilisAtcc10988()

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


def ZymomonasMobilisAtcc10988(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Zymomonas mobilis ATCC10988 graph.

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
    Instace of Zymomonas mobilis ATCC10988 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 21:10:41.557003
	
	The undirected graph Zymomonas mobilis ATCC10988 has 1704 nodes and 124722
	weighted edges, of which none are self-loops. The graph is dense as it
	has a density of 0.08596 and has 2 connected components, where the component
	with most nodes has 1702 nodes and the component with the least nodes has
	2 nodes. The graph median node degree is 129, the mean node degree is 146.39,
	and the node degree mode is 2. The top 5 most central nodes are 555217.Zmob_0789
	(degree 774), 555217.Zmob_0076 (degree 612), 555217.Zmob_0029 (degree 610),
	555217.Zmob_1003 (degree 536) and 555217.Zmob_0620 (degree 531).
	


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
	    from ensmallen_graph.datasets.string import ZymomonasMobilisAtcc10988
	
	    # Then load the graph
	    graph = ZymomonasMobilisAtcc10988()
	
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
        "ZymomonasMobilisAtcc10988",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
