from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm
from ensmallen_graph.datasets.linqs import Cora
from ensmallen_graph.datasets.string import HomoSapiens


def test_return_weight_behaviour_cora():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = Cora()
    graph.enable()

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=20)
    return_weights = np.concatenate([return_weights, return_weights])

    for (seed, return_weight) in tqdm(
        enumerate(return_weights),
        desc="Computing walks for different return_weight",
        leave=False,
        total=len(return_weights)
    ):
        walks = graph.random_walks(
            quantity=graph.get_nodes_number(),
            walk_length=1000,
            return_weight=return_weight,
            random_state=seed
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(return_weights, mean_uniques_counts)
    print("Cora return weight", correlation, p_value)
    assert p_value < 0.01 and correlation < -0.70


def test_return_weight_behaviour_string_homo_sapiens():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the explore_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = HomoSapiens()
    graph.enable()

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=20)

    for return_weight in tqdm(
        return_weights,
        desc="Computing walks for different return_weight",
        leave=False
    ):
        walks = graph.complete_walks(
            walk_length=200,
            return_weight=return_weight,
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(return_weights, mean_uniques_counts)
    print("STRING return weight", correlation, p_value)
    assert p_value < 0.01 and correlation < -0.8


def test_explore_weight_behaviour_cora():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = Cora()
    graph.enable()
    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=50)

    for explore_weight in tqdm(
        explore_weights,
        desc="Computing walks for different explore_weights",
        leave=False
    ):
        walks = graph.complete_walks(
            walk_length=400,
            explore_weight=explore_weight,
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(explore_weights, mean_uniques_counts)
    print("Cora explore_weights", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.5


def test_explore_weight_behaviour_string_homo_sapiens():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the explore_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = HomoSapiens()
    graph.enable()

    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=50)

    for explore_weight in tqdm(
        explore_weights,
        desc="Computing walks for different explore_weights",
        leave=False
    ):
        walks = graph.complete_walks(
            walk_length=100,
            explore_weight=explore_weight,
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(explore_weights, mean_uniques_counts)
    print("STRING explore_weights", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.6


def test_change_node_type_weight_behaviour_cora():
    """
    The change_node_type_weight parameter increases the probability for a
    switch between the node types during the walk.

    Here we test for a strong statistically significant correlation between 
    the number of switches happening in a walk with the change of the value
    of the change_node_type_weight parameter.
    """
    graph = Cora()
    graph.enable()

    mean_changes = []
    change_node_type_weights = np.linspace(0.01, 10, num=50)

    for change_node_type_weight in tqdm(
        change_node_type_weights,
        desc="Computing walks for different change_node_type_weights",
        leave=False
    ):
        walks = graph.complete_walks(
            iterations=1,
            walk_length=100,
            change_node_type_weight=change_node_type_weight
        )
        changes = []
        for walk in walks:
            type_changes = 0
            previous_node_type = -1
            for node_id in walk:
                node_type_id = graph.get_node_type_id_from_node_id(node_id)
                if previous_node_type != node_type_id:
                    type_changes += 1
                    previous_node_type = node_type_id
            changes.append(type_changes/len(walk))
        mean_changes.append(np.mean(changes))

    correlation, p_value = pearsonr(
        change_node_type_weights, mean_changes)
    print("Cora change_node_type_weight", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.5
