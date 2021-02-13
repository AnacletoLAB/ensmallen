"""
This file offers the methods to automatically retrieve the graph soc-twitter-mpi-sws.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 12:23:08.257147

The undirected graph soc-twitter-mpi-sws has 41652230 nodes and 1202513344
unweighted edges, of which 298 are self-loops. The graph is extremely sparse
as it has a density of 0.00000 and is connected, as it has a single component.
The graph median node degree is 12, the mean node degree is 57.74, and
the node degree mode is 4. The top 5 most central nodes are 1037948 (degree
2997487), 1803885 (degree 2696902), 5925043 (degree 2679644), 5874844 (degree
2450753) and 1829999 (degree 1994926).


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

@inproceedings{icwsm10cha,
        author = {Meeyoung Cha and Hamed Haddadi and Fabricio Benevenuto and Krishna P. Gummadi},
        title = {Measuring User Influence in Twitter: The Million Follower Fallacy},
        booktitle = {ICWSM},
        month = {May},
        year = {2010},
        address = {Washington DC, USA}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocTwitterMpiSws

    # Then load the graph
    graph = SocTwitterMpiSws()

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


def SocTwitterMpiSws(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-twitter-mpi-sws graph.

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
    Instace of soc-twitter-mpi-sws graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 12:23:08.257147
	
	The undirected graph soc-twitter-mpi-sws has 41652230 nodes and 1202513344
	unweighted edges, of which 298 are self-loops. The graph is extremely sparse
	as it has a density of 0.00000 and is connected, as it has a single component.
	The graph median node degree is 12, the mean node degree is 57.74, and
	the node degree mode is 4. The top 5 most central nodes are 1037948 (degree
	2997487), 1803885 (degree 2696902), 5925043 (degree 2679644), 5874844 (degree
	2450753) and 1829999 (degree 1994926).
	

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
	
	@inproceedings{icwsm10cha,
	        author = {Meeyoung Cha and Hamed Haddadi and Fabricio Benevenuto and Krishna P. Gummadi},
	        title = {Measuring User Influence in Twitter: The Million Follower Fallacy},
	        booktitle = {ICWSM},
	        month = {May},
	        year = {2010},
	        address = {Washington DC, USA}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocTwitterMpiSws
	
	    # Then load the graph
	    graph = SocTwitterMpiSws()
	
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
        "SocTwitterMpiSws",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[],
        dataset="networkrepository"
    )()
