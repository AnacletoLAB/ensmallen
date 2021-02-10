"""
This file offers the methods to automatically retrieve the graph Rhodobacter capsulatus.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 20:17:10.250251

The undirected graph Rhodobacter capsulatus has 3481 nodes and 279489 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.04614 and has 6 connected components, where the component with most nodes has 3468 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 125, the mean node degree is 160.58, and the node degree mode is 9. The top 5 most central nodes are 272942.RCAP_rcc02480 (degree 1452), 272942.RCAP_rcc01728 (degree 1262), 272942.RCAP_rcc01915 (degree 1071), 272942.RCAP_rcc01858 (degree 972) and 272942.RCAP_rcc00161 (degree 966).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import RhodobacterCapsulatus

    # Then load the graph
    graph = RhodobacterCapsulatus()

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


def RhodobacterCapsulatus(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Rhodobacter capsulatus graph.

    The graph is automatically retrieved from the STRING repository. 

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
    Instace of Rhodobacter capsulatus graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 20:17:10.250251

The undirected graph Rhodobacter capsulatus has 3481 nodes and 279489 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.04614 and has 6 connected components, where the component with most nodes has 3468 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 125, the mean node degree is 160.58, and the node degree mode is 9. The top 5 most central nodes are 272942.RCAP_rcc02480 (degree 1452), 272942.RCAP_rcc01728 (degree 1262), 272942.RCAP_rcc01915 (degree 1071), 272942.RCAP_rcc01858 (degree 972) and 272942.RCAP_rcc00161 (degree 966).


    References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


    Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import RhodobacterCapsulatus

    # Then load the graph
    graph = RhodobacterCapsulatus()

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
        "RhodobacterCapsulatus",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
