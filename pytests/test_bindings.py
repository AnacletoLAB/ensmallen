from ensmallen_graph import EnsmallenGraph
from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm


def test_return_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = EnsmallenGraph(
        edge_path="./pytests/data/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=False,
        edge_types_column="edge_label",
        node_path="./pytests/data/nodes.tsv",
        nodes_column="id",
        node_types_column="category",
        default_edge_type='biolink:interacts_with',
        default_node_type='biolink:NamedThing'
    )

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=100)

    for return_weight in tqdm(
            return_weights, desc="Computing walks for different return_weight"):
        walks = graph.walk(
            iterations=1,
            length=100,
            min_length=0,
            return_weight=return_weight,
            explore_weight=1,
            change_node_type_weight=1,
            change_edge_type_weight=1
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
    graph = EnsmallenGraph(
        edge_path="./pytests/data/pathway.tsv",
        sources_column="Gene_A",
        destinations_column="Gene_B",
        directed=False,
    )

    mean_uniques_counts = []
    return_weights = np.linspace(0.01, 10, num=100)

    for return_weight in tqdm(
            return_weights, desc="Computing walks for different return_weight"):
        walks = graph.walk(
            iterations=1,
            length=100,
            min_length=0,
            return_weight=return_weight,
            explore_weight=1,
            change_node_type_weight=1,
            change_edge_type_weight=1
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(return_weights, mean_uniques_counts)
    print("HPO return weight", correlation, p_value)
    assert p_value < 0.01 and correlation < -0.9


def test_explore_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = EnsmallenGraph(
        edge_path="./pytests/data/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=False,
        edge_types_column="edge_label",
        node_path="./pytests/data/nodes.tsv",
        nodes_column="id",
        node_types_column="category",
        default_edge_type='biolink:interacts_with',
        default_node_type='biolink:NamedThing'
    )

    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=100)

    for explore_weight in tqdm(
            explore_weights, desc="Computing walks for different explore_weights"):
        walks = graph.walk(
            iterations=1,
            length=100,
            min_length=0,
            return_weight=1,
            explore_weight=explore_weight,
            change_node_type_weight=1,
            change_edge_type_weight=1
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
    graph = EnsmallenGraph(
        edge_path="./pytests/data/pathway.tsv",
        sources_column="Gene_A",
        destinations_column="Gene_B",
        directed=False,
    )

    mean_uniques_counts = []
    explore_weights = np.linspace(0.01, 10, num=100)

    for explore_weight in tqdm(
            explore_weights, desc="Computing walks for different explore_weights"):
        walks = graph.walk(
            iterations=1,
            length=100,
            min_length=0,
            return_weight=1,
            explore_weight=explore_weight,
            change_node_type_weight=1,
            change_edge_type_weight=1
        )
        mean_uniques_counts.append(np.mean([
            np.unique(walk).size
            for walk in walks
        ]))

    correlation, p_value = pearsonr(explore_weights, mean_uniques_counts)
    print("Pathway explore_weights", correlation, p_value)
    assert p_value < 0.01 and correlation > 0.6


def test_change_node_type_weight_behaviour_hpo():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different nodes present in the walk and the parameter.
    """
    graph = EnsmallenGraph(
        edge_path="./pytests/data/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=False,
        edge_types_column="edge_label",
        node_path="./pytests/data/nodes.tsv",
        nodes_column="id",
        node_types_column="category",
        default_edge_type='biolink:interacts_with',
        default_node_type='biolink:NamedThing'
    )

    mean_changes_counts = []
    change_node_type_weights = np.linspace(0.01, 10, num=100)

    for change_node_type_weight in tqdm(
        change_node_type_weights,
        desc="Computing walks for different change_node_type_weights"
    ):
        walks = graph.walk(
            iterations=1,
            length=100,
            min_length=0,
            return_weight=1,
            explore_weight=1,
            change_node_type_weight=change_node_type_weight,
            change_edge_type_weight=1
        )
        node_changes = []
        for walk in walks:
            changes = 0
            previous_node_type = -1
            for node_id in walk:
                node_type_id = graph.get_node_type_id(node_id)
                if previous_node_type != node_type_id:
                    changes += 1
                    previous_node_type = node_type_id
            node_changes.append(changes/len(walk))
        mean_changes_counts.append(np.mean(node_changes))

    correlation, p_value = pearsonr(
        change_node_type_weights, mean_changes_counts)
    print("HPO change_node_type_weight", correlation, p_value)
    #assert p_value < 0.01 and correlation > 0.6
