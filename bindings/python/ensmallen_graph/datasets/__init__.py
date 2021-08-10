"""Module with datasets."""
from . import kghub, linqs, monarchinitiative, networkrepository, string, yue, zenodo, pheknowlatorkg
from .get_dataset import get_dataset, get_available_repository, get_available_graphs_from_repository

__all__ = [
    "get_dataset",
    "get_available_repository",
    "get_available_graphs_from_repository",
    "kghub", "linqs", "monarchinitiative",
    "networkrepository", "string", "yue", "zenodo", "pheknowlatorkg"
]
