"""
This file offers the methods to automatically retrieve the graph soc-BlogCatalog-ASU.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 08:50:39.241812

The undirected graph soc-BlogCatalog-ASU has 10312 nodes with 38 different node types:  the 5 most common are 8 (nodes number 1423), 5 (nodes number 843), 2 (nodes number 759), 6 (nodes number 735) and 24 (nodes number 586) and 333983 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00628 and is connected, as it has a single component. The graph median node degree is 21, the mean node degree is 64.78, and the node degree mode is 2. The top 5 most central nodes are 4839 (degree 3992), 176 (degree 3925), 4374 (degree 3449), 8157 (degree 2976) and 1226 (degree 2780).


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
    from ensmallen_graph.datasets.networkrepository import SocBlogcatalogAsu

    # Then load the graph
    graph = SocBlogcatalogAsu()

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


def SocBlogcatalogAsu(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-BlogCatalog-ASU graph.

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
    Instace of soc-BlogCatalog-ASU graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 08:50:39.241812

The undirected graph soc-BlogCatalog-ASU has 10312 nodes with 38 different node types:  the 5 most common are 8 (nodes number 1423), 5 (nodes number 843), 2 (nodes number 759), 6 (nodes number 735) and 24 (nodes number 586) and 333983 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00628 and is connected, as it has a single component. The graph median node degree is 21, the mean node degree is 64.78, and the node degree mode is 2. The top 5 most central nodes are 4839 (degree 3992), 176 (degree 3925), 4374 (degree 3449), 8157 (degree 2976) and 1226 (degree 2780).


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
    from ensmallen_graph.datasets.networkrepository import SocBlogcatalogAsu

    # Then load the graph
    graph = SocBlogcatalogAsu()

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
        "SocBlogcatalogAsu",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
