from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
import numpy as np
from ensmallen_graph.datasets.linqs import Cora
from ensmallen_graph.datasets.string import HomoSapiens


def test_tree():
    """Test execution of spanning arborescence."""
    for graph in tqdm((Cora(), HomoSapiens()), desc="Testing cooccurence", leave=False):
        _ = graph.spanning_arborescence()
