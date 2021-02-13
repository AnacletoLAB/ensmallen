"""
This file offers the methods to automatically retrieve the graph bio-dmela.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:30:40.791750

The undirected graph bio-dmela has 7393 nodes and 25569 unweighted edges,
of which none are self-loops. The graph is quite sparse as it has a density
of 0.00094 and is connected, as it has a single component. The graph median
node degree is 3, the mean node degree is 6.92 and the node degree mode
is 1. The top 5 most central nodes are 215 (degree 190), 663 (degree 149),
708 (degree 122), 209 (degree 115) and 76 (degree 113).


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

@article{singh2008-isorank-multi,
        author = {Singh, Rohit and Xu, Jinbo and Berger, Bonnie},
        title = {Global alignment of multiple protein interaction networks with application to functional orthology detection},
        journal = {PNAS},
        year = {2008},
        volume = {105},
        pages = {12763-12768},
        number = {35}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import BioDmela

    # Then load the graph
    graph = BioDmela()

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


def BioDmela(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the bio-dmela graph.

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
    Instace of bio-dmela graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:30:40.791750
	
	The undirected graph bio-dmela has 7393 nodes and 25569 unweighted edges,
	of which none are self-loops. The graph is quite sparse as it has a density
	of 0.00094 and is connected, as it has a single component. The graph median
	node degree is 3, the mean node degree is 6.92 and the node degree mode
	is 1. The top 5 most central nodes are 215 (degree 190), 663 (degree 149),
	708 (degree 122), 209 (degree 115) and 76 (degree 113).
	

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
	
	@article{singh2008-isorank-multi,
	        author = {Singh, Rohit and Xu, Jinbo and Berger, Bonnie},
	        title = {Global alignment of multiple protein interaction networks with application to functional orthology detection},
	        journal = {PNAS},
	        year = {2008},
	        volume = {105},
	        pages = {12763-12768},
	        number = {35}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import BioDmela
	
	    # Then load the graph
	    graph = BioDmela()
	
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
        "BioDmela",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
