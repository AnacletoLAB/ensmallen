"""
This file offers the methods to automatically retrieve the graph fe-tooth.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:12:38.661572

The undirected graph fe-tooth has 78136 nodes and 452591 unweighted edges,
of which none are self-loops. The graph is quite sparse as it has a density
of 0.00015 and is connected, as it has a single component. The graph median
node degree is 12, the mean node degree is 11.58 and the node degree mode
is 6. The top 5 most central nodes are 29906 (degree 39), 61929 (degree
31), 63865 (degree 30), 64912 (degree 30) and 64904 (degree 30).


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

@inproceedings{bader2012graph,
        title={Graph Partitioning and Graph Clustering},
        author={Bader, David A and Meyerhenke, Henning and Sanders, Peter and Wagner, Dorothea},
        booktitle={10th DIMACS Implementation Challenge Workshop},
        year={2012}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import FeTooth

    # Then load the graph
    graph = FeTooth()

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


def FeTooth(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the fe-tooth graph.

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
    Instace of fe-tooth graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:12:38.661572
	
	The undirected graph fe-tooth has 78136 nodes and 452591 unweighted edges,
	of which none are self-loops. The graph is quite sparse as it has a density
	of 0.00015 and is connected, as it has a single component. The graph median
	node degree is 12, the mean node degree is 11.58 and the node degree mode
	is 6. The top 5 most central nodes are 29906 (degree 39), 61929 (degree
	31), 63865 (degree 30), 64912 (degree 30) and 64904 (degree 30).
	


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
	
	@inproceedings{bader2012graph,
	        title={Graph Partitioning and Graph Clustering},
	        author={Bader, David A and Meyerhenke, Henning and Sanders, Peter and Wagner, Dorothea},
	        booktitle={10th DIMACS Implementation Challenge Workshop},
	        year={2012}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import FeTooth
	
	    # Then load the graph
	    graph = FeTooth()
	
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
        "FeTooth",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
