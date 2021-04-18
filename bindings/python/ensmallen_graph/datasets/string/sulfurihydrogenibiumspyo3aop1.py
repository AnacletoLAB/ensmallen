"""
This file offers the methods to automatically retrieve the graph Sulfurihydrogenibium sp. YO3AOP1.

The graph is automatically retrieved from the STRING repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 20:49:02.328670

The undirected graph Sulfurihydrogenibium sp. YO3AOP1 has 1708 nodes and
93358 weighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.06404 and has 3 connected components, where the
component with most nodes has 1704 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 86, the mean node degree
is 109.32, and the node degree mode is 7. The top 5 most central nodes
are 436114.SYO3AOP1_1481 (degree 679), 436114.SYO3AOP1_1628 (degree 634),
436114.SYO3AOP1_1559 (degree 600), 436114.SYO3AOP1_0889 (degree 540) and
436114.SYO3AOP1_0805 (degree 538).


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
    from ensmallen_graph.datasets.string import SulfurihydrogenibiumSpYo3aop1

    # Then load the graph
    graph = SulfurihydrogenibiumSpYo3aop1()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Whether to show a loading bar.
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


def SulfurihydrogenibiumSpYo3aop1(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the Sulfurihydrogenibium sp. YO3AOP1 graph.

    The graph is automatically retrieved from the STRING repository. 

	

    Parameters
    -------------------
    directed: bool = False,
        Whether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Whether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of Sulfurihydrogenibium sp. YO3AOP1 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 20:49:02.328670
	
	The undirected graph Sulfurihydrogenibium sp. YO3AOP1 has 1708 nodes and
	93358 weighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.06404 and has 3 connected components, where the
	component with most nodes has 1704 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 86, the mean node degree
	is 109.32, and the node degree mode is 7. The top 5 most central nodes
	are 436114.SYO3AOP1_1481 (degree 679), 436114.SYO3AOP1_1628 (degree 634),
	436114.SYO3AOP1_1559 (degree 600), 436114.SYO3AOP1_0889 (degree 540) and
	436114.SYO3AOP1_0805 (degree 538).
	

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
	    from ensmallen_graph.datasets.string import SulfurihydrogenibiumSpYo3aop1
	
	    # Then load the graph
	    graph = SulfurihydrogenibiumSpYo3aop1()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Whether to show a loading bar.
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
        graph_name="SulfurihydrogenibiumSpYo3aop1",
        dataset="string",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
