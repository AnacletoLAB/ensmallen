"""
This file offers the methods to automatically retrieve the graph Bacteroidetes bacterium SCGC AAA027N21.

The graph is automatically retrieved from the STRING repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 22:00:59.623419

The undirected graph Bacteroidetes bacterium SCGC AAA027N21 has 2033 nodes
and 199689 weighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.09668 and has 6 connected components, where the
component with most nodes has 2023 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 183, the mean node degree
is 196.45, and the node degree mode is 1. The top 5 most central nodes
are 938709.AUSH02000045_gene357 (degree 1200), 938709.AUSH02000046_gene54
(degree 928), 938709.AUSH02000015_gene587 (degree 917), 938709.AUSH02000073_gene1733
(degree 792) and 938709.AUSH02000069_gene1531 (degree 751).


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
    from ensmallen_graph.datasets.string import BacteroidetesBacteriumScgcAaa027n21

    # Then load the graph
    graph = BacteroidetesBacteriumScgcAaa027n21()

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

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def BacteroidetesBacteriumScgcAaa027n21(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the Bacteroidetes bacterium SCGC AAA027N21 graph.

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
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of Bacteroidetes bacterium SCGC AAA027N21 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 22:00:59.623419
	
	The undirected graph Bacteroidetes bacterium SCGC AAA027N21 has 2033 nodes
	and 199689 weighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.09668 and has 6 connected components, where the
	component with most nodes has 2023 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 183, the mean node degree
	is 196.45, and the node degree mode is 1. The top 5 most central nodes
	are 938709.AUSH02000045_gene357 (degree 1200), 938709.AUSH02000046_gene54
	(degree 928), 938709.AUSH02000015_gene587 (degree 917), 938709.AUSH02000073_gene1733
	(degree 792) and 938709.AUSH02000069_gene1531 (degree 751).
	

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
	    from ensmallen_graph.datasets.string import BacteroidetesBacteriumScgcAaa027n21
	
	    # Then load the graph
	    graph = BacteroidetesBacteriumScgcAaa027n21()
	
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
        graph_name="BacteroidetesBacteriumScgcAaa027n21",
        dataset="string",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
