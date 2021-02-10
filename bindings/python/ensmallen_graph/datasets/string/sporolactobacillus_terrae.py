"""
This file offers the methods to automatically retrieve the graph Sporolactobacillus terrae.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:01:39.010955

The undirected graph Sporolactobacillus terrae has 2981 nodes and 234647 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.05283 and has 20 connected components, where the component with most nodes has 2940 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 133, the mean node degree is 157.43, and the node degree mode is 1. The top 5 most central nodes are 1444306.JFZC01000040_gene433 (degree 1115), 1444306.JFZC01000101_gene1256 (degree 1018), 1444306.JFZC01000038_gene2852 (degree 808), 1444306.JFZC01000015_gene2149 (degree 790) and 1444306.JFZC01000038_gene2821 (degree 780).


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
    from ensmallen_graph.datasets.string import SporolactobacillusTerrae

    # Then load the graph
    graph = SporolactobacillusTerrae()

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


def SporolactobacillusTerrae(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Sporolactobacillus terrae graph.

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
    Instace of Sporolactobacillus terrae graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:01:39.010955

The undirected graph Sporolactobacillus terrae has 2981 nodes and 234647 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.05283 and has 20 connected components, where the component with most nodes has 2940 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 133, the mean node degree is 157.43, and the node degree mode is 1. The top 5 most central nodes are 1444306.JFZC01000040_gene433 (degree 1115), 1444306.JFZC01000101_gene1256 (degree 1018), 1444306.JFZC01000038_gene2852 (degree 808), 1444306.JFZC01000015_gene2149 (degree 790) and 1444306.JFZC01000038_gene2821 (degree 780).


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
    from ensmallen_graph.datasets.string import SporolactobacillusTerrae

    # Then load the graph
    graph = SporolactobacillusTerrae()

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
        "SporolactobacillusTerrae",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
