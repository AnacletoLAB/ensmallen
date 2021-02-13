"""
This file offers the methods to automatically retrieve the graph eco-mangwet.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:59:07.818789

The undirected graph eco-mangwet has 97 nodes and 1446 weighted edges,
of which none are self-loops. The graph is quite dense as it has a density
of 0.31057 and is connected, as it has a single component. The graph median
node degree is 31, the mean node degree is 29.81 and the node degree mode
is 23. The top 5 most central nodes are 97 (degree 90), 92 (degree 74),
93 (degree 74), 57 (degree 58) and 55 (degree 55).


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

@article{ulanowicz1998network,
        title={Network analysis of trophic dynamics in south florida ecosystems},
        author={Ulanowicz, Robert E and DeAngelis, Donald L},
        journal={FY97: The Florida Bay Ecosystem},
        pages={20688--20038},
        year={1998}
}

@article{melian2004food,
        title={Food web cohesion},
        author={Meli{\'a}n,
Carlos J and Bascompte, Jordi},
        journal={Ecology},
        volume={85},
        number={2},
        pages={352--358},
        year={2004},
        publisher={Eco Soc America}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import EcoMangwet

    # Then load the graph
    graph = EcoMangwet()

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


def EcoMangwet(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the eco-mangwet graph.

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
    Instace of eco-mangwet graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 23:59:07.818789
	
	The undirected graph eco-mangwet has 97 nodes and 1446 weighted edges,
	of which none are self-loops. The graph is quite dense as it has a density
	of 0.31057 and is connected, as it has a single component. The graph median
	node degree is 31, the mean node degree is 29.81 and the node degree mode
	is 23. The top 5 most central nodes are 97 (degree 90), 92 (degree 74),
	93 (degree 74), 57 (degree 58) and 55 (degree 55).
	

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
	
	@article{ulanowicz1998network,
	        title={Network analysis of trophic dynamics in south florida ecosystems},
	        author={Ulanowicz, Robert E and DeAngelis, Donald L},
	        journal={FY97: The Florida Bay Ecosystem},
	        pages={20688--20038},
	        year={1998}
	}
	
	@article{melian2004food,
	        title={Food web cohesion},
	        author={Meli{\'a}n,
	Carlos J and Bascompte, Jordi},
	        journal={Ecology},
	        volume={85},
	        number={2},
	        pages={352--358},
	        year={2004},
	        publisher={Eco Soc America}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import EcoMangwet
	
	    # Then load the graph
	    graph = EcoMangwet()
	
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
        "EcoMangwet",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
