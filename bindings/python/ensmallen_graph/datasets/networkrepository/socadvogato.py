"""
This file offers the methods to automatically retrieve the graph soc-advogato.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 10:54:54.557072

The undirected graph soc-advogato has 6551 nodes, of which 1384 are singletons
(all have self-loops), and 43427 weighted edges, of which 3995 are self-loops.
The graph is sparse as it has a density of 0.00193 and has 1441 connected
components, where the component with most nodes has 5054 nodes and the
component with the least nodes has a single node. The graph median node
degree is 3, the mean node degree is 12.65, and the node degree mode is
1. The top 5 most central nodes are 157 (degree 808), 46 (degree 764),
597 (degree 551), 30 (degree 526) and 328 (degree 407).


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

@inproceedings{massa2009bowling,
        title={Bowling alone and trust decline in social network sites},
        author={Massa, Paolo and Salvetti, Martino and Tomasoni, Danilo},
        booktitle={Dependable, Autonomic and Secure Computing, 2009. DASC'09. Eighth IEEE International Conference on},
        pages={658--663},
        year={2009},
        organization={IEEE}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocAdvogato

    # Then load the graph
    graph = SocAdvogato()

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


def SocAdvogato(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the soc-advogato graph.

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
    Instace of soc-advogato graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 10:54:54.557072
	
	The undirected graph soc-advogato has 6551 nodes, of which 1384 are singletons
	(all have self-loops), and 43427 weighted edges, of which 3995 are self-loops.
	The graph is sparse as it has a density of 0.00193 and has 1441 connected
	components, where the component with most nodes has 5054 nodes and the
	component with the least nodes has a single node. The graph median node
	degree is 3, the mean node degree is 12.65, and the node degree mode is
	1. The top 5 most central nodes are 157 (degree 808), 46 (degree 764),
	597 (degree 551), 30 (degree 526) and 328 (degree 407).
	

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
	
	@inproceedings{massa2009bowling,
	        title={Bowling alone and trust decline in social network sites},
	        author={Massa, Paolo and Salvetti, Martino and Tomasoni, Danilo},
	        booktitle={Dependable, Autonomic and Secure Computing, 2009. DASC'09. Eighth IEEE International Conference on},
	        pages={658--663},
	        year={2009},
	        organization={IEEE}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocAdvogato
	
	    # Then load the graph
	    graph = SocAdvogato()
	
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
        graph_name="SocAdvogato",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
