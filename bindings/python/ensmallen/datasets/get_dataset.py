"""Utility to automatically retrieve a graph by name and repository."""
from typing import Optional, Any, Callable, List
from ensmallen import Graph
from ensmallen import datasets
from glob import glob
import compress_json
from userinput.utils import set_validator, closest
import os
import pandas as pd


def get_available_repositories() -> List[str]:
    """Return list of available repositories."""
    black_list = {
        "__pycache__"
    }
    return [
        directory_candidate.split(os.sep)[-1]
        for directory_candidate in glob(
            os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "*"
            )
        )
        if os.path.isdir(directory_candidate) and directory_candidate.split(os.sep)[-1] not in black_list
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
    repositories = get_available_repositories()
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


def get_available_versions_from_graph_and_repository(graph_name: str, repository: str) -> List[str]:
    """Return list of available graphs from the given repositories.

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
    """
    return list(compress_json.local_load(os.path.join(
        repository,
        "{}.json.gz".format(graph_name)
    )).keys())


def get_all_available_graphs_dataframe() -> pd.DataFrame:
    """Return pandas dataframe with all the available graphs.,"""
    return pd.DataFrame([
        dict(
            repository=repository,
            graph_name=graph_name,
            version=version
        )
        for repository in get_available_repositories()
        for graph_name in get_available_graphs_from_repository(repository)
        for version in get_available_versions_from_graph_and_repository(graph_name, repository)
    ])


def validate_graph_version(
    graph_name: str,
    repository: str,
    version: str
):
    """Validates given triple.

    Parameters
    ----------------------
    graph_name: str,
        The name of the graph to retrieve.
    repository: str,
        The name of the repository to retrieve the graph from.
    version: str,
        The version to check for.

    Raises
    ----------------------
    ValueError,
        If the given repository is not available.
    """
    all_versions = get_available_versions_from_graph_and_repository(
        graph_name, repository)
    if not set_validator(all_versions)(version):
        raise ValueError((
            "The provided version `{}` is not within the set "
            "of supported versions of the graph `{}` and repository `{}`, {}.\n"
            "Did you mean `{}`?"
        ).format(
            repository,
            graph_name,
            repository,
            ", ".join(all_versions),
            closest(version, all_versions)
        ))


def get_dataset(
    graph_name: str,
    repository: str,
    version: Optional[str] = None
) -> Callable[[Any], Graph]:
    """Return the graph curresponding to the given graph name, repository and version.

    Parameters
    ----------------------
    graph_name: str,
        The name of the graph to retrieve.
    repository: str,
        The name of the repository to retrieve the graph from.
    version: Option[str],
        The version of the graph to retrieve.
        Note that this will ONLY check that the version is available.

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
        for candidate_repository in get_available_repositories():
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
