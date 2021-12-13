"""Module offering fast graph processing and graph datasets."""

import cpuinfo
import warnings

from .ensmallen import preprocessing  # pylint: disable=import-error
from .ensmallen import Graph  # pylint: disable=import-error
from .ensmallen import edge_list_utils  # pylint: disable=import-error

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = ["edge_list_utils", "Graph", "preprocessing", "datasets"]
