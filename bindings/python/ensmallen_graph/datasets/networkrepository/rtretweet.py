"""
This file offers the methods to automatically retrieve the graph rt-retweet.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 09:32:34.413628

The undirected graph rt-retweet has 96 nodes and 117 unweighted edges,
of which none are self-loops. The graph is dense as it has a density of
0.02566 and is connected, as it has a single component. The graph median
node degree is 1, the mean node degree is 2.44, and the node degree mode
is 1. The top 5 most central nodes are 54 (degree 17), 45 (degree 11),
72 (degree 9), 89 (degree 9) and 93 (degree 8).


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

@article{rossi2012fastclique,
        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
        journal={arXiv preprint arXiv:1210.5802},
        pages={1--11},
        year={2012}
}

@inproceedings{rossi2014pmc-www,
        title={Fast Maximum Clique Algorithms for Large Graphs},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
        year={2014}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import RtRetweet

    # Then load the graph
    graph = RtRetweet()

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


def RtRetweet(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the rt-retweet graph.

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
    Instace of rt-retweet graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 09:32:34.413628
	
	The undirected graph rt-retweet has 96 nodes and 117 unweighted edges,
	of which none are self-loops. The graph is dense as it has a density of
	0.02566 and is connected, as it has a single component. The graph median
	node degree is 1, the mean node degree is 2.44, and the node degree mode
	is 1. The top 5 most central nodes are 54 (degree 17), 45 (degree 11),
	72 (degree 9), 89 (degree 9) and 93 (degree 8).
	

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
	
	@article{rossi2012fastclique,
	        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
	        journal={arXiv preprint arXiv:1210.5802},
	        pages={1--11},
	        year={2012}
	}
	
	@inproceedings{rossi2014pmc-www,
	        title={Fast Maximum Clique Algorithms for Large Graphs},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
	        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
	        year={2014}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import RtRetweet
	
	    # Then load the graph
	    graph = RtRetweet()
	
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
        "RtRetweet",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
