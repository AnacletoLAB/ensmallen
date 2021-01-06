from .automatic_graph_retrieval import AutomaticallyRetrievedGraph
from .ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def StringPPI(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the StringPPI graph used in the Yue paper.

    The retrieved graph is automatically retrieved directly from the GitHub
    repository of the Yue et al paper (https://academic.oup.com/bioinformatics/article/36/4/1241/5581350#),
    that is https://github.com/xiangyue9607/BioNEV/tree/master/data.

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
        "YueStringPPI",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()


def CTDDDA(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the CTDDDA graph used in the Yue paper.

    The retrieved graph is automatically retrieved directly from the GitHub
    repository of the Yue et al paper (https://academic.oup.com/bioinformatics/article/36/4/1241/5581350#),
    that is https://github.com/xiangyue9607/BioNEV/tree/master/data.

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
    CTDDDA graph.
    """
    return AutomaticallyRetrievedGraph(
        "YueCTDDDA",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()


def DrugBankDDI(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the DrugBankDDI graph used in the Yue paper.

    The retrieved graph is automatically retrieved directly from the GitHub
    repository of the Yue et al paper (https://academic.oup.com/bioinformatics/article/36/4/1241/5581350#),
    that is https://github.com/xiangyue9607/BioNEV/tree/master/data.

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
    DrugBankDDI graph.
    """
    return AutomaticallyRetrievedGraph(
        "YueDrugBankDDI",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()


def NDFRTDDA(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the NDFRTDDA graph used in the Yue paper.

    The retrieved graph is automatically retrieved directly from the GitHub
    repository of the Yue et al paper (https://academic.oup.com/bioinformatics/article/36/4/1241/5581350#),
    that is https://github.com/xiangyue9607/BioNEV/tree/master/data.

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
    NDFRTDDA graph.
    """
    return AutomaticallyRetrievedGraph(
        "YueNDFRTDDA",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path
    )()
