"""Module offering fast graph processing and graph datasets."""
from .wrapper_ensmallen_graph import WrapperEnsmallenGraph as EnsmallenGraph
from .ensmallen_graph import preprocessing  # pylint: disable=import-error

__all__ = [
    "EnsmallenGraph",
    "preprocessing"
]
