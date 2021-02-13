"""
This file offers the methods to automatically retrieve the graph IntAct.

The graph is automatically retrieved from the KGHub repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-12 11:08:10.432483

The undirected graph IntAct has 3058 nodes with 4 different node types:
biolink:Protein (nodes number 2945), biolink:Drug (nodes number 82), biolink:RNA
(nodes number 28) and biolink:MolecularEntity (nodes number 3), of which
2265 are singletons (2 of these have self-loops), and 1167 unweighted edges
with a single edge type: biolink:interacts_with, of which 35 are self-loops.
The graph is quite sparse as it has a density of 0.00025 and has 2310 connected
components, where the component with most nodes has 603 nodes and the component
with the least nodes has a single node. The graph median node degree is
0, the mean node degree is 0.75, and the node degree mode is 0. The top
5 most central nodes are UniProtKB:P0DTC2 (degree 59), UniProtKB:Q14160
(degree 53), UniProtKB:Q12959 (degree 53), UniProtKB:P0C6X7-PRO_0000037311
(degree 34) and UniProtKB:Q9BYF1 (degree 31).


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
    from ensmallen_graph.datasets.kghub import IntAct

    # Then load the graph
    graph = IntAct()

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


def IntAct(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/kghub"
) -> EnsmallenGraph:
    """Return new instance of the IntAct graph.

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
    Instace of IntAct graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-12 11:08:10.432483
	
	The undirected graph IntAct has 3058 nodes with 4 different node types:
	biolink:Protein (nodes number 2945), biolink:Drug (nodes number 82), biolink:RNA
	(nodes number 28) and biolink:MolecularEntity (nodes number 3), of which
	2265 are singletons (2 of these have self-loops), and 1167 unweighted edges
	with a single edge type: biolink:interacts_with, of which 35 are self-loops.
	The graph is quite sparse as it has a density of 0.00025 and has 2310 connected
	components, where the component with most nodes has 603 nodes and the component
	with the least nodes has a single node. The graph median node degree is
	0, the mean node degree is 0.75, and the node degree mode is 0. The top
	5 most central nodes are UniProtKB:P0DTC2 (degree 59), UniProtKB:Q14160
	(degree 53), UniProtKB:Q12959 (degree 53), UniProtKB:P0C6X7-PRO_0000037311
	(degree 34) and UniProtKB:Q9BYF1 (degree 31).
	

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
	    from ensmallen_graph.datasets.kghub import IntAct
	
	    # Then load the graph
	    graph = IntAct()
	
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
        "IntAct",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="kghub"
    )()
