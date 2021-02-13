"""
This file offers the methods to automatically retrieve the graph rt-retweet-crawl.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:26:37.921189

The undirected graph rt-retweet-crawl has 1112702 nodes and 2278852 unweighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00000 and is connected, as it has a single component.
The graph median node degree is 1, the mean node degree is 4.10, and the
node degree mode is 1. The top 5 most central nodes are 508794 (degree
5070), 27351 (degree 4634), 390165 (degree 4342), 208853 (degree 1673)
and 1095919 (degree 1658).


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

@article{rossi2012fastclique,
        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
        journal={arXiv preprint arXiv:1210.5802},
        pages={1--11},
        year={2012}
}

@inproceedings{rossi2014pmc-www,
        title={Fast Maximum Clique Algorithms for Large Graphs},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
        year={2014}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import RtRetweetCrawl

    # Then load the graph
    graph = RtRetweetCrawl()

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


def RtRetweetCrawl(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the rt-retweet-crawl graph.

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
    Instace of rt-retweet-crawl graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:26:37.921189
	
	The undirected graph rt-retweet-crawl has 1112702 nodes and 2278852 unweighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00000 and is connected, as it has a single component.
	The graph median node degree is 1, the mean node degree is 4.10, and the
	node degree mode is 1. The top 5 most central nodes are 508794 (degree
	5070), 27351 (degree 4634), 390165 (degree 4342), 208853 (degree 1673)
	and 1095919 (degree 1658).
	

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
	
	@article{rossi2012fastclique,
	        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
	        journal={arXiv preprint arXiv:1210.5802},
	        pages={1--11},
	        year={2012}
	}
	
	@inproceedings{rossi2014pmc-www,
	        title={Fast Maximum Clique Algorithms for Large Graphs},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
	        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
	        year={2014}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import RtRetweetCrawl
	
	    # Then load the graph
	    graph = RtRetweetCrawl()
	
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
        "RtRetweetCrawl",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
