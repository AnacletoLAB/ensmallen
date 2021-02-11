"""
This file offers the methods to automatically retrieve the graph primary-school-proximity.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:37:41.721769

The undirected multigraph primary-school-proximity has 242 nodes and 125773
unweighted edges with 3100 different edge types:  the 5 most common are
38900, 144520, 38520, 38460 and 39040, of which none are self-loops and
234912 are parallel. The graph is quite dense as it has a density of 0.28521
and is connected, as it has a single component. The graph median node degree
is 1000, the mean node degree is 1039.45, and the node degree mode is 1033.
The top 5 most central nodes are 1695 (degree 2594), 1697 (degree 2532),
1665 (degree 2447), 1698 (degree 2442) and 1613 (degree 2249).


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
    from ensmallen_graph.datasets.networkrepository import PrimarySchoolProximity

    # Then load the graph
    graph = PrimarySchoolProximity()

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


def PrimarySchoolProximity(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the primary-school-proximity graph.

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
    Instace of primary-school-proximity graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:37:41.721769
	
	The undirected multigraph primary-school-proximity has 242 nodes and 125773
	unweighted edges with 3100 different edge types:  the 5 most common are
	38900, 144520, 38520, 38460 and 39040, of which none are self-loops and
	234912 are parallel. The graph is quite dense as it has a density of 0.28521
	and is connected, as it has a single component. The graph median node degree
	is 1000, the mean node degree is 1039.45, and the node degree mode is 1033.
	The top 5 most central nodes are 1695 (degree 2594), 1697 (degree 2532),
	1665 (degree 2447), 1698 (degree 2442) and 1613 (degree 2249).
	

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
	    from ensmallen_graph.datasets.networkrepository import PrimarySchoolProximity
	
	    # Then load the graph
	    graph = PrimarySchoolProximity()
	
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
        "PrimarySchoolProximity",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
