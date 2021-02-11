"""
This file offers the methods to automatically retrieve the graph tech-p2p-gnutella.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:52:10.440713

The undirected graph tech-p2p-gnutella has 62561 nodes and 147878 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00008 and is connected, as it has a single component.
The graph median node degree is 2, the mean node degree is 4.73, and the
node degree mode is 1. The top 5 most central nodes are 5310 (degree 95),
31884 (degree 78), 165 (degree 70), 62074 (degree 66) and 51244 (degree
62).


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

@article{matei2002mapping,
        title={Mapping the Gnutella network},
        author={Matei, R. and Iamnitchi, A. and Foster, P.},
        journal={Internet Computing},
        volume={6},
        number={1},
        pages={50--57},
        year={2002},
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import TechP2pGnutella

    # Then load the graph
    graph = TechP2pGnutella()

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


def TechP2pGnutella(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the tech-p2p-gnutella graph.

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
    Instace of tech-p2p-gnutella graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:52:10.440713
	
	The undirected graph tech-p2p-gnutella has 62561 nodes and 147878 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00008 and is connected, as it has a single component.
	The graph median node degree is 2, the mean node degree is 4.73, and the
	node degree mode is 1. The top 5 most central nodes are 5310 (degree 95),
	31884 (degree 78), 165 (degree 70), 62074 (degree 66) and 51244 (degree
	62).
	


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
	
	@article{matei2002mapping,
	        title={Mapping the Gnutella network},
	        author={Matei, R. and Iamnitchi, A. and Foster, P.},
	        journal={Internet Computing},
	        volume={6},
	        number={1},
	        pages={50--57},
	        year={2002},
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import TechP2pGnutella
	
	    # Then load the graph
	    graph = TechP2pGnutella()
	
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
        "TechP2pGnutella",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
