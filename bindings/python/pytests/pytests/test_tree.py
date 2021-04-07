from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
from .utils import load_hpo, load_pathway
import numpy as np


def test_tree():
    """Test execution of spanning arborescence."""
    for graph in tqdm((load_hpo(), load_pathway()), desc="Testing cooccurence", leave=False):
        _ = graph.spanning_arborescence()