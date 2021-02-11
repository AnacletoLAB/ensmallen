"""
This file offers the methods to automatically retrieve the graph aff-github-user2project.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:02:45.361855

The undirected graph aff-github-user2project has 120867 nodes, of which
2 are singletons (all have self-loops), and 439930 weighted edges, of which
72 are self-loops. The graph is extremely sparse as it has a density of
0.00006 and has 3 connected components, where the component with most nodes
has 120865 nodes and the component with the least nodes has a single node.
The graph median node degree is 2, the mean node degree is 7.28, and the
node degree mode is 1. The top 5 most central nodes are 17 (degree 3692),
299 (degree 1950), 645 (degree 1644), 76 (degree 1597) and 298 (degree
1492).


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

@misc{aff-github-user2project,
        author = {Scott Chacon},
        title = {The 2009 GitHub Contest},
        month = {July},
        year = {2009}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import AffGithubUser2project

    # Then load the graph
    graph = AffGithubUser2project()

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


def AffGithubUser2project(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the aff-github-user2project graph.

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
    Instace of aff-github-user2project graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:02:45.361855
	
	The undirected graph aff-github-user2project has 120867 nodes, of which
	2 are singletons (all have self-loops), and 439930 weighted edges, of which
	72 are self-loops. The graph is extremely sparse as it has a density of
	0.00006 and has 3 connected components, where the component with most nodes
	has 120865 nodes and the component with the least nodes has a single node.
	The graph median node degree is 2, the mean node degree is 7.28, and the
	node degree mode is 1. The top 5 most central nodes are 17 (degree 3692),
	299 (degree 1950), 645 (degree 1644), 76 (degree 1597) and 298 (degree
	1492).
	

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
	
	@misc{aff-github-user2project,
	        author = {Scott Chacon},
	        title = {The 2009 GitHub Contest},
	        month = {July},
	        year = {2009}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import AffGithubUser2project
	
	    # Then load the graph
	    graph = AffGithubUser2project()
	
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
        "AffGithubUser2project",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
