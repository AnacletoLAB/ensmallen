"""
This file offers the methods to automatically retrieve the graph Veillonella sp. AS16.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 22:00:41.434136

The undirected graph Veillonella sp. AS16 has 1723 nodes and 116808 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.07874 and has 5 connected components, where the component with most nodes has 1713 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 111, the mean node degree is 135.59, and the node degree mode is 55. The top 5 most central nodes are 936589.HMPREF1521_0155 (degree 809), 936589.HMPREF1521_1351 (degree 660), 936589.HMPREF1521_0213 (degree 658), 936589.HMPREF1521_1628 (degree 648) and 936589.HMPREF1521_0432 (degree 629).


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
    from ensmallen_graph.datasets.string import VeillonellaSp.As16

    # Then load the graph
    graph = VeillonellaSp.As16()

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


def VeillonellaSp.As16(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Veillonella sp. AS16 graph.

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
    Instace of Veillonella sp. AS16 graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 22:00:41.434136

The undirected graph Veillonella sp. AS16 has 1723 nodes and 116808 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.07874 and has 5 connected components, where the component with most nodes has 1713 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 111, the mean node degree is 135.59, and the node degree mode is 55. The top 5 most central nodes are 936589.HMPREF1521_0155 (degree 809), 936589.HMPREF1521_1351 (degree 660), 936589.HMPREF1521_0213 (degree 658), 936589.HMPREF1521_1628 (degree 648) and 936589.HMPREF1521_0432 (degree 629).


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
    from ensmallen_graph.datasets.string import VeillonellaSp.As16

    # Then load the graph
    graph = VeillonellaSp.As16()

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
        "VeillonellaSp.As16",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
