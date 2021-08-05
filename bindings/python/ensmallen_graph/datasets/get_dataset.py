"""Utility to automatically retrieve a graph by name and repository."""
from typing import Optional, Any, Callable, List
from ensmallen_graph import EnsmallenGraph
from ensmallen_graph import datasets
from glob import glob
from userinput.utils import set_validator, closest
import os


def get_available_repository() -> List[str]:
    """Return list of available repositories."""
    return [
        directory_candidate.split(os.sep)[-1]
        for directory_candidate in glob(
            os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "*"
            )
        )
        if os.path.isdir(directory_candidate)
    ]


def get_available_graphs_from_repository(repository: str) -> List[str]:
    """Return list of available graphs from the given repositories.

    Parameters
    ----------------------
    repository: str,
        The name of the repository to retrieve the graph from.

    Raises
    ----------------------
    ValueError,
        If the given repository is not available.
    """
    repositories = get_available_repository()
    if not set_validator(repositories)(repository):
        raise ValueError((
            "The provided repository `{}` is not within the set "
            "of supported repositories, {}.\n"
            "Did you mean `{}`?"
        ).format(
            repository,
            ", ".join(repositories),
            closest(repository, repositories)
        ))

    return [
        ".".join(path.split(os.sep)[-1].split(".")[:-2])
        for path in glob(os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            repository,
            "*.json.gz"
        ))
    ]


def get_dataset(
    graph_name: str,
    repository: str,
) -> Callable[[Any], EnsmallenGraph]:
    """Return the graph curresponding to the given graph name, repository and version.

    Parameters
    ----------------------
    graph_name: str,
        The name of the graph to retrieve.
    repository: str,
        The name of the repository to retrieve the graph from.

    Raises
    ----------------------
    ValueError,
        If the given repository is not available.
    ValueError,
        If the given graph is not available.
    """

    graph_names = get_available_graphs_from_repository(repository)

    if not set_validator(graph_names)(graph_name):

        # We check if the given graph is from another repository
        other_repository = None
        for candidate_repository in get_available_repository():
            if graph_name in get_available_graphs_from_repository(
                candidate_repository
            ):
                other_repository = candidate_repository

        raise ValueError((
            "The provided graph name `{}` is not within the set "
            "of supported graph names within the repository {}.\n"
            "Did you mean `{}`?\n"
            "{}"
            "The complete set of graphs available from the given "
            "repository is {}."
        ).format(
            graph_name,
            repository,
            closest(graph_name, graph_names),
            "" if other_repository is None else "We have found a graph with the given name in the repository `{}`. Maybe you wanted to use this one?\n".format(
                other_repository
            ),
            ", ".join(graph_names),
        ))

    return getattr(getattr(datasets, repository), graph_name)
