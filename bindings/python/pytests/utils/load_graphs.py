from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module


def load_hpo() -> EnsmallenGraph:
    """Test that HPO graph can be loaded."""
    graph =  EnsmallenGraph.from_unsorted_csv(
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
    graph.enable_fast_walk()
    return graph


def load_pathway() -> EnsmallenGraph:
    """Test that Pathway can be loaded."""
    graph = EnsmallenGraph.from_unsorted_csv(
        edge_path="./pytests/data/pathway.tsv",
        sources_column="Gene_A",
        destinations_column="Gene_B",
        directed=False,
    )
    graph.enable_fast_walk()
    return graph
