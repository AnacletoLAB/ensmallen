from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module


def load_hpo():
    return EnsmallenGraph.from_csv(
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


def load_pathway():
    return EnsmallenGraph.from_csv(
        edge_path="./pytests/data/pathway.tsv",
        sources_column="Gene_A",
        destinations_column="Gene_B",
        directed=False,
    )
