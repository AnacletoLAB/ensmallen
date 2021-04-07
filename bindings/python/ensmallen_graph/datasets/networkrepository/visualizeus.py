"""
This file offers the methods to automatically retrieve the graph visualize-us.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 07:45:14.828036

The undirected graph visualize-us has 594559 nodes with 3 different node
types: 2 (nodes number 495402), 3 (nodes number 82035) and 1 (nodes number
17122) and 3247673 unweighted edges with 680 different edge types:  the
5 most common are 1, 2, 3, 4 and 5, of which none are self-loops. The graph
is extremely sparse as it has a density of 0.00002 and has 7 connected
components, where the component with most nodes has 594506 nodes and the
component with the least nodes has 3 nodes. The graph median node degree
is 3, the mean node degree is 10.92 and the node degree mode is 2. The
top 5 most central nodes are 512767 (degree 237308), 512741 (degree 51646),
512613 (degree 34008), 512903 (degree 28437) and 512859 (degree 27288).


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


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import VisualizeUs

    # Then load the graph
    graph = VisualizeUs()

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


def VisualizeUs(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the visualize-us graph.

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
    Instace of visualize-us graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 07:45:14.828036
	
	The undirected graph visualize-us has 594559 nodes with 3 different node
	types: 2 (nodes number 495402), 3 (nodes number 82035) and 1 (nodes number
	17122) and 3247673 unweighted edges with 680 different edge types:  the
	5 most common are 1, 2, 3, 4 and 5, of which none are self-loops. The graph
	is extremely sparse as it has a density of 0.00002 and has 7 connected
	components, where the component with most nodes has 594506 nodes and the
	component with the least nodes has 3 nodes. The graph median node degree
	is 3, the mean node degree is 10.92 and the node degree mode is 2. The
	top 5 most central nodes are 512767 (degree 237308), 512741 (degree 51646),
	512613 (degree 34008), 512903 (degree 28437) and 512859 (degree 27288).
	

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
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import VisualizeUs
	
	    # Then load the graph
	    graph = VisualizeUs()
	
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
        graph_name="VisualizeUs",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
