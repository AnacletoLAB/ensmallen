"""
This file offers the methods to automatically retrieve the graph pli.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:25:31.846560

The undirected graph pli has 22695 nodes and 686502 unweighted edges, of which 22695 are self-loops. The graph is sparse as it has a density of 0.00262 and has 2 connected components, where the component with most nodes has 18355 nodes and the component with the least nodes has 4340 nodes. The graph median node degree is 53, the mean node degree is 59.50, and the node degree mode is 27. The top 5 most central nodes are 17111 (degree 108), 17101 (degree 108), 17086 (degree 108), 17081 (degree 108) and 17076 (degree 108).


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
    from ensmallen_graph.datasets.networkrepository import Pli

    # Then load the graph
    graph = Pli()

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


def Pli(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the pli graph.

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
    Instace of pli graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 11:25:31.846560

The undirected graph pli has 22695 nodes and 686502 unweighted edges, of which 22695 are self-loops. The graph is sparse as it has a density of 0.00262 and has 2 connected components, where the component with most nodes has 18355 nodes and the component with the least nodes has 4340 nodes. The graph median node degree is 53, the mean node degree is 59.50, and the node degree mode is 27. The top 5 most central nodes are 17111 (degree 108), 17101 (degree 108), 17086 (degree 108), 17081 (degree 108) and 17076 (degree 108).


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
    from ensmallen_graph.datasets.networkrepository import Pli

    # Then load the graph
    graph = Pli()

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
        "Pli",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
