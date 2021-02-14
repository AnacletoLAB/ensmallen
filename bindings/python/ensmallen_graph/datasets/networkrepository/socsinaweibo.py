"""
This file offers the methods to automatically retrieve the graph soc-sinaweibo.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 10:55:53.742142

The undirected graph soc-sinaweibo has 58655849 nodes and 261321071 unweighted
edges, of which 38 are self-loops. The graph is extremely sparse as it
has a density of 0.00000 and has 15 connected components, where the component
with most nodes has 58655820 nodes and the component with the least nodes
has a single node. The graph median node degree is 2, the mean node degree
is 8.91, and the node degree mode is 1. The top 5 most central nodes are
1029 (degree 278490), 243011 (degree 104633), 179813 (degree 94577), 107829
(degree 94118) and 243610 (degree 93796).


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

@incollection{zhang2014characterizing,
        title={Characterizing Tweeting Behaviors of Sina Weibo Users via Public Data Streaming},
        author={Zhang, Kai and Yu, Qian and Lei, Kai and Xu, Kuai},
        booktitle={Web-Age Information Management},
        pages={294--297},
        year={2014},
        publisher={Springer}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocSinaweibo

    # Then load the graph
    graph = SocSinaweibo()

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


def SocSinaweibo(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the soc-sinaweibo graph.

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
    Instace of soc-sinaweibo graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 10:55:53.742142
	
	The undirected graph soc-sinaweibo has 58655849 nodes and 261321071 unweighted
	edges, of which 38 are self-loops. The graph is extremely sparse as it
	has a density of 0.00000 and has 15 connected components, where the component
	with most nodes has 58655820 nodes and the component with the least nodes
	has a single node. The graph median node degree is 2, the mean node degree
	is 8.91, and the node degree mode is 1. The top 5 most central nodes are
	1029 (degree 278490), 243011 (degree 104633), 179813 (degree 94577), 107829
	(degree 94118) and 243610 (degree 93796).
	

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
	
	@incollection{zhang2014characterizing,
	        title={Characterizing Tweeting Behaviors of Sina Weibo Users via Public Data Streaming},
	        author={Zhang, Kai and Yu, Qian and Lei, Kai and Xu, Kuai},
	        booktitle={Web-Age Information Management},
	        pages={294--297},
	        year={2014},
	        publisher={Springer}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocSinaweibo
	
	    # Then load the graph
	    graph = SocSinaweibo()
	
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
        graph_name="SocSinaweibo",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
