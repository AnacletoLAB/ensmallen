"""
This file offers the methods to automatically retrieve the graph GOCAMs.

The graph is automatically retrieved from the KGHub repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-12 11:06:11.886458

The undirected multigraph GOCAMs has 3681 nodes with a single node type:
biolink:NamedThing (nodes number 3681), of which 47 are singletons (all
have self-loops), and 3598 unweighted edges with 10 different edge types:
 the 5 most common are biolink:related_to, biolink:negatively_regulates_process_to_process,
biolink:part_of, biolink:enabled_by and biolink:has_input, of which 234
are self-loops and 31 are parallel. The graph is quite sparse as it has
a density of 0.00051 and has 766 connected components, where the component
with most nodes has 510 nodes and the component with the least nodes has
a single node. The graph median node degree is 1, the mean node degree
is 1.89, and the node degree mode is 1. The top 5 most central nodes are
REACT:R-HSA-68819 (degree 34), MGI:MGI:95809 (degree 25), REACT:R-HSA-5693527
(degree 21), REACT:R-HSA-2537512 (degree 18) and REACT:R-HSA-1442481 (degree
18).


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
    from ensmallen_graph.datasets.kghub import GOCAMs

    # Then load the graph
    graph = GOCAMs()

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


def GOCAMs(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/kghub",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the GOCAMs graph.

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
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of GOCAMs graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-12 11:06:11.886458
	
	The undirected multigraph GOCAMs has 3681 nodes with a single node type:
	biolink:NamedThing (nodes number 3681), of which 47 are singletons (all
	have self-loops), and 3598 unweighted edges with 10 different edge types:
	 the 5 most common are biolink:related_to, biolink:negatively_regulates_process_to_process,
	biolink:part_of, biolink:enabled_by and biolink:has_input, of which 234
	are self-loops and 31 are parallel. The graph is quite sparse as it has
	a density of 0.00051 and has 766 connected components, where the component
	with most nodes has 510 nodes and the component with the least nodes has
	a single node. The graph median node degree is 1, the mean node degree
	is 1.89, and the node degree mode is 1. The top 5 most central nodes are
	REACT:R-HSA-68819 (degree 34), MGI:MGI:95809 (degree 25), REACT:R-HSA-5693527
	(degree 21), REACT:R-HSA-2537512 (degree 18) and REACT:R-HSA-1442481 (degree
	18).
	

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
	    from ensmallen_graph.datasets.kghub import GOCAMs
	
	    # Then load the graph
	    graph = GOCAMs()
	
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
        graph_name="GOCAMs",
        dataset="kghub",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
