"""
This file offers the methods to automatically retrieve the graph soc-epinions.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:44:33.306873

The undirected graph soc-epinions has 26588 nodes and 100120 unweighted
edges, of which none are self-loops. The graph is quite sparse as it has
a density of 0.00028 and is connected, as it has a single component. The
graph median node degree is 2, the mean node degree is 7.53, and the node
degree mode is 1. The top 5 most central nodes are 35 (degree 443), 32
(degree 426), 1783 (degree 423), 486 (degree 422) and 1024 (degree 412).


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

@incollection{richardson2003trust,
        title={Trust management for the semantic web},
        author={Richardson, Matthew and Agrawal, Rakesh and Domingos, Pedro},
        booktitle={The Semantic Web-ISWC 2003},
        pages={351--368},
        year={2003},
        publisher={Springer}
}

@inproceedings{nr:massa05,
        title = {Controversial Users Demand Local Trust Metrics: An Experimental Study on epinions.com Community},
        author = {Paolo Massa and Paolo Avesani},
        booktitle = {AAAI},
        year = {2005},
        pages = {121--126},
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocEpinions

    # Then load the graph
    graph = SocEpinions()

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


def SocEpinions(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-epinions graph.

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
    Instace of soc-epinions graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:44:33.306873
	
	The undirected graph soc-epinions has 26588 nodes and 100120 unweighted
	edges, of which none are self-loops. The graph is quite sparse as it has
	a density of 0.00028 and is connected, as it has a single component. The
	graph median node degree is 2, the mean node degree is 7.53, and the node
	degree mode is 1. The top 5 most central nodes are 35 (degree 443), 32
	(degree 426), 1783 (degree 423), 486 (degree 422) and 1024 (degree 412).
	

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
	
	@incollection{richardson2003trust,
	        title={Trust management for the semantic web},
	        author={Richardson, Matthew and Agrawal, Rakesh and Domingos, Pedro},
	        booktitle={The Semantic Web-ISWC 2003},
	        pages={351--368},
	        year={2003},
	        publisher={Springer}
	}
	
	@inproceedings{nr:massa05,
	        title = {Controversial Users Demand Local Trust Metrics: An Experimental Study on epinions.com Community},
	        author = {Paolo Massa and Paolo Avesani},
	        booktitle = {AAAI},
	        year = {2005},
	        pages = {121--126},
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocEpinions
	
	    # Then load the graph
	    graph = SocEpinions()
	
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
        "SocEpinions",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
