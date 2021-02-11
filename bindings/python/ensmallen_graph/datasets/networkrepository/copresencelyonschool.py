"""
This file offers the methods to automatically retrieve the graph copresence-LyonSchool.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:43:01.274669

The undirected multigraph copresence-LyonSchool has 242 nodes and 6594492
unweighted edges with 3124 different edge types:  the 5 most common are
46560, 46640, 46580, 46600 and 46680, of which none are self-loops and
13135796 are parallel. The graph is extremely dense as it has a density
of 0.91197 and is connected, as it has a single component. The graph median
node degree is 55504, the mean node degree is 54499.93 and the node degree
mode is 28366. The top 5 most central nodes are 1695 (degree 97198), 1912
(degree 97035), 1675 (degree 96113), 1684 (degree 95074) and 1697 (degree
94706).


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
    from ensmallen_graph.datasets.networkrepository import CopresenceLyonschool

    # Then load the graph
    graph = CopresenceLyonschool()

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


def CopresenceLyonschool(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the copresence-LyonSchool graph.

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
    Instace of copresence-LyonSchool graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:43:01.274669
	
	The undirected multigraph copresence-LyonSchool has 242 nodes and 6594492
	unweighted edges with 3124 different edge types:  the 5 most common are
	46560, 46640, 46580, 46600 and 46680, of which none are self-loops and
	13135796 are parallel. The graph is extremely dense as it has a density
	of 0.91197 and is connected, as it has a single component. The graph median
	node degree is 55504, the mean node degree is 54499.93 and the node degree
	mode is 28366. The top 5 most central nodes are 1695 (degree 97198), 1912
	(degree 97035), 1675 (degree 96113), 1684 (degree 95074) and 1697 (degree
	94706).
	

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
	    from ensmallen_graph.datasets.networkrepository import CopresenceLyonschool
	
	    # Then load the graph
	    graph = CopresenceLyonschool()
	
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
        "CopresenceLyonschool",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
