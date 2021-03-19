"""
This file offers the methods to automatically retrieve the graph web-google-dir.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:33:00.190376

The undirected graph web-google-dir has 875713 nodes and 4322051 weighted
edges, of which none are self-loops. The graph is extremely sparse as it
has a density of 0.00001 and has 2746 connected components, where the component
with most nodes has 855802 nodes and the component with the least nodes
has 2 nodes. The graph median node degree is 5, the mean node degree is
9.87, and the node degree mode is 1. The top 5 most central nodes are 3179
(degree 6332), 116 (degree 5356), 1182 (degree 5273), 13084 (degree 5192)
and 3151 (degree 5100).


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

@article{leskovec2009community,
        title={Community structure in large networks: Natural cluster sizes and the absence of large well-defined clusters},
        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
        journal={Internet Mathematics},
        volume={6},
        number={1},
        pages={29--123},
        year={2009},
        publisher={Taylor \& Francis}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebGoogleDir

    # Then load the graph
    graph = WebGoogleDir()

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


def WebGoogleDir(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the web-google-dir graph.

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
    Instace of web-google-dir graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:33:00.190376
	
	The undirected graph web-google-dir has 875713 nodes and 4322051 weighted
	edges, of which none are self-loops. The graph is extremely sparse as it
	has a density of 0.00001 and has 2746 connected components, where the component
	with most nodes has 855802 nodes and the component with the least nodes
	has 2 nodes. The graph median node degree is 5, the mean node degree is
	9.87, and the node degree mode is 1. The top 5 most central nodes are 3179
	(degree 6332), 116 (degree 5356), 1182 (degree 5273), 13084 (degree 5192)
	and 3151 (degree 5100).
	

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
	
	@article{leskovec2009community,
	        title={Community structure in large networks: Natural cluster sizes and the absence of large well-defined clusters},
	        author={Leskovec, Jure and Lang, Kevin J and Dasgupta, Anirban and Mahoney, Michael W},
	        journal={Internet Mathematics},
	        volume={6},
	        number={1},
	        pages={29--123},
	        year={2009},
	        publisher={Taylor \& Francis}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import WebGoogleDir
	
	    # Then load the graph
	    graph = WebGoogleDir()
	
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
        graph_name="WebGoogleDir",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
