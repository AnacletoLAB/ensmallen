from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
from .utils import load_hpo, load_pathway
import numpy as np


def test_cooccurence_matrix():
    """Test execution of cooccurence_matrix."""
    for graph in tqdm((load_hpo(), load_pathway()), desc="Testing cooccurence", leave=False):
        words, contexts, frequencies = graph.cooccurence_matrix(80, verbose=False)
        assert len(words) == len(contexts)
        assert len(words) == len(frequencies)
        assert (np.array(frequencies) <= 1).all()
        assert (np.array(frequencies) > 0).all()
