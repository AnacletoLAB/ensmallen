"""
This file offers the methods to automatically retrieve the graph inf-USAir97.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 07:46:07.623066

The undirected graph inf-USAir97 has 332 nodes and 2126 weighted edges,
of which none are self-loops. The graph is dense as it has a density of
0.03869 and is connected, as it has a single component. The graph median
node degree is 5, the mean node degree is 12.81 and the node degree mode
is 1. The top 5 most central nodes are 118 (degree 139), 261 (degree 118),
255 (degree 101), 182 (degree 94) and 152 (degree 94).


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

@article{colizza2007reaction,
        title={Reaction--diffusion processes and metapopulation models in heterogeneous networks},
        author={Colizza, Vittoria and Pastor-Satorras, Romualdo and Vespignani, Alessandro},
        journal={Nature Physics},
        volume={3},
        number={4},
        pages={276--282},
        year={2007},
        publisher={Nature Publishing Group}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import InfUsair97

    # Then load the graph
    graph = InfUsair97()

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


def InfUsair97(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the inf-USAir97 graph.

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
    Instace of inf-USAir97 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 07:46:07.623066
	
	The undirected graph inf-USAir97 has 332 nodes and 2126 weighted edges,
	of which none are self-loops. The graph is dense as it has a density of
	0.03869 and is connected, as it has a single component. The graph median
	node degree is 5, the mean node degree is 12.81 and the node degree mode
	is 1. The top 5 most central nodes are 118 (degree 139), 261 (degree 118),
	255 (degree 101), 182 (degree 94) and 152 (degree 94).
	

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
	
	@article{colizza2007reaction,
	        title={Reaction--diffusion processes and metapopulation models in heterogeneous networks},
	        author={Colizza, Vittoria and Pastor-Satorras, Romualdo and Vespignani, Alessandro},
	        journal={Nature Physics},
	        volume={3},
	        number={4},
	        pages={276--282},
	        year={2007},
	        publisher={Nature Publishing Group}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import InfUsair97
	
	    # Then load the graph
	    graph = InfUsair97()
	
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
        graph_name="InfUsair97",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
