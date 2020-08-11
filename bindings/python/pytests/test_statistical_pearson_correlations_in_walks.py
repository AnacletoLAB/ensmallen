from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm
from .utils import load_hpo, load_pathway


def test_return_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = load_hpo()

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=50)

    for return_weight in tqdm(
        return_weights,
        desc="Computing walks for different return_weight",
        leave=False
    ):
        walks = graph.walk(
            length=100,
            return_weight=return_weight,
            verbose=False
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(return_weights, mean_uniques_counts)
    print("HPO return weight", correlation, p_value)
    assert p_value < 0.01 and correlation < -0.9


def test_return_weight_behaviour_pathway():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the explore_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = load_pathway()

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=50)

    for return_weight in tqdm(
        return_weights,
        desc="Computing walks for different return_weight",
        leave=False
    ):
        walks = graph.walk(
            length=100,
            return_weight=return_weight,
            verbose=False
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(return_weights, mean_uniques_counts)
    print("Pathway return weight", correlation, p_value)
    assert p_value < 0.01 and correlation < -0.9


def test_explore_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = load_hpo()

    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=50)

    for explore_weight in tqdm(
        explore_weights,
        desc="Computing walks for different explore_weights",
        leave=False
    ):
        walks = graph.walk(
            length=100,
            explore_weight=explore_weight,
            verbose=False
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(explore_weights, mean_uniques_counts)
    print("HPO explore_weights", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.6


def test_explore_weight_behaviour_pathway():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the explore_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = load_pathway()

    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=50)

    for explore_weight in tqdm(
        explore_weights,
        desc="Computing walks for different explore_weights",
        leave=False
    ):
        walks = graph.walk(
            length=100,
            explore_weight=explore_weight,
            verbose=False
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(explore_weights, mean_uniques_counts)
    print("Pathway explore_weights", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.6


def test_change_edge_type_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different edges present in the walk and the parameter.
    """
    graph = load_hpo()

    mean_changes_counts = []
    change_edge_type_weights = np.linspace(0.01, 10, num=50)

    for change_edge_type_weight in tqdm(
        change_edge_type_weights,
        desc="Computing walks for different change_edge_type_weights",
        leave=False
    ):
        walks = graph.walk(
            length=100,
            change_edge_type_weight=change_edge_type_weight,
            verbose=False
        )
        edge_changes = []
        for walk in walks:
            changes = 0
            previous_egde_type = -1
            for src, dst in zip(walk[:-1], walk[1:]):
                edge_type_id = graph.get_edge_type_id(
                    graph.get_edge_id(src, dst)
                )
                if previous_egde_type != edge_type_id:
                    changes += 1
                    previous_egde_type = edge_type_id
            edge_changes.append(changes/len(walk))
        mean_changes_counts.append(np.mean(edge_changes))

    correlation, p_value = pearsonr(
        change_edge_type_weights, mean_changes_counts)
    print("HPO change_edge_type_weight", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.9

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
        desc="Computing walks for different change_node_type_weights",
        leave=False
    ):
        walks = graph.walk(
            iterations=1,
            length=100,
            change_node_type_weight=change_node_type_weight,
            verbose=False,
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
