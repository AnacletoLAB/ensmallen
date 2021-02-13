"""
This file offers the methods to automatically retrieve the graph soc-wiki-Vote.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 09:52:09.046570

The undirected graph soc-wiki-Vote has 889 nodes and 2914 unweighted edges,
of which none are self-loops. The graph is sparse as it has a density of
0.00738 and is connected, as it has a single component. The graph median
node degree is 4, the mean node degree is 6.56, and the node degree mode
is 1. The top 5 most central nodes are 431 (degree 102), 273 (degree 92),
170 (degree 66), 536 (degree 60) and 399 (degree 56).


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

@inproceedings{leskovec2010signed,
        title={Signed networks in social media},
        author={Leskovec, Jure and Huttenlocher, Daniel and Kleinberg, Jon},
        booktitle={Proceedings of the SIGCHI Conference on Human Factors in Computing Systems},
        pages={1361--1370},
        year={2010},
        organization={ACM}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocWikiVote

    # Then load the graph
    graph = SocWikiVote()

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


def SocWikiVote(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-wiki-Vote graph.

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
    Instace of soc-wiki-Vote graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 09:52:09.046570
	
	The undirected graph soc-wiki-Vote has 889 nodes and 2914 unweighted edges,
	of which none are self-loops. The graph is sparse as it has a density of
	0.00738 and is connected, as it has a single component. The graph median
	node degree is 4, the mean node degree is 6.56, and the node degree mode
	is 1. The top 5 most central nodes are 431 (degree 102), 273 (degree 92),
	170 (degree 66), 536 (degree 60) and 399 (degree 56).
	

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
	
	@inproceedings{leskovec2010signed,
	        title={Signed networks in social media},
	        author={Leskovec, Jure and Huttenlocher, Daniel and Kleinberg, Jon},
	        booktitle={Proceedings of the SIGCHI Conference on Human Factors in Computing Systems},
	        pages={1361--1370},
	        year={2010},
	        organization={ACM}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocWikiVote
	
	    # Then load the graph
	    graph = SocWikiVote()
	
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
        "SocWikiVote",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
