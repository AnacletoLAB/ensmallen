"""Module with datasets."""
from . import (kghub, kgobo, linqs, monarchinitiative, networkrepository,
               pheknowlatorkg, string, yue, zenodo, wikidata)
from .get_dataset import (get_all_available_graphs_dataframe,
                          get_available_graphs_from_repository,
                          get_available_repositories,
                          get_available_versions_from_graph_and_repository,
                          validate_graph_version,
                          get_dataset)

__all__ = [
    "get_dataset",
    "get_available_repositories",
    "get_available_graphs_from_repository",
    "get_all_available_graphs_dataframe",
    "get_available_versions_from_graph_and_repository",
    "validate_graph_version",
    "kghub", "kgobo", "linqs", "monarchinitiative", "wikidata",
    "networkrepository", "string", "yue", "zenodo", "pheknowlatorkg"
]
