from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm


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
        default_node_type='biolink:NamedThing',
        validate_input_data=False
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
    assert p_value < 0.01 and correlation > 0.8
