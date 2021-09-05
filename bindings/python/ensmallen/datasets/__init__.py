"""Module with datasets."""
from . import kghub, linqs, monarchinitiative, networkrepository, string, yue, zenodo, pheknowlatorkg
from .get_dataset import get_dataset, get_available_repositories, get_available_graphs_from_repository, get_all_available_graphs_dataframe

__all__ = [
    "get_dataset",
    "get_available_repositories",
    "get_available_graphs_from_repository",
    "get_all_available_graphs_dataframe",
    "kghub", "linqs", "monarchinitiative",
    "networkrepository", "string", "yue", "zenodo", "pheknowlatorkg"
]
