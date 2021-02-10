"""
This file offers the methods to automatically retrieve the graph Saccharothrix sp. NRRLB16314.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:08:15.699833

The undirected graph Saccharothrix sp. NRRLB16314 has 7809 nodes and 1433089 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.04701 and has 40 connected components, where the component with most nodes has 7718 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 305, the mean node degree is 367.04, and the node degree mode is 2. The top 5 most central nodes are 1463825.JNXC01000017_gene6960 (degree 2599), 1463825.JNXC01000001_gene5547 (degree 2458), 1463825.JNXC01000007_gene805 (degree 2277), 1463825.JNXC01000013_gene7764 (degree 2267) and 1463825.JNXC01000022_gene514 (degree 2251).


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
    from ensmallen_graph.datasets.string import SaccharothrixSp.Nrrlb16314

    # Then load the graph
    graph = SaccharothrixSp.Nrrlb16314()

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


def SaccharothrixSp.Nrrlb16314(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Saccharothrix sp. NRRLB16314 graph.

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
    Instace of Saccharothrix sp. NRRLB16314 graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 23:08:15.699833

The undirected graph Saccharothrix sp. NRRLB16314 has 7809 nodes and 1433089 weighted edges, of which none are self-loops. The graph is dense as it has a density of 0.04701 and has 40 connected components, where the component with most nodes has 7718 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 305, the mean node degree is 367.04, and the node degree mode is 2. The top 5 most central nodes are 1463825.JNXC01000017_gene6960 (degree 2599), 1463825.JNXC01000001_gene5547 (degree 2458), 1463825.JNXC01000007_gene805 (degree 2277), 1463825.JNXC01000013_gene7764 (degree 2267) and 1463825.JNXC01000022_gene514 (degree 2251).


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
    from ensmallen_graph.datasets.string import SaccharothrixSp.Nrrlb16314

    # Then load the graph
    graph = SaccharothrixSp.Nrrlb16314()

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
        "SaccharothrixSp.Nrrlb16314",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
