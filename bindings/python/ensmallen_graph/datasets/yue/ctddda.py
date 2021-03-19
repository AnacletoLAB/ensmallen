"""
This file offers the methods to automatically retrieve the graph CTDDDA.

The graph is automatically retrieved from the Yue repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-03-02 10:32:30.488037

The undirected graph CTDDDA has 12765 nodes with 2 different node types:
chemical (nodes number 9580) and disease (nodes number 3185) and 92813
unweighted edges, of which none are self-loops. The graph is sparse as
it has a density of 0.00114 and has 20 connected components, where the
component with most nodes has 12724 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 3, the mean node degree
is 14.54, and the node degree mode is 1. The top 5 most central nodes are
D056486 (degree 1217), D012640 (degree 1098), D006973 (degree 838), D009336
(degree 831) and D007674 (degree 705). The hash of the graph is d0ea7570f7a904ae
.


References
---------------------
Please cite the following if you use the data:

@article{yue2020graph,
  title={Graph embedding on biomedical networks: methods, applications and evaluations},
  author={Yue, Xiang and Wang, Zhen and Huang, Jingong and Parthasarathy, Srinivasan and Moosavinasab, Soheil and Huang, Yungui and Lin, Simon M and Zhang, Wen and Zhang, Ping and Sun, Huan},
  journal={Bioinformatics},
  volume={36},
  number={4},
  pages={1241--1251},
  year={2020},
  publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.yue import CTDDDA

    # Then load the graph
    graph = CTDDDA()

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


def CTDDDA(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/yue",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the CTDDDA graph.

    The graph is automatically retrieved from the Yue repository. 

	

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
    Instace of CTDDDA graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-03-02 10:32:30.488037
	
	The undirected graph CTDDDA has 12765 nodes with 2 different node types:
	chemical (nodes number 9580) and disease (nodes number 3185) and 92813
	unweighted edges, of which none are self-loops. The graph is sparse as
	it has a density of 0.00114 and has 20 connected components, where the
	component with most nodes has 12724 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 3, the mean node degree
	is 14.54, and the node degree mode is 1. The top 5 most central nodes are
	D056486 (degree 1217), D012640 (degree 1098), D006973 (degree 838), D009336
	(degree 831) and D007674 (degree 705). The hash of the graph is d0ea7570f7a904ae
	.
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@article{yue2020graph,
	  title={Graph embedding on biomedical networks: methods, applications and evaluations},
	  author={Yue, Xiang and Wang, Zhen and Huang, Jingong and Parthasarathy, Srinivasan and Moosavinasab, Soheil and Huang, Yungui and Lin, Simon M and Zhang, Wen and Zhang, Ping and Sun, Huan},
	  journal={Bioinformatics},
	  volume={36},
	  number={4},
	  pages={1241--1251},
	  year={2020},
	  publisher={Oxford University Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.yue import CTDDDA
	
	    # Then load the graph
	    graph = CTDDDA()
	
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
        graph_name="CTDDDA",
        dataset="yue",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
