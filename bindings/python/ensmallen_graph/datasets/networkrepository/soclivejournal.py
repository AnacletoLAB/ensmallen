"""
This file offers the methods to automatically retrieve the graph soc-livejournal.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 16:11:54.327073

The undirected graph soc-livejournal has 4033137 nodes and 27933062 unweighted edges, of which none are self-loops. The graph is extremely sparse as it has a density of 0.00000 and is connected, as it has a single component. The graph median node degree is 5, the mean node degree is 13.85, and the node degree mode is 1. The top 5 most central nodes are 7929 (degree 2651), 434686 (degree 1527), 222412 (degree 1242), 43471 (degree 833) and 576327 (degree 796).


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

@inproceedings{mislove-2007-socialnetworks,
        author = {Alan Mislove and Massimiliano Marcon and Krishna P. Gummadi and Peter Druschel and Bobby Bhattacharjee},
        title = {{Measurement and Analysis of Online Social Networks}},
        booktitle = {Proceedings of the 5th ACM/Usenix Internet Measurement Conference (IMC'07)},
        address = {San Diego, CA},
        month = {October},
        year = {2007}}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocLivejournal

    # Then load the graph
    graph = SocLivejournal()

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


def SocLivejournal(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-livejournal graph.

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
    Instace of soc-livejournal graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 16:11:54.327073

The undirected graph soc-livejournal has 4033137 nodes and 27933062 unweighted edges, of which none are self-loops. The graph is extremely sparse as it has a density of 0.00000 and is connected, as it has a single component. The graph median node degree is 5, the mean node degree is 13.85, and the node degree mode is 1. The top 5 most central nodes are 7929 (degree 2651), 434686 (degree 1527), 222412 (degree 1242), 43471 (degree 833) and 576327 (degree 796).


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

@inproceedings{mislove-2007-socialnetworks,
        author = {Alan Mislove and Massimiliano Marcon and Krishna P. Gummadi and Peter Druschel and Bobby Bhattacharjee},
        title = {{Measurement and Analysis of Online Social Networks}},
        booktitle = {Proceedings of the 5th ACM/Usenix Internet Measurement Conference (IMC'07)},
        address = {San Diego, CA},
        month = {October},
        year = {2007}}


    Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocLivejournal

    # Then load the graph
    graph = SocLivejournal()

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
        "SocLivejournal",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
