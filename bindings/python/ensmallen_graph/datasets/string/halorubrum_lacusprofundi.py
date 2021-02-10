"""
This file offers the methods to automatically retrieve the graph Halorubrum lacusprofundi.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 20:46:46.218775

The undirected graph Halorubrum lacusprofundi has 3513 nodes and 577076 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.09355 and has 19 connected components, where the component with most nodes has 3468 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 342, the mean node degree is 328.54, and the node degree mode is 1. The top 5 most central nodes are 416348.Hlac_3225 (degree 1243), 416348.Hlac_2432 (degree 1177), 416348.Hlac_1683 (degree 1166), 416348.Hlac_0836 (degree 1148) and 416348.Hlac_0682 (degree 981).


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
    from ensmallen_graph.datasets.string import HalorubrumLacusprofundi

    # Then load the graph
    graph = HalorubrumLacusprofundi()

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


def HalorubrumLacusprofundi(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Halorubrum lacusprofundi graph.

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
    Instace of Halorubrum lacusprofundi graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 20:46:46.218775

The undirected graph Halorubrum lacusprofundi has 3513 nodes and 577076 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.09355 and has 19 connected components, where the component with most nodes has 3468 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 342, the mean node degree is 328.54, and the node degree mode is 1. The top 5 most central nodes are 416348.Hlac_3225 (degree 1243), 416348.Hlac_2432 (degree 1177), 416348.Hlac_1683 (degree 1166), 416348.Hlac_0836 (degree 1148) and 416348.Hlac_0682 (degree 981).


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
    from ensmallen_graph.datasets.string import HalorubrumLacusprofundi

    # Then load the graph
    graph = HalorubrumLacusprofundi()

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
        "HalorubrumLacusprofundi",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
