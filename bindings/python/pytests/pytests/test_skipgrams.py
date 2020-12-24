from tqdm.auto import tqdm
import numpy as np
from .utils import load_hpo, load_pathway


def test_skipgrams():
    """Test execution of skipgrams."""
    for graph in tqdm((load_hpo(), load_pathway()), desc="Testing Skipgrams", leave=False):
        words, contexts = graph.node2vec(
            batch_size=32,
            walk_length=50,
            window_size=4,
            random_state=42,
        )
        assert len(words) == len(contexts)
        embedding = np.random.uniform(size=(
            graph.get_nodes_number(),
            100
        ))
        graph.set_embedding(embedding)
        edges, labels = graph.link_prediction(
            idx=0,
            batch_size=32,
            method="Hadamard"
        )
        assert len(edges) == len(labels)
        assert set(labels) <= set([0, 1])
