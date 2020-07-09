from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from tqdm.auto import tqdm
from .utils import load_hpo, load_pathway


def test_skipgrams():
    """Test execution of skipgrams."""
    for graph in tqdm((load_hpo(), load_pathway()), desc="Testing Skipgrams", leave=False):
        (words, contexts), labels = graph.binary_skipgrams(0, 32, 80)
        assert len(words) == len(contexts)
        assert len(words) == len(labels)
        assert set(labels) <= set([0, 1])