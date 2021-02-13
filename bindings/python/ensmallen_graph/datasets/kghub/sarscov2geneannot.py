"""
This file offers the methods to automatically retrieve the graph SARSCOV2GeneAnnot.

The graph is automatically retrieved from the KGHub repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-12 11:08:13.317930

The undirected graph SARSCOV2GeneAnnot has 2528 nodes with 2 different
node types: biolink:Protein (nodes number 2403) and biolink:OntologyClass
(nodes number 125), of which 6 are singletons, and 46150 unweighted edges
with 3 different edge types: biolink:enables, biolink:involved_in and biolink:part_of,
of which none are self-loops. The graph is dense as it has a density of
0.01445 and has 7 connected components, where the component with most nodes
has 2522 nodes and the component with the least nodes has a single node.
The graph median node degree is 13, the mean node degree is 36.51, and
the node degree mode is 1. The top 5 most central nodes are GO:0016020
(degree 1411), GO:0016021 (degree 1381), GO:0033644 (degree 1305), GO:0016032
(degree 1148) and GO:0003723 (degree 1048).


References
---------------------
Please cite the following if you use the data:

@article{reese2021kg,
  title={KG-COVID-19: a framework to produce customized knowledge graphs for COVID-19 response},
  author={Reese, Justin T and Unni, Deepak and Callahan, Tiffany J and Cappelletti, Luca and Ravanmehr, Vida and Carbon, Seth and Shefchek, Kent A and Good, Benjamin M and Balhoff, James P and Fontana, Tommaso and others},
  journal={Patterns},
  volume={2},
  number={1},
  pages={100155},
  year={2021},
  publisher={Elsevier}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.kghub import SARSCOV2GeneAnnot

    # Then load the graph
    graph = SARSCOV2GeneAnnot()

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


def SARSCOV2GeneAnnot(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/kghub"
) -> EnsmallenGraph:
    """Return new instance of the SARSCOV2GeneAnnot graph.

    The graph is automatically retrieved from the KGHub repository. 

	

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
    Instace of SARSCOV2GeneAnnot graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-12 11:08:13.317930
	
	The undirected graph SARSCOV2GeneAnnot has 2528 nodes with 2 different
	node types: biolink:Protein (nodes number 2403) and biolink:OntologyClass
	(nodes number 125), of which 6 are singletons, and 46150 unweighted edges
	with 3 different edge types: biolink:enables, biolink:involved_in and biolink:part_of,
	of which none are self-loops. The graph is dense as it has a density of
	0.01445 and has 7 connected components, where the component with most nodes
	has 2522 nodes and the component with the least nodes has a single node.
	The graph median node degree is 13, the mean node degree is 36.51, and
	the node degree mode is 1. The top 5 most central nodes are GO:0016020
	(degree 1411), GO:0016021 (degree 1381), GO:0033644 (degree 1305), GO:0016032
	(degree 1148) and GO:0003723 (degree 1048).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@article{reese2021kg,
	  title={KG-COVID-19: a framework to produce customized knowledge graphs for COVID-19 response},
	  author={Reese, Justin T and Unni, Deepak and Callahan, Tiffany J and Cappelletti, Luca and Ravanmehr, Vida and Carbon, Seth and Shefchek, Kent A and Good, Benjamin M and Balhoff, James P and Fontana, Tommaso and others},
	  journal={Patterns},
	  volume={2},
	  number={1},
	  pages={100155},
	  year={2021},
	  publisher={Elsevier}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.kghub import SARSCOV2GeneAnnot
	
	    # Then load the graph
	    graph = SARSCOV2GeneAnnot()
	
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
        "SARSCOV2GeneAnnot",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="kghub"
    )()
