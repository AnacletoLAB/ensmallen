"""
This file offers the methods to automatically retrieve the graph soc-twitter-2010.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 11:19:24.013907

The undirected graph soc-twitter-2010 has 21297772 nodes and 265025809 weighted edges, of which 264 are self-loops. The graph is extremely sparse as it has a density of 0.00000 and is connected, as it has a single component. The graph median node degree is 3, the mean node degree is 24.89, and the node degree mode is 1. The top 5 most central nodes are 10119001 (degree 698112), 11977289 (degree 442427), 11977292 (degree 418083), 11977350 (degree 384210) and 11980205 (degree 358469).


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

@inproceedings{BoVWFI,
        author ={Paolo Boldi and Sebastiano Vigna},
        title = {The {W}eb{G}raph Framework {I}: {C}ompression Techniques},
        year = {2004},
        booktitle= {Proc. of the Thirteenth International World Wide Web Conference (WWW 2004)},
        address={Manhattan, USA},
        pages={595--601},
        publisher={ACM Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocTwitter2010

    # Then load the graph
    graph = SocTwitter2010()

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


def SocTwitter2010(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-twitter-2010 graph.

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
    Instace of soc-twitter-2010 graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 11:19:24.013907

The undirected graph soc-twitter-2010 has 21297772 nodes and 265025809 weighted edges, of which 264 are self-loops. The graph is extremely sparse as it has a density of 0.00000 and is connected, as it has a single component. The graph median node degree is 3, the mean node degree is 24.89, and the node degree mode is 1. The top 5 most central nodes are 10119001 (degree 698112), 11977289 (degree 442427), 11977292 (degree 418083), 11977350 (degree 384210) and 11980205 (degree 358469).


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

@inproceedings{BoVWFI,
        author ={Paolo Boldi and Sebastiano Vigna},
        title = {The {W}eb{G}raph Framework {I}: {C}ompression Techniques},
        year = {2004},
        booktitle= {Proc. of the Thirteenth International World Wide Web Conference (WWW 2004)},
        address={Manhattan, USA},
        pages={595--601},
        publisher={ACM Press}
}


    Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocTwitter2010

    # Then load the graph
    graph = SocTwitter2010()

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
        "SocTwitter2010",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
