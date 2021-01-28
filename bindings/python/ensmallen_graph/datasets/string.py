from ..ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error

from .automatic_graph_retrieval import AutomaticallyRetrievedGraph


def HumanString(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the human String graph (not filtered).

    The retrieved graph is automatically retrieved directly from the STRING
    website, that is https://string-db.org/.

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.

    Returns
    -----------------------
    String PPI graph.
    """
    return AutomaticallyRetrievedGraph(
        "HumanString",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()


def CompleteString(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the Complete (cross-species) String graph (not filtered).

    THIS GRAPH IS VERY VERY BIG, ABOUT 43GB!

    The retrieved graph is automatically retrieved directly from the STRING
    website, that is https://string-db.org/.

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.

    Returns
    -----------------------
    String  graph.
    """
    return AutomaticallyRetrievedGraph(
        "CompleteString",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
