"""
This file offers the methods to automatically retrieve the graph SW-10000-4-0d3-trial3.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 10:40:43.124998

The undirected graph SW-10000-4-0d3-trial3 has 10000 nodes and 20000 unweighted edges, of which none are self-loops. The graph is quite sparse as it has a density of 0.00040 and is connected, as it has a single component. The graph median node degree is 4, the mean node degree is 4.00, and the node degree mode is 4. The top 5 most central nodes are 7721 (degree 9), 4608 (degree 9), 1222 (degree 9), 2186 (degree 9) and 3663 (degree 9).


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
    from ensmallen_graph.datasets.networkrepository import Sw1000040d3Trial3

    # Then load the graph
    graph = Sw1000040d3Trial3()

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


def Sw1000040d3Trial3(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the SW-10000-4-0d3-trial3 graph.

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
    Instace of SW-10000-4-0d3-trial3 graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 10:40:43.124998

The undirected graph SW-10000-4-0d3-trial3 has 10000 nodes and 20000 unweighted edges, of which none are self-loops. The graph is quite sparse as it has a density of 0.00040 and is connected, as it has a single component. The graph median node degree is 4, the mean node degree is 4.00, and the node degree mode is 4. The top 5 most central nodes are 7721 (degree 9), 4608 (degree 9), 1222 (degree 9), 2186 (degree 9) and 3663 (degree 9).


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
    from ensmallen_graph.datasets.networkrepository import Sw1000040d3Trial3

    # Then load the graph
    graph = Sw1000040d3Trial3()

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
        "Sw1000040d3Trial3",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
