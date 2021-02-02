"""
This file offers the methods to automatically retrieve the graph {graph_name}.

The graph is automatically retrieved from the {repository_name} repository. 

{report}

{references}

{usage_example}
"""
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def {graph_method_name}(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/{repository_package_name}"
) -> EnsmallenGraph:
    """Return new instance of the {graph_name} graph.

    The graph is automatically retrieved from the {repository_name} repository. 

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
    Instace of {graph_name} graph.

    {report}

    {references}

    {usage_example}
    """
    return AutomaticallyRetrievedGraph(
        "{graph_method_name}",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="{repository_package_name}"
    )()
