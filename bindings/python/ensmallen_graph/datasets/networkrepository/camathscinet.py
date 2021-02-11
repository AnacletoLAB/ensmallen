"""
This file offers the methods to automatically retrieve the graph ca-MathSciNet.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:38:32.294875

The undirected graph ca-MathSciNet has 332689 nodes and 820644 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00001 and is connected, as it has a single component.
The graph median node degree is 3, the mean node degree is 4.93 and the
node degree mode is 1. The top 5 most central nodes are 29552 (degree 496),
30063 (degree 282), 10519 (degree 276), 534 (degree 267) and 31978 (degree
266).


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

@article{palla2008fundamental,
        title={Fundamental statistical features and self-similar properties of tagged networks},
        author={Palla, Gergely and Farkas, Ill{\'e}s J and Pollner,
P{\'e}ter and Der{\'e}nyi,
Imre and Vicsek, Tam{\'a}s},
        journal={New Journal of Physics},
        volume={10},
        number={12},
        pages={123026},
        year={2008},
        publisher={IOP Publishing}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import CaMathscinet

    # Then load the graph
    graph = CaMathscinet()

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


def CaMathscinet(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the ca-MathSciNet graph.

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
    Instace of ca-MathSciNet graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:38:32.294875
	
	The undirected graph ca-MathSciNet has 332689 nodes and 820644 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00001 and is connected, as it has a single component.
	The graph median node degree is 3, the mean node degree is 4.93 and the
	node degree mode is 1. The top 5 most central nodes are 29552 (degree 496),
	30063 (degree 282), 10519 (degree 276), 534 (degree 267) and 31978 (degree
	266).
	


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
	
	@article{palla2008fundamental,
	        title={Fundamental statistical features and self-similar properties of tagged networks},
	        author={Palla, Gergely and Farkas, Ill{\'e}s J and Pollner,
	P{\'e}ter and Der{\'e}nyi,
	Imre and Vicsek, Tam{\'a}s},
	        journal={New Journal of Physics},
	        volume={10},
	        number={12},
	        pages={123026},
	        year={2008},
	        publisher={IOP Publishing}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import CaMathscinet
	
	    # Then load the graph
	    graph = CaMathscinet()
	
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
        "CaMathscinet",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
