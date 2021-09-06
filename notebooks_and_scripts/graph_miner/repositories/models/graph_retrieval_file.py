"""
This file offers the methods to automatically retrieve the graph {graph_name}.

The graph is automatically retrieved from the {repository_name} repository. 
{description}

{references}
"""
from typing import Dict
{imports}
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def {graph_method_name}(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/{repository_package_name}",
    version: str = "{default_version}",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the {graph_name} graph.

    The graph is automatically retrieved from the {repository_name} repository.{tabbed_description}

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True,
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True,
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    version: str = "{default_version}",
        The version of the graph to retrieve.{available_graph_versions}
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of {graph_name} graph.

{tabbed_references}
    """
    return AutomaticallyRetrievedGraph(
        graph_name="{graph_method_name}",
        repository="{repository_package_name}",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs{callbacks_data}
    )()
