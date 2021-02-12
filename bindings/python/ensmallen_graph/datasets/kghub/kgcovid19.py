"""
This file offers the methods to automatically retrieve the graph KGCOVID19.

The graph is automatically retrieved from the KGHub repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-12 11:10:25.244690

The undirected multigraph KGCOVID19 has 444887 nodes with 44 different
node types:  the 5 most common are biolink:Publication (nodes number 190048),
biolink:OntologyClass (nodes number 52013), biolink:Drug (nodes number
36151), biolink:Disease (nodes number 24409) and biolink:BiologicalProcess
(nodes number 23310), of which 9256 are singletons (41 of these have self-loops),
and 17944003 unweighted edges with 34 different edge types:  the 5 most
common are biolink:related_to, biolink:interacts_with, biolink:subclass_of,
biolink:part_of and biolink:enables, of which 504 are self-loops and 2754
are parallel. The graph is quite sparse as it has a density of 0.00018
and has 9976 connected components, where the component with most nodes
has 431964 nodes and the component with the least nodes has a single node.
The graph median node degree is 9, the mean node degree is 80.67, and the
node degree mode is 1. The top 5 most central nodes are MESH:D014780 (degree
122238), MESH:D006801 (degree 104307), MESH:D018352 (degree 92928), WD:Q30
(degree 92141) and NCBITaxon:2697049 (degree 91006).


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
    from ensmallen_graph.datasets.kghub import KGCOVID19

    # Then load the graph
    graph = KGCOVID19()

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


def KGCOVID19(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/kghub"
) -> EnsmallenGraph:
    """Return new instance of the KGCOVID19 graph.

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
    Instace of KGCOVID19 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-12 11:10:25.244690
	
	The undirected multigraph KGCOVID19 has 444887 nodes with 44 different
	node types:  the 5 most common are biolink:Publication (nodes number 190048),
	biolink:OntologyClass (nodes number 52013), biolink:Drug (nodes number
	36151), biolink:Disease (nodes number 24409) and biolink:BiologicalProcess
	(nodes number 23310), of which 9256 are singletons (41 of these have self-loops),
	and 17944003 unweighted edges with 34 different edge types:  the 5 most
	common are biolink:related_to, biolink:interacts_with, biolink:subclass_of,
	biolink:part_of and biolink:enables, of which 504 are self-loops and 2754
	are parallel. The graph is quite sparse as it has a density of 0.00018
	and has 9976 connected components, where the component with most nodes
	has 431964 nodes and the component with the least nodes has a single node.
	The graph median node degree is 9, the mean node degree is 80.67, and the
	node degree mode is 1. The top 5 most central nodes are MESH:D014780 (degree
	122238), MESH:D006801 (degree 104307), MESH:D018352 (degree 92928), WD:Q30
	(degree 92141) and NCBITaxon:2697049 (degree 91006).
	

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
	    from ensmallen_graph.datasets.kghub import KGCOVID19
	
	    # Then load the graph
	    graph = KGCOVID19()
	
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
        "KGCOVID19",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kghub"
    )()
