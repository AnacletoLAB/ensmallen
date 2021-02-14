"""
This file offers the methods to automatically retrieve the graph soc-student-coop.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 09:51:27.912695

The undirected multigraph soc-student-coop has 185 nodes and 360 unweighted
edges with 3 different edge types: 1, 2 and 3, of which none are self-loops
and 98 are parallel. The graph is dense as it has a density of 0.01827
and has 12 connected components, where the component with most nodes has
141 nodes and the component with the least nodes has 2 nodes. The graph
median node degree is 4, the mean node degree is 3.89, and the node degree
mode is 3. The top 5 most central nodes are 117 (degree 10), 102 (degree
10), 15 (degree 9), 159 (degree 9) and 27 (degree 9).


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

@article{Fire2012,
        title={Predicting Student Exam's Scores by Analyzing Social Network Data},
        author={Michael Fire, Gilad Katz, Yuval Elovici, Bracha Shapira, and Lior Rokach},
        booktitle={Active Media Technology},
        pages={584--595},
        year={2012},
        publisher={Springer}	}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocStudentCoop

    # Then load the graph
    graph = SocStudentCoop()

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


def SocStudentCoop(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the soc-student-coop graph.

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
    Instace of soc-student-coop graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 09:51:27.912695
	
	The undirected multigraph soc-student-coop has 185 nodes and 360 unweighted
	edges with 3 different edge types: 1, 2 and 3, of which none are self-loops
	and 98 are parallel. The graph is dense as it has a density of 0.01827
	and has 12 connected components, where the component with most nodes has
	141 nodes and the component with the least nodes has 2 nodes. The graph
	median node degree is 4, the mean node degree is 3.89, and the node degree
	mode is 3. The top 5 most central nodes are 117 (degree 10), 102 (degree
	10), 15 (degree 9), 159 (degree 9) and 27 (degree 9).
	

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
	
	@article{Fire2012,
	        title={Predicting Student Exam's Scores by Analyzing Social Network Data},
	        author={Michael Fire, Gilad Katz, Yuval Elovici, Bracha Shapira, and Lior Rokach},
	        booktitle={Active Media Technology},
	        pages={584--595},
	        year={2012},
	        publisher={Springer}	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocStudentCoop
	
	    # Then load the graph
	    graph = SocStudentCoop()
	
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
        graph_name="SocStudentCoop",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
