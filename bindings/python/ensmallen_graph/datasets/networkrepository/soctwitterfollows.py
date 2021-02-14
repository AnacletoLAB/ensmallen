"""
This file offers the methods to automatically retrieve the graph soc-twitter-follows.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:48:15.440541

The undirected graph soc-twitter-follows has 404719 nodes and 713319 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00001 and is connected, as it has a single component.
The graph median node degree is 1, the mean node degree is 3.53, and the
node degree mode is 1. The top 5 most central nodes are 355672 (degree
626), 245697 (degree 566), 163661 (degree 554), 194116 (degree 544) and
381201 (degree 529).


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

@article{dprIM,
        author = {Gleich, David F. and Rossi, Ryan A.},
        title = {A Dynamical System for PageRank with Time-Dependent Teleportation},
        journal = {Internet Mathematics},
        volume = {10},
        number = {1-2},
        pages = {188-217},
        year = {2014},
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocTwitterFollows

    # Then load the graph
    graph = SocTwitterFollows()

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
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def SocTwitterFollows(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the soc-twitter-follows graph.

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
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of soc-twitter-follows graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:48:15.440541
	
	The undirected graph soc-twitter-follows has 404719 nodes and 713319 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00001 and is connected, as it has a single component.
	The graph median node degree is 1, the mean node degree is 3.53, and the
	node degree mode is 1. The top 5 most central nodes are 355672 (degree
	626), 245697 (degree 566), 163661 (degree 554), 194116 (degree 544) and
	381201 (degree 529).
	

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
	
	@article{dprIM,
	        author = {Gleich, David F. and Rossi, Ryan A.},
	        title = {A Dynamical System for PageRank with Time-Dependent Teleportation},
	        journal = {Internet Mathematics},
	        volume = {10},
	        number = {1-2},
	        pages = {188-217},
	        year = {2014},
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocTwitterFollows
	
	    # Then load the graph
	    graph = SocTwitterFollows()
	
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
        graph_name="SocTwitterFollows",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
