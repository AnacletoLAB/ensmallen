"""
This file offers the methods to automatically retrieve the graph copresence-LH10.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:41:59.727426

The undirected multigraph copresence-LH10 has 73 nodes and 150126 unweighted
edges with 12605 different edge types:  the 5 most common are 119860, 119700,
119840, 119620 and 119460, of which none are self-loops and 297490 are
parallel. The graph is extremely dense as it has a density of 0.52549 and
is connected, as it has a single component. The graph median node degree
is 3563, the mean node degree is 4113.04 and the node degree mode is 2352.
The top 5 most central nodes are 1320 (degree 15599), 1391 (degree 14621),
1374 (degree 11409), 1148 (degree 10330) and 1362 (degree 10098).


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
    from ensmallen_graph.datasets.networkrepository import CopresenceLh10

    # Then load the graph
    graph = CopresenceLh10()

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


def CopresenceLh10(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the copresence-LH10 graph.

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
    Instace of copresence-LH10 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:41:59.727426
	
	The undirected multigraph copresence-LH10 has 73 nodes and 150126 unweighted
	edges with 12605 different edge types:  the 5 most common are 119860, 119700,
	119840, 119620 and 119460, of which none are self-loops and 297490 are
	parallel. The graph is extremely dense as it has a density of 0.52549 and
	is connected, as it has a single component. The graph median node degree
	is 3563, the mean node degree is 4113.04 and the node degree mode is 2352.
	The top 5 most central nodes are 1320 (degree 15599), 1391 (degree 14621),
	1374 (degree 11409), 1148 (degree 10330) and 1362 (degree 10098).
	

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
	    from ensmallen_graph.datasets.networkrepository import CopresenceLh10
	
	    # Then load the graph
	    graph = CopresenceLh10()
	
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
        "CopresenceLh10",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
