"""
This file offers the methods to automatically retrieve the graph copresence-InVS15.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:41:38.737094

The undirected multigraph copresence-InVS15 has 219 nodes and 1283194 unweighted
edges with 21536 different edge types:  the 5 most common are 733120, 734620,
733080, 733100 and 733520, of which none are self-loops and 2532938 are
parallel. The graph is extremely dense as it has a density of 0.70064 and
is connected, as it has a single component. The graph median node degree
is 11140, the mean node degree is 11718.67 and the node degree mode is
14370. The top 5 most central nodes are 219 (degree 39888), 889 (degree
39463), 845 (degree 35968), 753 (degree 33828) and 466 (degree 31011).


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

@misc{infect,
        author={{SocioPatterns}},
        title={Infectious contact networks},
        url={http://www.sociopatterns.org/datasets/}}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import CopresenceInvs15

    # Then load the graph
    graph = CopresenceInvs15()

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


def CopresenceInvs15(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the copresence-InVS15 graph.

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
    Instace of copresence-InVS15 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:41:38.737094
	
	The undirected multigraph copresence-InVS15 has 219 nodes and 1283194 unweighted
	edges with 21536 different edge types:  the 5 most common are 733120, 734620,
	733080, 733100 and 733520, of which none are self-loops and 2532938 are
	parallel. The graph is extremely dense as it has a density of 0.70064 and
	is connected, as it has a single component. The graph median node degree
	is 11140, the mean node degree is 11718.67 and the node degree mode is
	14370. The top 5 most central nodes are 219 (degree 39888), 889 (degree
	39463), 845 (degree 35968), 753 (degree 33828) and 466 (degree 31011).
	

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
	
	@misc{infect,
	        author={{SocioPatterns}},
	        title={Infectious contact networks},
	        url={http://www.sociopatterns.org/datasets/}}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import CopresenceInvs15
	
	    # Then load the graph
	    graph = CopresenceInvs15()
	
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
        "CopresenceInvs15",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
