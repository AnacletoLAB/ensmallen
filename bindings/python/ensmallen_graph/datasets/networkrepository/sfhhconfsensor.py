"""
This file offers the methods to automatically retrieve the graph SFHH-conf-sensor.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:35:03.981918

The undirected multigraph SFHH-conf-sensor has 403 nodes and 70261 unweighted
edges with 3509 different edge types:  the 5 most common are 41320, 41280,
41240, 60860 and 41300, of which none are self-loops and 121392 are parallel.
The graph is quite dense as it has a density of 0.11808 and is connected,
as it has a single component. The graph median node degree is 220, the
mean node degree is 348.69 and the node degree mode is 189. The top 5 most
central nodes are 1825 (degree 2446), 1525 (degree 2241), 1549 (degree
2189), 1599 (degree 2112) and 1441 (degree 1994).


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
    from ensmallen_graph.datasets.networkrepository import SfhhConfSensor

    # Then load the graph
    graph = SfhhConfSensor()

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


def SfhhConfSensor(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the SFHH-conf-sensor graph.

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
    Instace of SFHH-conf-sensor graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:35:03.981918
	
	The undirected multigraph SFHH-conf-sensor has 403 nodes and 70261 unweighted
	edges with 3509 different edge types:  the 5 most common are 41320, 41280,
	41240, 60860 and 41300, of which none are self-loops and 121392 are parallel.
	The graph is quite dense as it has a density of 0.11808 and is connected,
	as it has a single component. The graph median node degree is 220, the
	mean node degree is 348.69 and the node degree mode is 189. The top 5 most
	central nodes are 1825 (degree 2446), 1525 (degree 2241), 1549 (degree
	2189), 1599 (degree 2112) and 1441 (degree 1994).
	

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
	    from ensmallen_graph.datasets.networkrepository import SfhhConfSensor
	
	    # Then load the graph
	    graph = SfhhConfSensor()
	
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
        "SfhhConfSensor",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
