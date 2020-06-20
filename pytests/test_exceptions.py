
import pytest
from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
from scipy.stats import pearsonr
import numpy as np
from tqdm.auto import tqdm

def test_no_existent_column():
    """The return weight parameter is the 'exploitation' parameter.

    The higher the return_weight parameter goes, the less exploration should
    happen in the walk.

    We test here that there is an inverse correlation between the number of
    different edges present in the walk and the parameter.
    """
    with pytest.raises(ValueError):
        graph = EnsmallenGraph(
            edge_path="./pytests/data/edges.tsv",
            sources_column="subject",
            destinations_column="NOT A REAL COLUMNS",
            directed=False,
            edge_types_column="edge_label",
            node_path="./pytests/data/nodes.tsv",
            nodes_column="id",
            node_types_column="category",
            default_edge_type='biolink:interacts_with',
            default_node_type='biolink:NamedThing'
        )

    with pytest.raises(ValueError):
        graph = EnsmallenGraph(
            edge_path="./pytests/data/edges.tsv",
            sources_column="subject",
            destinations_column="",
            directed=False,
            edge_types_column="NOT A REAL COLUMNS",
            node_path="./pytests/data/nodes.tsv",
            nodes_column="id",
            node_types_column="category",
            default_edge_type='biolink:interacts_with',
            default_node_type='biolink:NamedThing'
        )

    with pytest.raises(ValueError):
        graph = EnsmallenGraph(
            edge_path="./pytests/data/edges.tsv",
            sources_column="subject",
            destinations_column="",
            directed=False,
            edge_types_column="edge_label",
            node_path="./pytests/data/nodes.tsv",
            nodes_column="id",
            node_types_column="NOT A REAL COLUMNS",
            default_edge_type='biolink:interacts_with',
            default_node_type='biolink:NamedThing'
        )

    with pytest.raises(ValueError):
        graph = EnsmallenGraph(
            edge_path="./pytests/data/edges.tsv",
            sources_column="subject",
            destinations_column="object",
            directed=False,
            edge_types_column="edge_label",
            node_path="./pytests/data/nodes.tsv",
            nodes_column="NOT A REAL COLUMNS",
            node_types_column="category",
            default_edge_type='biolink:interacts_with',
            default_node_type='biolink:NamedThing'
        )
