from .automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ..ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def KGCOVID19(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the KG-COVID19 graph.

    TODO: Further describe the graph!

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
    Instace of KG-COVID graph.

    References
    -----------------------
    https://www.cell.com/patterns/pdf/S2666-3899(20)30203-8.pdf
    """
    return AutomaticallyRetrievedGraph(
        "KG-COVID19",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def ChEMBL(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the ChEMBL graph.

    TODO: Further describe the graph!

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
    Instace of ChEMBL graph.
    """
    return AutomaticallyRetrievedGraph(
        "ChEMBL",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def GOCAMs(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the GOCAMs graph.

    TODO: Further describe the graph!

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
    Instace of GOCAMs graph.
    """
    return AutomaticallyRetrievedGraph(
        "GOCAMs",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def STRING(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the STRING graph.

    TODO: Further describe the graph!

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
    Instace of STRING graph.
    """
    return AutomaticallyRetrievedGraph(
        "STRING",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def DrugCentral(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the DrugCentral graph.

    TODO: Further describe the graph!

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
    Instace of DrugCentral graph.
    """
    return AutomaticallyRetrievedGraph(
        "DrugCentral",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def IntAct(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the IntAct graph.

    TODO: Further describe the graph!

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
    Instace of IntAct graph.
    """
    return AutomaticallyRetrievedGraph(
        "IntAct",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def PharmGKB(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the PharmGKB graph.

    TODO: Further describe the graph!

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
    Instace of PharmGKB graph.
    """
    return AutomaticallyRetrievedGraph(
        "PharmGKB",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def SARSCOV2GeneAnnot(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the SARSCOV2GeneAnnot graph.

    TODO: Further describe the graph!

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
    Instace of SARSCOV2GeneAnnot graph.
    """
    return AutomaticallyRetrievedGraph(
        "SARSCOV2GeneAnnot",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()


def ZhouHostProteins(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs"
) -> EnsmallenGraph:
    """Return new instance of the ZhouHostProteins graph.

    TODO: Further describe the graph!

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
    Instace of ZhouHostProteins graph.
    """
    return AutomaticallyRetrievedGraph(
        "ZhouHostProteins",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="kg_hub"
    )()
