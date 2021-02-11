"""
This file offers the methods to automatically retrieve the graph misc-lesmis.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 08:19:09.338457

The undirected graph misc-lesmis has 77 nodes and 254 unweighted edges
with 17 different edge types:  the 5 most common are 1, 2, 3, 4 and 5,
of which none are self-loops. The graph is dense as it has a density of
0.08681 and is connected, as it has a single component. The graph median
node degree is 6, the mean node degree is 6.60, and the node degree mode
is 1. The top 5 most central nodes are 12 (degree 36), 49 (degree 22),
56 (degree 19), 28 (degree 17) and 26 (degree 16).


References
---------------------
Please cite the following if you use the data:

@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}

@article{newman2004finding,
        title={Finding and evaluating community structure in networks},
        author={Newman, Mark EJ and Girvan, Michelle},
        journal={Physical review E},
        volume={69},
        number={2},
        pages={026113},
        year={2004},
        publisher={APS}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import MiscLesmis

    # Then load the graph
    graph = MiscLesmis()

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


def MiscLesmis(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the misc-lesmis graph.

    The graph is automatically retrieved from the NetworkRepository repository. 

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
    Instace of misc-lesmis graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 08:19:09.338457
	
	The undirected graph misc-lesmis has 77 nodes and 254 unweighted edges
	with 17 different edge types:  the 5 most common are 1, 2, 3, 4 and 5,
	of which none are self-loops. The graph is dense as it has a density of
	0.08681 and is connected, as it has a single component. The graph median
	node degree is 6, the mean node degree is 6.60, and the node degree mode
	is 1. The top 5 most central nodes are 12 (degree 36), 49 (degree 22),
	56 (degree 19), 28 (degree 17) and 26 (degree 16).
	


	References
	---------------------
	Please cite the following if you use the data:
	
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	
	@article{newman2004finding,
	        title={Finding and evaluating community structure in networks},
	        author={Newman, Mark EJ and Girvan, Michelle},
	        journal={Physical review E},
	        volume={69},
	        number={2},
	        pages={026113},
	        year={2004},
	        publisher={APS}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import MiscLesmis
	
	    # Then load the graph
	    graph = MiscLesmis()
	
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
        "MiscLesmis",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
