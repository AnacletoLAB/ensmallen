"""
This file offers the methods to automatically retrieve the graph ER-MD.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 09:06:53.999659

The undirected graph ER-MD has 9512 nodes with 10 different node types:  the 5 most common are 1 (nodes number 8013), 2 (nodes number 1075), 4 (nodes number 175), 3 (nodes number 171) and 6 (nodes number 31) and 104741 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00232 and has 446 connected components, where the component with most nodes has 43 nodes and the component with the least nodes has 4 nodes. The graph median node degree is 21, the mean node degree is 22.02 and the node degree mode is 22. The top 5 most central nodes are 5588 (degree 42), 5587 (degree 42), 5586 (degree 42), 5585 (degree 42) and 5584 (degree 42).


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
    from ensmallen_graph.datasets.networkrepository import ErMd

    # Then load the graph
    graph = ErMd()

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


def ErMd(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the ER-MD graph.

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
    Instace of ER-MD graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 09:06:53.999659

The undirected graph ER-MD has 9512 nodes with 10 different node types:  the 5 most common are 1 (nodes number 8013), 2 (nodes number 1075), 4 (nodes number 175), 3 (nodes number 171) and 6 (nodes number 31) and 104741 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00232 and has 446 connected components, where the component with most nodes has 43 nodes and the component with the least nodes has 4 nodes. The graph median node degree is 21, the mean node degree is 22.02 and the node degree mode is 22. The top 5 most central nodes are 5588 (degree 42), 5587 (degree 42), 5586 (degree 42), 5585 (degree 42) and 5584 (degree 42).


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
    from ensmallen_graph.datasets.networkrepository import ErMd

    # Then load the graph
    graph = ErMd()

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
        "ErMd",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
