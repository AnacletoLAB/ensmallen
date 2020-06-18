from ensmallen_graph import EnsmallenGraph

def test_placeholder():
    return 
    graph = EnsmallenGraph(
        edge_path="data/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=True,
        edge_types_column="edge_label",
        node_path="data/nodes.tsv",
        nodes_column="id",
        node_types_column="category",
        default_edge_type='biolink:interacts_with',
        default_node_type='biolink:NamedThing'
    )
    graph.walk(
        iterations=10,
        length=80,
        min_length=0,
        return_weight=1,
        explore_weight=1,
        change_node_type_weight=1,
        change_edge_type_weight=1
    )