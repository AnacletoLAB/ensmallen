from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module

def test_load_ppi():
    EnsmallenGraph.from_csv(
        edge_path="./pytests/data/ppi/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=False,
        weights_column="weight",
        node_path="./pytests/data/ppi/nodes.tsv",
        nodes_column="id",
        node_types_column="molecular_function",
        default_node_type="Missing"
    )
