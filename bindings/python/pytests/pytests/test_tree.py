from ensmallen import Graph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
import numpy as np
from ensmallen.datasets.linqs import Cora
from ensmallen.datasets.string import HomoSapiens


def test_tree():
    """Test execution of spanning arborescence."""
    for graph in tqdm((Cora(), HomoSapiens()), desc="Testing cooccurence", leave=False):
        _ = graph.spanning_arborescence()
