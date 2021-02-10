"""
This file offers the methods to automatically retrieve the graph web-edu.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 09:52:21.089562

The undirected graph web-edu has 3031 nodes and 6474 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00141 and is connected, as it has a single component. The graph median node degree is 3, the mean node degree is 4.27, and the node degree mode is 3. The top 5 most central nodes are 2818 (degree 104), 342 (degree 84), 294 (degree 73), 1575 (degree 72) and 1849 (degree 66).


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

@article{gleich2004fast,
        title={Fast parallel PageRank: A linear system approach},
        author={Gleich, David and Zhukov, Leonid and Berkhin, Pavel},
        journal={Yahoo! Research Technical Report YRL-2004-038},
        volume={13},
        pages={22},
        year={2004}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebEdu

    # Then load the graph
    graph = WebEdu()

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


def WebEdu(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the web-edu graph.

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
    Instace of web-edu graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 09:52:21.089562

The undirected graph web-edu has 3031 nodes and 6474 unweighted edges, of which none are self-loops. The graph is sparse as it has a density of 0.00141 and is connected, as it has a single component. The graph median node degree is 3, the mean node degree is 4.27, and the node degree mode is 3. The top 5 most central nodes are 2818 (degree 104), 342 (degree 84), 294 (degree 73), 1575 (degree 72) and 1849 (degree 66).


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

@article{gleich2004fast,
        title={Fast parallel PageRank: A linear system approach},
        author={Gleich, David and Zhukov, Leonid and Berkhin, Pavel},
        journal={Yahoo! Research Technical Report YRL-2004-038},
        volume={13},
        pages={22},
        year={2004}
}


    Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import WebEdu

    # Then load the graph
    graph = WebEdu()

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
        "WebEdu",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
