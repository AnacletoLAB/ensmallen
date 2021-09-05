from ensmallen import EnsmallenGraph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
import numpy as np
from ensmallen.datasets.linqs import Cora
from ensmallen.datasets.string import HomoSapiens


def test_cooccurence_matrix():
    """Test execution of cooccurence_matrix."""
    for graph in tqdm((Cora(), HomoSapiens()), desc="Testing cooccurence", leave=False):
        words, contexts, frequencies = graph.cooccurence_matrix(80, verbose=False)
        assert len(words) == len(contexts)
        assert len(words) == len(frequencies)
        assert (np.array(frequencies) <= 1).all()
        assert (np.array(frequencies) > 0).all()
