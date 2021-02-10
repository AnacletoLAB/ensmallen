"""
This file offers the methods to automatically retrieve the graph tech-as-skitter.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 15:47:19.769919

The undirected graph tech-as-skitter has 1696415 nodes and 11095298 unweighted edges, of which none are self-loops. The graph is extremely sparse as it has a density of 0.00001 and has 756 connected components, where the component with most nodes has 1694616 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 5, the mean node degree is 13.08, and the node degree mode is 2. The top 5 most central nodes are 7047 (degree 35455), 1040 (degree 33982), 7580 (degree 33376), 7589 (degree 30714) and 7041 (degree 30056).


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

@misc{skitter,
        title={Skitter},
        author={{CAIDA}},
        note={{\scriptsize \url{http://caida.org/tools/measurement/skitter/}}}}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import TechAsSkitter

    # Then load the graph
    graph = TechAsSkitter()

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


def TechAsSkitter(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the tech-as-skitter graph.

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
    Instace of tech-as-skitter graph.

    Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-07 15:47:19.769919

The undirected graph tech-as-skitter has 1696415 nodes and 11095298 unweighted edges, of which none are self-loops. The graph is extremely sparse as it has a density of 0.00001 and has 756 connected components, where the component with most nodes has 1694616 nodes and the component with the least nodes has 2 nodes. The graph median node degree is 5, the mean node degree is 13.08, and the node degree mode is 2. The top 5 most central nodes are 7047 (degree 35455), 1040 (degree 33982), 7580 (degree 33376), 7589 (degree 30714) and 7041 (degree 30056).


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

@misc{skitter,
        title={Skitter},
        author={{CAIDA}},
        note={{\scriptsize \url{http://caida.org/tools/measurement/skitter/}}}}


    Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import TechAsSkitter

    # Then load the graph
    graph = TechAsSkitter()

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
        "TechAsSkitter",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()
