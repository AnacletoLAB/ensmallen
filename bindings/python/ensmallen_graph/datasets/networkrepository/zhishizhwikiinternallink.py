"""
This file offers the methods to automatically retrieve the graph zhishi-zhwiki-internallink.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 15:57:41.801486

The undirected graph zhishi-zhwiki-internallink has 1930275 nodes, of which 5 are singletons (all have self-loops), and 8961954 unweighted edges, of which 5052 are self-loops. The graph is extremely sparse as it has a density of 0.00000 and has 328 connected components, where the component with most nodes has 1929387 nodes and the component with the least nodes has a single node. The graph median node degree is 1, the mean node degree is 9.28, and the node degree mode is 1. The top 5 most central nodes are 64 (degree 29005), 262 (degree 28263), 1464 (degree 27877), 33371 (degree 27597) and 3680 (degree 24209).


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
    from ensmallen_graph.datasets.networkrepository import ZhishiZhwikiInternallink

    # Then load the graph
    graph = ZhishiZhwikiInternallink()

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


def ZhishiZhwikiInternallink(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the zhishi-zhwiki-internallink graph.

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
    Instace of zhishi-zhwiki-internallink graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 15:57:41.801486

The undirected graph zhishi-zhwiki-internallink has 1930275 nodes, of which 5 are singletons (all have self-loops), and 8961954 unweighted edges, of which 5052 are self-loops. The graph is extremely sparse as it has a density of 0.00000 and has 328 connected components, where the component with most nodes has 1929387 nodes and the component with the least nodes has a single node. The graph median node degree is 1, the mean node degree is 9.28, and the node degree mode is 1. The top 5 most central nodes are 64 (degree 29005), 262 (degree 28263), 1464 (degree 27877), 33371 (degree 27597) and 3680 (degree 24209).


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
    from ensmallen_graph.datasets.networkrepository import ZhishiZhwikiInternallink

    # Then load the graph
    graph = ZhishiZhwikiInternallink()

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
        "ZhishiZhwikiInternallink",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
