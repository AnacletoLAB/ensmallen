"""
This file offers the methods to automatically retrieve the graph web-BerkStan-dir.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:32:20.197334

The undirected graph web-BerkStan-dir has 685230 nodes and 6649470 weighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00003 and has 676 connected components, where the component
with most nodes has 654782 nodes and the component with the least nodes
has 2 nodes. The graph median node degree is 7, the mean node degree is
19.41, and the node degree mode is 2. The top 5 most central nodes are
10 (degree 84230), 115 (degree 48207), 114 (degree 44292), 598 (degree
44102) and 601 (degree 44076).


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

@inproceedings{leskovec2008statistical,
        title={Statistical properties of community structure in large social and information networks},
        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
        booktitle={Proceedings of the 17th international conference on World Wide Web},
        pages={695--704},
        year={2008},
        organization={ACM}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebBerkstanDir

    # Then load the graph
    graph = WebBerkstanDir()

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


def WebBerkstanDir(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the web-BerkStan-dir graph.

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
    Instace of web-BerkStan-dir graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:32:20.197334
	
	The undirected graph web-BerkStan-dir has 685230 nodes and 6649470 weighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00003 and has 676 connected components, where the component
	with most nodes has 654782 nodes and the component with the least nodes
	has 2 nodes. The graph median node degree is 7, the mean node degree is
	19.41, and the node degree mode is 2. The top 5 most central nodes are
	10 (degree 84230), 115 (degree 48207), 114 (degree 44292), 598 (degree
	44102) and 601 (degree 44076).
	

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
	
	@inproceedings{leskovec2008statistical,
	        title={Statistical properties of community structure in large social and information networks},
	        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
	        booktitle={Proceedings of the 17th international conference on World Wide Web},
	        pages={695--704},
	        year={2008},
	        organization={ACM}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import WebBerkstanDir
	
	    # Then load the graph
	    graph = WebBerkstanDir()
	
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
        "WebBerkstanDir",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
