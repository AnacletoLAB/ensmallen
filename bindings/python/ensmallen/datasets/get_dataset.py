"""Utility to automatically retrieve a graph by name and repository."""
import os
from glob import glob
from typing import Any, Callable, List, Optional

import compress_json
import pandas as pd
from ensmallen import Graph, datasets
from userinput.utils import closest, set_validator, get_k_closest


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


def get_all_available_graphs() -> List[str]:
    """Return list of all available graphs."""
    return [
        name
        for repository in get_available_repositories()
        for name in get_available_graphs_from_repository(repository)
    ]


def get_available_versions_from_graph_and_repository(name: str, repository: str) -> List[str]:
    """Return list of available graphs from the given repositories.

    Parameters
    ----------------------
    name: str,
        The name of the graph to retrieve.
    repository: str,
        The name of the repository to retrieve the graph from.

    Raises
    ----------------------
    ValueError,
        If the given repository is not available.
    """
    return list(compress_json.local_load("{}.json.gz".format(repository))[name].keys())


def get_repositories_containing_graph(name: str) -> List[str]:
    """Returns the repositories containing a graph with the given graph name.

    Parameters
    ----------------------------
    name: str,
        The name of the graph to retrieve.

    Returns
    ----------------------------
    List of repository names.
    """
    return [
        repository
        for repository in get_available_repositories()
        if name in get_available_graphs_from_repository(repository)
    ]


def get_all_available_graphs_dataframe() -> pd.DataFrame:
    """Return pandas dataframe with all the available graphs.,"""
    return pd.DataFrame([
        dict(
            repository=repository,
            name=name,
            version=version
        )
        for repository in get_available_repositories()
        for name in get_available_graphs_from_repository(repository)
        for version in get_available_versions_from_graph_and_repository(name, repository)
    ])


def validate_graph_version(
    name: str,
    repository: str,
    version: str
):
    """Validates given triple.

    Parameters
    ----------------------
    name: str,
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
        name,
        repository
    )
    if not set_validator(all_versions)(version):
        raise ValueError((
            "The provided version `{}` is not within the set "
            "of supported versions of the graph `{}` and repository `{}`, {}.\n"
            "Did you mean `{}`?"
        ).format(
            repository,
            name,
            repository,
            ", ".join(all_versions),
            closest(version, all_versions)
        ))


def get_dataset(
    name: str,
    repository: Optional[str] = None,
    version: Optional[str] = None
) -> Callable[[Any], Graph]:
    """Return the graph curresponding to the given graph name, repository and version.

    Parameters
    ----------------------
    name: str = None
        The name of the graph to retrieve.
    repository: Optional[str] = None
        The name of the repository to retrieve the graph from.
        This is needed only when there is not an unique graph name for the
        provided graph.
    version: Option[str] = None
        The version of the graph to retrieve.
        Note that this will ONLY check that the version is available.

    Raises
    ----------------------
    ValueError,
        If the given repository is not available.
    ValueError,
        If the given graph is not available.
    """
    # If the repository was not specified
    if repository is None:
        # We retrieve the repositoris that seem to contain this graph.
        candidate_repositories = get_repositories_containing_graph(name)
        if len(candidate_repositories) == 0:
            # If no candidate repository was found, then we need
            # to raise a proper error.
            raise ValueError(
                (
                    "The provided graph `{name}` is not available in any of the repositories.\n"
                    "The top 10 graphs with the most similar names are:\n"
                    "{similar_names}"
                ).format(
                    name=name,
                    similar_names="".join([
                        "\t-{}\n".format(name)
                        for name in get_k_closest(
                            name,
                            get_all_available_graphs(),
                            k=10
                        )
                    ])
                )
            )
        elif len(candidate_repositories) == 1:
            # We have found the repository we wanted!
            repository = candidate_repositories[0]
        elif len(candidate_repositories) > 1:
            raise ValueError(
                (
                    "The provided graph `{name}` appears in {number_of_occurrences} repositories "
                    "and therefore it is not possible to automatically infer from where to extract "
                    "this specific graph.\n"
                    "Specifically, the repositories that include the provided graph name are:\n"
                    "{candidate_repositories}"
                ).format(
                    name=name,
                    number_of_occurrences=len(candidate_repositories),
                    candidate_repositories="".join([
                        "\t-{}\n".format(candidate_repository)
                        for candidate_repository in candidate_repositories
                    ])
                )
            )

    names = get_available_graphs_from_repository(repository)
    if not set_validator(names)(name):
        raise ValueError((
            "The provided graph `{name}` is not available in the repository {repository}.\n"
            "The top 10 graphs within the given repository with the most similar names are:\n"
            "{similar_names}"
        ).format(
            name=name,
            repository=repository,
            similar_names="".join([
                "\t-{}\n".format(name)
                for name in get_k_closest(
                    name,
                    names,
                    k=10
                )
            ])
        ))

    if version is not None:
        validate_graph_version(name, repository, version)

    return getattr(getattr(datasets, repository), name)
