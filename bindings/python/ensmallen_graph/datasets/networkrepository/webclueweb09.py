"""
This file offers the methods to automatically retrieve the graph web-ClueWeb09.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 19:19:43.776124

The undirected graph web-ClueWeb09 has 1684868322 nodes and 7811385827
unweighted edges, of which none are self-loops. The graph is extremely
sparse as it has a density of 0.00000 and has 5642809 connected components,
where the component with most nodes has 1592230585 nodes and the component
with the least nodes has 2 nodes. The graph median node degree is 1, the
mean node degree is 9.27, and the node degree mode is 1. The top 5 most
central nodes are 0 (degree 6444720), 2 (degree 6045226), 4 (degree 5104665),
1 (degree 4937883) and 9 (degree 4160186).


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

@techreport{clarke2009overview,
        title={Overview of the trec 2009 web track},
        author={Clarke, Charles L and Craswell, Nick and Soboroff, Ian},
        year={2009},
        institution={DTIC Document}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebClueweb09

    # Then load the graph
    graph = WebClueweb09()

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


def WebClueweb09(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the web-ClueWeb09 graph.

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
    Instace of web-ClueWeb09 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 19:19:43.776124
	
	The undirected graph web-ClueWeb09 has 1684868322 nodes and 7811385827
	unweighted edges, of which none are self-loops. The graph is extremely
	sparse as it has a density of 0.00000 and has 5642809 connected components,
	where the component with most nodes has 1592230585 nodes and the component
	with the least nodes has 2 nodes. The graph median node degree is 1, the
	mean node degree is 9.27, and the node degree mode is 1. The top 5 most
	central nodes are 0 (degree 6444720), 2 (degree 6045226), 4 (degree 5104665),
	1 (degree 4937883) and 9 (degree 4160186).
	

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
	
	@techreport{clarke2009overview,
	        title={Overview of the trec 2009 web track},
	        author={Clarke, Charles L and Craswell, Nick and Soboroff, Ian},
	        year={2009},
	        institution={DTIC Document}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import WebClueweb09
	
	    # Then load the graph
	    graph = WebClueweb09()
	
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
        "WebClueweb09",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
