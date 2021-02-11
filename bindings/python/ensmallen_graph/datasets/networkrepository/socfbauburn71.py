"""
This file offers the methods to automatically retrieve the graph socfb-Auburn71.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:48:21.326617

The undirected graph socfb-Auburn71 has 18448 nodes and 973918 unweighted
edges, of which none are self-loops. The graph is sparse as it has a density
of 0.00572 and is connected, as it has a single component. The graph median
node degree is 76, the mean node degree is 105.59, and the node degree
mode is 1. The top 5 most central nodes are 5472 (degree 5160), 10626 (degree
3439), 14497 (degree 3267), 12499 (degree 2857) and 7194 (degree 2495).


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

@article{traud2012social,
        title={Social structure of {F}acebook networks},
        author={Traud, Amanda L and Mucha, Peter J and Porter, Mason A},
        journal={Phys. A},
        month={Aug},
        number={16},
        pages={4165--4180},
        volume={391},
        year={2012}
}

@article{Traud:2011fs,
        title={Comparing Community Structure to Characteristics in Online Collegiate Social Networks},
        author={Traud, Amanda L and Kelsic, Eric D and Mucha, Peter J and Porter, Mason A},
        journal={SIAM Rev.},
        number={3},
        pages={526--543},
        volume={53},
        year={2011}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocfbAuburn71

    # Then load the graph
    graph = SocfbAuburn71()

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


def SocfbAuburn71(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the socfb-Auburn71 graph.

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

    Returns
    -----------------------
    Instace of socfb-Auburn71 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 11:48:21.326617
	
	The undirected graph socfb-Auburn71 has 18448 nodes and 973918 unweighted
	edges, of which none are self-loops. The graph is sparse as it has a density
	of 0.00572 and is connected, as it has a single component. The graph median
	node degree is 76, the mean node degree is 105.59, and the node degree
	mode is 1. The top 5 most central nodes are 5472 (degree 5160), 10626 (degree
	3439), 14497 (degree 3267), 12499 (degree 2857) and 7194 (degree 2495).
	

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
	
	@article{traud2012social,
	        title={Social structure of {F}acebook networks},
	        author={Traud, Amanda L and Mucha, Peter J and Porter, Mason A},
	        journal={Phys. A},
	        month={Aug},
	        number={16},
	        pages={4165--4180},
	        volume={391},
	        year={2012}
	}
	
	@article{Traud:2011fs,
	        title={Comparing Community Structure to Characteristics in Online Collegiate Social Networks},
	        author={Traud, Amanda L and Kelsic, Eric D and Mucha, Peter J and Porter, Mason A},
	        journal={SIAM Rev.},
	        number={3},
	        pages={526--543},
	        volume={53},
	        year={2011}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocfbAuburn71
	
	    # Then load the graph
	    graph = SocfbAuburn71()
	
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
        "SocfbAuburn71",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
