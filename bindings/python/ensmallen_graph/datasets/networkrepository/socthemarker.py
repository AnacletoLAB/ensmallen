"""
This file offers the methods to automatically retrieve the graph soc-themarker.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:30:30.816399

The undirected graph soc-themarker has 69413 nodes and 1644849 unweighted
edges, of which 6 are self-loops. The graph is quite sparse as it has a
density of 0.00068 and has 48 connected components, where the component
with most nodes has 69317 nodes and the component with the least nodes
has 2 nodes. The graph median node degree is 6, the mean node degree is
47.39, and the node degree mode is 1. The top 5 most central nodes are
4856 (degree 8930), 64699 (degree 8019), 31469 (degree 7608), 53121 (degree
7089) and 51000 (degree 6625).


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

@inproceedings{Fire2011,
        title={Link Prediction in Social Networks using Computationally Efficient Topological Features},
        author={Fire, M. and Tenenboim, L. and Lesser, O. and Puzis, R. and Rokach, L. and Elovici, Y.},
        booktitle={ IEEE Third International Confernece on Social Computing (SocialCom)},
        pages={73--80},
        year={2011},
        organization={IEEE}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocThemarker

    # Then load the graph
    graph = SocThemarker()

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


def SocThemarker(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-themarker graph.

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
    Instace of soc-themarker graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:30:30.816399
	
	The undirected graph soc-themarker has 69413 nodes and 1644849 unweighted
	edges, of which 6 are self-loops. The graph is quite sparse as it has a
	density of 0.00068 and has 48 connected components, where the component
	with most nodes has 69317 nodes and the component with the least nodes
	has 2 nodes. The graph median node degree is 6, the mean node degree is
	47.39, and the node degree mode is 1. The top 5 most central nodes are
	4856 (degree 8930), 64699 (degree 8019), 31469 (degree 7608), 53121 (degree
	7089) and 51000 (degree 6625).
	

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
	
	@inproceedings{Fire2011,
	        title={Link Prediction in Social Networks using Computationally Efficient Topological Features},
	        author={Fire, M. and Tenenboim, L. and Lesser, O. and Puzis, R. and Rokach, L. and Elovici, Y.},
	        booktitle={ IEEE Third International Confernece on Social Computing (SocialCom)},
	        pages={73--80},
	        year={2011},
	        organization={IEEE}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocThemarker
	
	    # Then load the graph
	    graph = SocThemarker()
	
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
        "SocThemarker",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
