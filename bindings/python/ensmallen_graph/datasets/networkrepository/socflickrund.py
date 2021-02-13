"""
This file offers the methods to automatically retrieve the graph soc-flickr-und.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 16:04:44.674554

The undirected graph soc-flickr-und has 1715255 nodes and 15555042 unweighted
edges, of which 1 are self-loops. The graph is extremely sparse as it has
a density of 0.00001 and has 20318 connected components, where the component
with most nodes has 1624992 nodes and the component with the least nodes
has 2 nodes. The graph median node degree is 1, the mean node degree is
18.14, and the node degree mode is 1. The top 5 most central nodes are
847 (degree 27236), 3410 (degree 19581), 3421 (degree 19129), 397 (degree
16348) and 915 (degree 14483).


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

@inproceedings{mislove2008growth,
        title={Growth of the flickr social network},
        author={Mislove, Alan and Koppula, Hema Swetha and Gummadi, Krishna P and Druschel, Peter and Bhattacharjee, Bobby},
        booktitle={Proceedings of the first workshop on Online social networks},
        pages={25--30},
        year={2008},
        organization={ACM}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocFlickrUnd

    # Then load the graph
    graph = SocFlickrUnd()

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


def SocFlickrUnd(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-flickr-und graph.

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
    Instace of soc-flickr-und graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-07 16:04:44.674554
	
	The undirected graph soc-flickr-und has 1715255 nodes and 15555042 unweighted
	edges, of which 1 are self-loops. The graph is extremely sparse as it has
	a density of 0.00001 and has 20318 connected components, where the component
	with most nodes has 1624992 nodes and the component with the least nodes
	has 2 nodes. The graph median node degree is 1, the mean node degree is
	18.14, and the node degree mode is 1. The top 5 most central nodes are
	847 (degree 27236), 3410 (degree 19581), 3421 (degree 19129), 397 (degree
	16348) and 915 (degree 14483).
	

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
	
	@inproceedings{mislove2008growth,
	        title={Growth of the flickr social network},
	        author={Mislove, Alan and Koppula, Hema Swetha and Gummadi, Krishna P and Druschel, Peter and Bhattacharjee, Bobby},
	        booktitle={Proceedings of the first workshop on Online social networks},
	        pages={25--30},
	        year={2008},
	        organization={ACM}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocFlickrUnd
	
	    # Then load the graph
	    graph = SocFlickrUnd()
	
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
        "SocFlickrUnd",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
