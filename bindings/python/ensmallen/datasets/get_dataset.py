"""Utility to automatically retrieve a graph by name and repository."""
import os
from glob import glob
from typing import Any, Callable, List, Optional
from tqdm.auto import tqdm
import compress_json
import pandas as pd
from ensmallen import Graph, datasets
from userinput.utils import get_k_closest, must_be_in_set


def get_available_repositories() -> List[str]:
    """Return list of available repositories."""
    return [
        directory_candidate.split(os.sep)[-1].split(".")[0]
        for directory_candidate in glob(
            os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "*.json.gz"
            )
        )
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
    repository = must_be_in_set(
        repository,
        get_available_repositories(),
        "graph repository"
    )
    return compress_json.local_load("{repository}.json.gz".format(
        repository=repository
    )).keys()


def get_all_available_graphs() -> List[str]:
    """Return list of all available graphs."""
    return [
        name
        for repository in get_available_repositories()
        for name in get_available_graphs_from_repository(repository)
    ]


def get_available_versions_from_graph_and_repository_unchecked(graph_name: str, repository: str) -> List[str]:
    """Return list of available graphs from the given repositories.

    Parameters
    ----------------------
    graph_name: str,
        The name of the graph to retrieve.
    repository: str,
        The name of the repository to retrieve the graph from.

    Safety
    ----------------------
    The values must be correct or it will raise an index error.
    """
    return list(compress_json.local_load(
        "{}.json.gz".format(repository),
        use_cache=True
    )[graph_name].keys())


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
    repository = must_be_in_set(
        repository,
        get_available_repositories(),
        "graph repository"
    )
    graph_name = must_be_in_set(
        graph_name,
        get_available_graphs_from_repository(repository),
        "graph"
    )
    return get_available_versions_from_graph_and_repository_unchecked(
        graph_name=graph_name,
        repository=repository,
    )


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


def get_all_available_graphs_dataframe(verbose: bool = True) -> pd.DataFrame:
    """Return pandas dataframe with all the available graphs.,"""
    return pd.DataFrame([
        dict(
            repository=repository,
            name=name,
            version=version
        )
        for repository in tqdm(
            get_available_repositories(),
            desc="Parsing repositories",
            dynamic_ncols=True,
            leave=False,
            disable=not verbose
        )
        for name in tqdm(
            get_available_graphs_from_repository(repository),
            desc="Parsing graphs",
            dynamic_ncols=True,
            leave=False,
            disable=not verbose
        )
        for version in get_available_versions_from_graph_and_repository_unchecked(name, repository)
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
    must_be_in_set(
        version,
        get_available_versions_from_graph_and_repository(
            graph_name,
            repository
        ),
        "graph version"
    )


def get_dataset(
    graph_name: str,
    repository: Optional[str] = None,
    version: Optional[str] = None
) -> Callable[[Any], Graph]:
    """Return the graph curresponding to the given graph name, repository and version.

    Parameters
    ----------------------
    graph_name: str
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
        candidate_repositories = get_repositories_containing_graph(graph_name)
        if len(candidate_repositories) == 0:
            # If no candidate repository was found, then we need
            # to raise a proper error.
            raise ValueError(
                (
                    "The provided graph `{name}` is not available in any of the repositories.\n"
                    "The top 10 graphs with the most similar names are:\n"
                    "{similar_names}"
                ).format(
                    name=graph_name,
                    similar_names="".join([
                        "\t-{}\n".format(name)
                        for name in get_k_closest(
                            graph_name,
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
                    name=graph_name,
                    number_of_occurrences=len(candidate_repositories),
                    candidate_repositories="".join([
                        "\t-{}\n".format(candidate_repository)
                        for candidate_repository in candidate_repositories
                    ])
                )
            )

    graph_name = must_be_in_set(
        graph_name,
        get_available_graphs_from_repository(repository),
        "graph"
    )

    if version is not None:
        validate_graph_version(graph_name, repository, version)

    return getattr(getattr(datasets, repository), graph_name)
