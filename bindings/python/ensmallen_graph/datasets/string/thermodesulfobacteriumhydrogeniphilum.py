"""
This file offers the methods to automatically retrieve the graph Thermodesulfobacterium hydrogeniphilum.

The graph is automatically retrieved from the STRING repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 19:53:42.219820

The undirected graph Thermodesulfobacterium hydrogeniphilum has 1670 nodes
and 129728 weighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.09309 and has 3 connected components, where the
component with most nodes has 1665 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 134, the mean node degree
is 155.36, and the node degree mode is 21. The top 5 most central nodes
are 161156.JQKW01000007_gene614 (degree 823), 161156.JQKW01000008_gene538
(degree 682), 161156.JQKW01000007_gene640 (degree 675), 161156.JQKW01000014_gene240
(degree 623) and 161156.JQKW01000009_gene1576 (degree 600).


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
    from ensmallen_graph.datasets.string import ThermodesulfobacteriumHydrogeniphilum

    # Then load the graph
    graph = ThermodesulfobacteriumHydrogeniphilum()

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


def ThermodesulfobacteriumHydrogeniphilum(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Thermodesulfobacterium hydrogeniphilum graph.

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
    Instace of Thermodesulfobacterium hydrogeniphilum graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 19:53:42.219820
	
	The undirected graph Thermodesulfobacterium hydrogeniphilum has 1670 nodes
	and 129728 weighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.09309 and has 3 connected components, where the
	component with most nodes has 1665 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 134, the mean node degree
	is 155.36, and the node degree mode is 21. The top 5 most central nodes
	are 161156.JQKW01000007_gene614 (degree 823), 161156.JQKW01000008_gene538
	(degree 682), 161156.JQKW01000007_gene640 (degree 675), 161156.JQKW01000014_gene240
	(degree 623) and 161156.JQKW01000009_gene1576 (degree 600).
	

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
	    from ensmallen_graph.datasets.string import ThermodesulfobacteriumHydrogeniphilum
	
	    # Then load the graph
	    graph = ThermodesulfobacteriumHydrogeniphilum()
	
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
        "ThermodesulfobacteriumHydrogeniphilum",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="string"
    )()
