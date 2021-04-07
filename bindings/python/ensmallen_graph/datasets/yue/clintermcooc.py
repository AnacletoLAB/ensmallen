"""
This file offers the methods to automatically retrieve the graph ClinTermCOOC.

The graph is automatically retrieved from the Yue repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-03-02 10:35:53.865089

The undirected graph ClinTermCOOC has 48651 nodes and 1659248 unweighted
edges, of which none are self-loops. The graph is sparse as it has a density
of 0.00140 and has 25 connected components, where the component with most
nodes has 48601 nodes and the component with the least nodes has 2 nodes.
The graph median node degree is 20, the mean node degree is 68.21, and
the node degree mode is 1. The top 5 most central nodes are 167 (degree
3394), 196 (degree 2978), 488 (degree 2378), 1276 (degree 2084) and 1041
(degree 2055). The hash of the graph is da61007aadfef6b3 .


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
    from ensmallen_graph.datasets.yue import ClinTermCOOC

    # Then load the graph
    graph = ClinTermCOOC()

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


def ClinTermCOOC(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/yue",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the ClinTermCOOC graph.

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
    Instace of ClinTermCOOC graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-03-02 10:35:53.865089
	
	The undirected graph ClinTermCOOC has 48651 nodes and 1659248 unweighted
	edges, of which none are self-loops. The graph is sparse as it has a density
	of 0.00140 and has 25 connected components, where the component with most
	nodes has 48601 nodes and the component with the least nodes has 2 nodes.
	The graph median node degree is 20, the mean node degree is 68.21, and
	the node degree mode is 1. The top 5 most central nodes are 167 (degree
	3394), 196 (degree 2978), 488 (degree 2378), 1276 (degree 2084) and 1041
	(degree 2055). The hash of the graph is da61007aadfef6b3 .
	

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
	    from ensmallen_graph.datasets.yue import ClinTermCOOC
	
	    # Then load the graph
	    graph = ClinTermCOOC()
	
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
        graph_name="ClinTermCOOC",
        dataset="yue",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
