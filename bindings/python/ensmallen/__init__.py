"""Module offering fast graph processing and graph datasets."""
from .ensmallen import edge_list_utils # pylint: disable=import-error
from .ensmallen import Graph # pylint: disable=import-error
from .ensmallen import preprocessing # pylint: disable=import-error
__all__ = ["edge_list_utils", "Graph", "preprocessing"]