"""
This file offers the methods to automatically retrieve the graph NDFRTDDA.

The graph is automatically retrieved from the Yue repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-12 11:23:22.289475

The undirected graph NDFRTDDA has 13546 nodes with 3 different node types:
drug (nodes number 12337), disease (nodes number 1208) and type (nodes
number 1), of which 1 are singletons, and 56515 unweighted edges, of which
none are self-loops. The graph is quite sparse as it has a density of 0.00062
and has 86 connected components, where the component with most nodes has
13033 nodes and the component with the least nodes has a single node. The
graph median node degree is 3, the mean node degree is 8.34, and the node
degree mode is 1. The top 5 most central nodes are C0014544 (degree 845),
C0003864 (degree 741), C0751861 (degree 653), C0458219 (degree 575) and
C0036982 (degree 534).


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
    from ensmallen_graph.datasets.yue import NDFRTDDA

    # Then load the graph
    graph = NDFRTDDA()

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


def NDFRTDDA(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/yue"
) -> EnsmallenGraph:
    """Return new instance of the NDFRTDDA graph.

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

    Returns
    -----------------------
    Instace of NDFRTDDA graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-12 11:23:22.289475
	
	The undirected graph NDFRTDDA has 13546 nodes with 3 different node types:
	drug (nodes number 12337), disease (nodes number 1208) and type (nodes
	number 1), of which 1 are singletons, and 56515 unweighted edges, of which
	none are self-loops. The graph is quite sparse as it has a density of 0.00062
	and has 86 connected components, where the component with most nodes has
	13033 nodes and the component with the least nodes has a single node. The
	graph median node degree is 3, the mean node degree is 8.34, and the node
	degree mode is 1. The top 5 most central nodes are C0014544 (degree 845),
	C0003864 (degree 741), C0751861 (degree 653), C0458219 (degree 575) and
	C0036982 (degree 534).
	

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
	    from ensmallen_graph.datasets.yue import NDFRTDDA
	
	    # Then load the graph
	    graph = NDFRTDDA()
	
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
        "NDFRTDDA",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="yue"
    )()
