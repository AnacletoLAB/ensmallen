"""
This file offers the methods to automatically retrieve the graph tech-WHOIS.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 10:55:23.333611

The undirected graph tech-WHOIS has 7476 nodes and 56943 unweighted edges,
of which none are self-loops. The graph is sparse as it has a density of
0.00204 and is connected, as it has a single component. The graph median
node degree is 3, the mean node degree is 15.23, and the node degree mode
is 2. The top 5 most central nodes are 22 (degree 1079), 974 (degree 817),
269 (degree 611), 456 (degree 569) and 2848 (degree 506).


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

@article{mahadevan2006internet,
        title={The Internet AS-level topology: three data sources and one definitive metric},
        author={Mahadevan, P. and Krioukov, D. and Fomenkov, M. and Dimitropoulos, X. and Vahdat, A. and others},
        journal={SIGCOMM},
        volume={36},
        number={1},
        pages={17--26},
        year={2006},
}

@misc{whois,
        author={{WHOIS}},
        title={{Internet} Routing Registries},
        note={{\scriptsize \url{http://www.irr.net/}}}}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import TechWhois

    # Then load the graph
    graph = TechWhois()

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


def TechWhois(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the tech-WHOIS graph.

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
    Instace of tech-WHOIS graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 10:55:23.333611
	
	The undirected graph tech-WHOIS has 7476 nodes and 56943 unweighted edges,
	of which none are self-loops. The graph is sparse as it has a density of
	0.00204 and is connected, as it has a single component. The graph median
	node degree is 3, the mean node degree is 15.23, and the node degree mode
	is 2. The top 5 most central nodes are 22 (degree 1079), 974 (degree 817),
	269 (degree 611), 456 (degree 569) and 2848 (degree 506).
	


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
	
	@article{mahadevan2006internet,
	        title={The Internet AS-level topology: three data sources and one definitive metric},
	        author={Mahadevan, P. and Krioukov, D. and Fomenkov, M. and Dimitropoulos, X. and Vahdat, A. and others},
	        journal={SIGCOMM},
	        volume={36},
	        number={1},
	        pages={17--26},
	        year={2006},
	}
	
	@misc{whois,
	        author={{WHOIS}},
	        title={{Internet} Routing Registries},
	        note={{\scriptsize \url{http://www.irr.net/}}}}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import TechWhois
	
	    # Then load the graph
	    graph = TechWhois()
	
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
        "TechWhois",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
