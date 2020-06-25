from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm
from .utils import load_hpo


def test_change_node_type_weight_behaviour_hpo():
    """
    The change_node_type_weight parameter increases the probability for a
    switch between the node types during the walk.

    Here we test for a strong statistically significant correlation between 
    the number of switches happening in a walk with the change of the value
    of the change_node_type_weight parameter.
    """
    graph = load_hpo()

    mean_changes = []
    change_node_type_weights = np.linspace(0.01, 10, num=50)

    for change_node_type_weight in tqdm(
        change_node_type_weights,
        desc="Computing walks for different change_node_type_weights"
    ):
        walks = graph.walk(
            iterations=1,
            length=100,
            change_node_type_weight=change_node_type_weight,
        )
        changes = []
        for walk in walks:
            type_changes = 0
            previous_node_type = -1
            for node_id in walk:
                node_type_id = graph.get_node_type_id(node_id)
                if previous_node_type != node_type_id:
                    type_changes += 1
                    previous_node_type = node_type_id
            changes.append(type_changes/len(walk))
        mean_changes.append(np.mean(changes))

    correlation, p_value = pearsonr(
        change_node_type_weights, mean_changes)
    print("HPO change_node_type_weight", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.8
