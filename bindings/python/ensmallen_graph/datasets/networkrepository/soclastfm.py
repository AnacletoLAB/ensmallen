"""
This file offers the methods to automatically retrieve the graph soc-lastfm.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:30:22.831337

The undirected graph soc-lastfm has 1191805 nodes and 4519330 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00001 and is connected, as it has a single component.
The graph median node degree is 2, the mean node degree is 7.58, and the
node degree mode is 1. The top 5 most central nodes are 106938 (degree
5150), 108034 (degree 4492), 107095 (degree 3011), 107974 (degree 2593)
and 108129 (degree 2216).


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

@inproceedings{konstas2009social,
        title={On social networks and collaborative recommendation},
        author={Konstas, Ioannis and Stathopoulos, Vassilios and Jose, Joemon M},
        booktitle={Proceedings of the 32nd international ACM SIGIR conference on Research and development in information retrieval},
        pages={195--202},
        year={2009},
        organization={ACM}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocLastfm

    # Then load the graph
    graph = SocLastfm()

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


def SocLastfm(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the soc-lastfm graph.

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
    Instace of soc-lastfm graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:30:22.831337
	
	The undirected graph soc-lastfm has 1191805 nodes and 4519330 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00001 and is connected, as it has a single component.
	The graph median node degree is 2, the mean node degree is 7.58, and the
	node degree mode is 1. The top 5 most central nodes are 106938 (degree
	5150), 108034 (degree 4492), 107095 (degree 3011), 107974 (degree 2593)
	and 108129 (degree 2216).
	

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
	
	@inproceedings{konstas2009social,
	        title={On social networks and collaborative recommendation},
	        author={Konstas, Ioannis and Stathopoulos, Vassilios and Jose, Joemon M},
	        booktitle={Proceedings of the 32nd international ACM SIGIR conference on Research and development in information retrieval},
	        pages={195--202},
	        year={2009},
	        organization={ACM}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocLastfm
	
	    # Then load the graph
	    graph = SocLastfm()
	
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
        graph_name="SocLastfm",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
