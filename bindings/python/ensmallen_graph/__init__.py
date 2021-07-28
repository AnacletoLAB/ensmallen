"""Module offering fast graph processing and graph datasets."""
from .ensmallen_graph import edge_list_utils # pylint: disable=import-error
from .ensmallen_graph import url_utilities # pylint: disable=import-error
from .ensmallen_graph import constructors # pylint: disable=import-error
from .ensmallen_graph import EnsmallenGraph # pylint: disable=import-error
__all__ = ["edge_list_utils", "url_utilities", "constructors", "EnsmallenGraph"]