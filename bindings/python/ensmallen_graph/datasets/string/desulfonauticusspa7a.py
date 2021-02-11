"""
This file offers the methods to automatically retrieve the graph Desulfonauticus sp. A7A.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:11:10.577004

The undirected graph Desulfonauticus sp. A7A has 1912 nodes and 135353
weighted edges, of which none are self-loops. The graph is dense as it
has a density of 0.07409 and has 5 connected components, where the component
with most nodes has 1902 nodes and the component with the least nodes has
2 nodes. The graph median node degree is 116, the mean node degree is 141.58,
and the node degree mode is 1. The top 5 most central nodes are 1379281.AVAG01000011_gene1246
(degree 879), 1379281.AVAG01000023_gene1901 (degree 706), 1379281.AVAG01000016_gene954
(degree 689), 1379281.AVAG01000052_gene912 (degree 675) and 1379281.AVAG01000031_gene106
(degree 673).


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
    from ensmallen_graph.datasets.string import DesulfonauticusSpA7a

    # Then load the graph
    graph = DesulfonauticusSpA7a()

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


def DesulfonauticusSpA7a(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Desulfonauticus sp. A7A graph.

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
    Instace of Desulfonauticus sp. A7A graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:11:10.577004
	
	The undirected graph Desulfonauticus sp. A7A has 1912 nodes and 135353
	weighted edges, of which none are self-loops. The graph is dense as it
	has a density of 0.07409 and has 5 connected components, where the component
	with most nodes has 1902 nodes and the component with the least nodes has
	2 nodes. The graph median node degree is 116, the mean node degree is 141.58,
	and the node degree mode is 1. The top 5 most central nodes are 1379281.AVAG01000011_gene1246
	(degree 879), 1379281.AVAG01000023_gene1901 (degree 706), 1379281.AVAG01000016_gene954
	(degree 689), 1379281.AVAG01000052_gene912 (degree 675) and 1379281.AVAG01000031_gene106
	(degree 673).
	

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
	    from ensmallen_graph.datasets.string import DesulfonauticusSpA7a
	
	    # Then load the graph
	    graph = DesulfonauticusSpA7a()
	
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
        "DesulfonauticusSpA7a",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
