"""
This file offers the methods to automatically retrieve the graph web-arabic-2005.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:32:38.702352

The undirected graph web-arabic-2005 has 163598 nodes and 1747269 unweighted
edges, of which none are self-loops. The graph is quite sparse as it has
a density of 0.00013 and is connected, as it has a single component. The
graph median node degree is 15, the mean node degree is 21.36, and the
node degree mode is 1. The top 5 most central nodes are 73084 (degree 1102),
71059 (degree 1091), 163190 (degree 420), 4933 (degree 411) and 19161 (degree
268).


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
    from ensmallen_graph.datasets.networkrepository import WebArabic2005

    # Then load the graph
    graph = WebArabic2005()

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


def WebArabic2005(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the web-arabic-2005 graph.

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
    Instace of web-arabic-2005 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:32:38.702352
	
	The undirected graph web-arabic-2005 has 163598 nodes and 1747269 unweighted
	edges, of which none are self-loops. The graph is quite sparse as it has
	a density of 0.00013 and is connected, as it has a single component. The
	graph median node degree is 15, the mean node degree is 21.36, and the
	node degree mode is 1. The top 5 most central nodes are 73084 (degree 1102),
	71059 (degree 1091), 163190 (degree 420), 4933 (degree 411) and 19161 (degree
	268).
	

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
	    from ensmallen_graph.datasets.networkrepository import WebArabic2005
	
	    # Then load the graph
	    graph = WebArabic2005()
	
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
        "WebArabic2005",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
