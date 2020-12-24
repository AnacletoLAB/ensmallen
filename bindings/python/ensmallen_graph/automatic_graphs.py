from .ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error

from .automatic_graph_retrieval import AutomaticallyRetrievedGraph


def StringPPI(
    directed: bool = False,
    verbose: bool = True,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the StringPPI graph.

    The retrieved graph is automatically retrieved directly from the STRING
    website, that is https://string-db.org/.

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: bool = True,
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


def KGCOVID19(
    directed: bool = False,
    verbose: bool = True,
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
    verbose: bool = True,
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
