"""
This file offers the methods to automatically retrieve the graph ca-hollywood-2009.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:48:35.178528

The undirected graph ca-hollywood-2009 has 1069126 nodes and 56306653 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00010 and is connected, as it has a single component.
The graph median node degree is 31, the mean node degree is 105.33 and
the node degree mode is 9. The top 5 most central nodes are 601226 (degree
11467), 599865 (degree 10022), 601227 (degree 9065), 600691 (degree 8293)
and 791344 (degree 8191).


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

@inproceedings{BoVWFI,
        author ={Paolo Boldi and Sebastiano Vigna},
        title = {The {W}eb{G}raph Framework {I}: {C}ompression Techniques},
        year = {2004},
        booktitle= {Proc. of the Thirteenth International World Wide Web Conference (WWW 2004)},
        address={Manhattan, USA},
        pages={595--601},
        publisher={ACM Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import CaHollywood2009

    # Then load the graph
    graph = CaHollywood2009()

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


def CaHollywood2009(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the ca-hollywood-2009 graph.

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
    Instace of ca-hollywood-2009 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:48:35.178528
	
	The undirected graph ca-hollywood-2009 has 1069126 nodes and 56306653 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00010 and is connected, as it has a single component.
	The graph median node degree is 31, the mean node degree is 105.33 and
	the node degree mode is 9. The top 5 most central nodes are 601226 (degree
	11467), 599865 (degree 10022), 601227 (degree 9065), 600691 (degree 8293)
	and 791344 (degree 8191).
	

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
	
	@inproceedings{BoVWFI,
	        author ={Paolo Boldi and Sebastiano Vigna},
	        title = {The {W}eb{G}raph Framework {I}: {C}ompression Techniques},
	        year = {2004},
	        booktitle= {Proc. of the Thirteenth International World Wide Web Conference (WWW 2004)},
	        address={Manhattan, USA},
	        pages={595--601},
	        publisher={ACM Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import CaHollywood2009
	
	    # Then load the graph
	    graph = CaHollywood2009()
	
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
        "CaHollywood2009",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
