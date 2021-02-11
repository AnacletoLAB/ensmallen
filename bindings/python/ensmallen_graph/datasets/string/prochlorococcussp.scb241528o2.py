"""
This file offers the methods to automatically retrieve the graph Prochlorococcus sp. scB241528O2.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:12:32.838099

The undirected graph Prochlorococcus sp. scB241528O2 has 1830 nodes and
105019 weighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.06275 and has 56 connected components, where the
component with most nodes has 1678 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 98, the mean node degree
is 114.77, and the node degree mode is 1. The top 5 most central nodes
are 1471459.JFLJ01000130_gene1539 (degree 708), 1471459.JFLJ01000156_gene868
(degree 542), 1471459.JFLJ01000114_gene653 (degree 536), 1471459.JFLJ01000156_gene1013
(degree 518) and 1471459.JFLJ01000108_gene1680 (degree 511).


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
    from ensmallen_graph.datasets.string import ProchlorococcusSp.Scb241528o2

    # Then load the graph
    graph = ProchlorococcusSp.Scb241528o2()

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


def ProchlorococcusSp.Scb241528o2(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Prochlorococcus sp. scB241528O2 graph.

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
    Instace of Prochlorococcus sp. scB241528O2 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:12:32.838099
	
	The undirected graph Prochlorococcus sp. scB241528O2 has 1830 nodes and
	105019 weighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.06275 and has 56 connected components, where the
	component with most nodes has 1678 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 98, the mean node degree
	is 114.77, and the node degree mode is 1. The top 5 most central nodes
	are 1471459.JFLJ01000130_gene1539 (degree 708), 1471459.JFLJ01000156_gene868
	(degree 542), 1471459.JFLJ01000114_gene653 (degree 536), 1471459.JFLJ01000156_gene1013
	(degree 518) and 1471459.JFLJ01000108_gene1680 (degree 511).
	

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
	    from ensmallen_graph.datasets.string import ProchlorococcusSp.Scb241528o2
	
	    # Then load the graph
	    graph = ProchlorococcusSp.Scb241528o2()
	
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
        "ProchlorococcusSp.Scb241528o2",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
