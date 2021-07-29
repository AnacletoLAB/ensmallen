"""Module offering fast graph processing and graph datasets."""
from .ensmallen_graph import EnsmallenGraph # pylint: disable=import-error
from .ensmallen_graph import preprocessing # pylint: disable=import-error
__all__ = ["EnsmallenGraph", "preprocessing"]