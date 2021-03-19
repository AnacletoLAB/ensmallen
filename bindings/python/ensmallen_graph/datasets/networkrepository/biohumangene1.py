"""
This file offers the methods to automatically retrieve the graph bio-human-gene1.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:33:29.803249

The undirected graph bio-human-gene1 has 22283 nodes, of which 393 are
singletons (all have self-loops), and 12345963 weighted edges, of which
22283 are self-loops. The graph is dense as it has a density of 0.04969
and has 410 connected components, where the component with most nodes has
21853 nodes and the component with the least nodes has a single node. The
graph median node degree is 537, the mean node degree is 1107.11 and the
node degree mode is 1. The top 5 most central nodes are 20435 (degree 7939),
15280 (degree 7724), 7206 (degree 7587), 8022 (degree 7579) and 16204 (degree
7571).


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

@article{bansal2007infer,
        title={How to infer gene networks from expression profiles},
        author={Bansal, Mukesh and Belcastro, Vincenzo and Ambesi-Impiombato, Alberto and Di Bernardo, Diego},
        journal={Molecular systems biology},
        volume={3},
        number={1},
        year={2007},
        publisher={Wiley Online Library}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import BioHumanGene1

    # Then load the graph
    graph = BioHumanGene1()

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


def BioHumanGene1(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the bio-human-gene1 graph.

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
    Instace of bio-human-gene1 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:33:29.803249
	
	The undirected graph bio-human-gene1 has 22283 nodes, of which 393 are
	singletons (all have self-loops), and 12345963 weighted edges, of which
	22283 are self-loops. The graph is dense as it has a density of 0.04969
	and has 410 connected components, where the component with most nodes has
	21853 nodes and the component with the least nodes has a single node. The
	graph median node degree is 537, the mean node degree is 1107.11 and the
	node degree mode is 1. The top 5 most central nodes are 20435 (degree 7939),
	15280 (degree 7724), 7206 (degree 7587), 8022 (degree 7579) and 16204 (degree
	7571).
	

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
	
	@article{bansal2007infer,
	        title={How to infer gene networks from expression profiles},
	        author={Bansal, Mukesh and Belcastro, Vincenzo and Ambesi-Impiombato, Alberto and Di Bernardo, Diego},
	        journal={Molecular systems biology},
	        volume={3},
	        number={1},
	        year={2007},
	        publisher={Wiley Online Library}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import BioHumanGene1
	
	    # Then load the graph
	    graph = BioHumanGene1()
	
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
        graph_name="BioHumanGene1",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
