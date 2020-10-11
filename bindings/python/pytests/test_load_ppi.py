from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
import os

ROOT_DIR = os.path.abspath(
    os.path.dirname(__file__)
)


def test_load_ppi() -> EnsmallenGraph:
    """Test that PPI can be loaded."""
    EnsmallenGraph.from_unsorted_csv(
        edge_path=os.path.join(ROOT_DIR, "data/ppi/edges.tsv"),
        sources_column="subject",
        destinations_column="object",
        directed=False,
        weights_column="weight",
        node_path="./pytests/data/ppi/nodes.tsv",
        nodes_column="id",
        node_types_column="molecular_function",
        default_node_type="Missing"
    )
