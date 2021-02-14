"""
This file offers the methods to automatically retrieve the graph socfb-UIllinois20.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:32:01.366381

The undirected graph socfb-UIllinois20 has 30809 nodes and 1264428 unweighted
edges, of which none are self-loops. The graph is sparse as it has a density
of 0.00266 and has 8 connected components, where the component with most
nodes has 30795 nodes and the component with the least nodes has 2 nodes.
The graph median node degree is 62, the mean node degree is 82.08, and
the node degree mode is 1. The top 5 most central nodes are 21491 (degree
4632), 4907 (degree 1487), 1091 (degree 1446), 25512 (degree 1088) and
25084 (degree 1083).


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
    from ensmallen_graph.datasets.networkrepository import SocfbUillinois20

    # Then load the graph
    graph = SocfbUillinois20()

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


def SocfbUillinois20(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the socfb-UIllinois20 graph.

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
    Instace of socfb-UIllinois20 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:32:01.366381
	
	The undirected graph socfb-UIllinois20 has 30809 nodes and 1264428 unweighted
	edges, of which none are self-loops. The graph is sparse as it has a density
	of 0.00266 and has 8 connected components, where the component with most
	nodes has 30795 nodes and the component with the least nodes has 2 nodes.
	The graph median node degree is 62, the mean node degree is 82.08, and
	the node degree mode is 1. The top 5 most central nodes are 21491 (degree
	4632), 4907 (degree 1487), 1091 (degree 1446), 25512 (degree 1088) and
	25084 (degree 1083).
	

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
	    from ensmallen_graph.datasets.networkrepository import SocfbUillinois20
	
	    # Then load the graph
	    graph = SocfbUillinois20()
	
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
        graph_name="SocfbUillinois20",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
