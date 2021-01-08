from .ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error

from .automatic_graph_retrieval import AutomaticallyRetrievedGraph


def StringPPI(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the human StringPPI graph (not filtered).

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
        "StringPPI",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()

def CompleteStringPPI(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the Complete (cross-species) StringPPI graph (not filtered).

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
    String PPI graph.
    """
    return AutomaticallyRetrievedGraph(
        "CompleteStringPPI",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()


def KGCOVID19(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the KG-COVID19 graph.

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
        "KG-COVID19",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()
