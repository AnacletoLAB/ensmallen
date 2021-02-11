"""
This file offers the methods to automatically retrieve the graph soc-slashdot-trust-all.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:48:04.666402

The undirected multigraph soc-slashdot-trust-all has 79120 nodes and 469768
unweighted edges with 2 different edge types: +1 and -1, of which none
are self-loops and 3798 are parallel. The graph is quite sparse as it has
a density of 0.00015 and is connected, as it has a single component. The
graph median node degree is 2, the mean node degree is 11.87, and the node
degree mode is 1. The top 5 most central nodes are 12660 (degree 2537),
12956 (degree 2377), 9449 (degree 2223), 35731 (degree 1709) and 279 (degree
1684).


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

@article{leskovec2009community,
        title={Community structure in large networks: Natural cluster sizes and the absence of large well-defined clusters},
        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
        journal={Internet Mathematics},
        volume={6},
        number={1},
        pages={29--123},
        year={2009},
        publisher={Taylor \& Francis}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocSlashdotTrustAll

    # Then load the graph
    graph = SocSlashdotTrustAll()

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


def SocSlashdotTrustAll(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-slashdot-trust-all graph.

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
    Instace of soc-slashdot-trust-all graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:48:04.666402
	
	The undirected multigraph soc-slashdot-trust-all has 79120 nodes and 469768
	unweighted edges with 2 different edge types: +1 and -1, of which none
	are self-loops and 3798 are parallel. The graph is quite sparse as it has
	a density of 0.00015 and is connected, as it has a single component. The
	graph median node degree is 2, the mean node degree is 11.87, and the node
	degree mode is 1. The top 5 most central nodes are 12660 (degree 2537),
	12956 (degree 2377), 9449 (degree 2223), 35731 (degree 1709) and 279 (degree
	1684).
	


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
	
	@article{leskovec2009community,
	        title={Community structure in large networks: Natural cluster sizes and the absence of large well-defined clusters},
	        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
	        journal={Internet Mathematics},
	        volume={6},
	        number={1},
	        pages={29--123},
	        year={2009},
	        publisher={Taylor \& Francis}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocSlashdotTrustAll
	
	    # Then load the graph
	    graph = SocSlashdotTrustAll()
	
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
        "SocSlashdotTrustAll",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
