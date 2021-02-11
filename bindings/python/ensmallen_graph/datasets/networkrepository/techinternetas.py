"""
This file offers the methods to automatically retrieve the graph tech-internet-as.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 10:55:27.406613

The undirected graph tech-internet-as has 40164 nodes and 85123 unweighted edges,
of which none are self-loops. The graph is quite sparse as it has a density of 0.00011
and is connected, as it has a single component. The graph median node degree is 2,
the mean node degree is 4.24, and the node degree mode is 2. The top 5 most central
nodes are 8440 (degree 3370), 2064 (degree 3171), 31733 (degree 2423), 35973 (degree
1925) and 21680 (degree 1884).


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

@inproceedings{rossi2013topology,
        title={A Multi-Level Approach for Evaluating Internet Topology Generators},
        author={Ryan A. Rossi and Sonia Fahmy and Nilothpal Talukder},
        booktitle={Networking},
        pages={1--9},
        year={2013}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import TechInternetAs

    # Then load the graph
    graph = TechInternetAs()

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


def TechInternetAs(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the tech-internet-as graph.

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
    Instace of tech-internet-as graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 10:55:27.406613
	
	The undirected graph tech-internet-as has 40164 nodes and 85123 unweighted edges,
	of which none are self-loops. The graph is quite sparse as it has a density of 0.00011
	and is connected, as it has a single component. The graph median node degree is 2,
	the mean node degree is 4.24, and the node degree mode is 2. The top 5 most central
	nodes are 8440 (degree 3370), 2064 (degree 3171), 31733 (degree 2423), 35973 (degree
	1925) and 21680 (degree 1884).
	


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
	
	@inproceedings{rossi2013topology,
	        title={A Multi-Level Approach for Evaluating Internet Topology Generators},
	        author={Ryan A. Rossi and Sonia Fahmy and Nilothpal Talukder},
	        booktitle={Networking},
	        pages={1--9},
	        year={2013}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import TechInternetAs
	
	    # Then load the graph
	    graph = TechInternetAs()
	
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
        "TechInternetAs",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
