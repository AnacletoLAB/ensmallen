"""
This file offers the methods to automatically retrieve the graph web-uk-2002-all.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-05 08:12:13.044596

The undirected graph web-uk-2002-all has 18484117 nodes, of which 931 are
singletons (all have self-loops), and 267657357 unweighted edges, of which
5870099 are self-loops. The graph is extremely sparse as it has a density
of 0.00000 and has 1990 connected components, where the component with
most nodes has 18459128 nodes and the component with the least nodes has
a single node. The graph median node degree is 11, the mean node degree
is 28.64, and the node degree mode is 1. The top 5 most central nodes are
17159800 (degree 194955), 13118356 (degree 102239), 15237350 (degree 83861),
8504955 (degree 73231) and 6748292 (degree 41933).


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

@ARTICLE{boldi2004-ubicrawler,
        author = {Paolo Boldi and Bruno Codenotti and Massimo Santini and Sebastiano Vigna},
        title = {{UbiCrawler}: A Scalable Fully Distributed Web Crawler},
        journal = {Software: Practice \& Experience},
        year = {2004},
        volume = {34},
        pages = {711--726},
        number = {8}}

@INPROCEEDINGS{Boldi-2011-layered,
        author = {Paolo Boldi and Marco Rosa and Massimo Santini and Sebastiano Vigna},
        title = {Layered Label Propagation: A MultiResolution Coordinate-Free Ordering	for Compressing Social Networks},
        booktitle = {WWW},
        year = {2011},
        pages = {587--596}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebUk2002All

    # Then load the graph
    graph = WebUk2002All()

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


def WebUk2002All(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the web-uk-2002-all graph.

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
    Instace of web-uk-2002-all graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-05 08:12:13.044596
	
	The undirected graph web-uk-2002-all has 18484117 nodes, of which 931 are
	singletons (all have self-loops), and 267657357 unweighted edges, of which
	5870099 are self-loops. The graph is extremely sparse as it has a density
	of 0.00000 and has 1990 connected components, where the component with
	most nodes has 18459128 nodes and the component with the least nodes has
	a single node. The graph median node degree is 11, the mean node degree
	is 28.64, and the node degree mode is 1. The top 5 most central nodes are
	17159800 (degree 194955), 13118356 (degree 102239), 15237350 (degree 83861),
	8504955 (degree 73231) and 6748292 (degree 41933).
	

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
	
	@ARTICLE{boldi2004-ubicrawler,
	        author = {Paolo Boldi and Bruno Codenotti and Massimo Santini and Sebastiano Vigna},
	        title = {{UbiCrawler}: A Scalable Fully Distributed Web Crawler},
	        journal = {Software: Practice \& Experience},
	        year = {2004},
	        volume = {34},
	        pages = {711--726},
	        number = {8}}
	
	@INPROCEEDINGS{Boldi-2011-layered,
	        author = {Paolo Boldi and Marco Rosa and Massimo Santini and Sebastiano Vigna},
	        title = {Layered Label Propagation: A MultiResolution Coordinate-Free Ordering	for Compressing Social Networks},
	        booktitle = {WWW},
	        year = {2011},
	        pages = {587--596}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import WebUk2002All
	
	    # Then load the graph
	    graph = WebUk2002All()
	
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
        "WebUk2002All",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
