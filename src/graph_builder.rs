use crate::graph::Graph;

struct GraphBuilder {
    edge_path: String,
    sources_column: String,
    destinations_column: String,
    directed: bool,
    edge_types_column: Option<String>,
    weights_column: Option<String>,
    node_path: Option<String>,
    nodes_column: Option<String>,
    node_types_column: Option<String>,
    edge_sep: Option<String>,
    node_sep: Option<String>,
    edge_file_has_header: Option<bool>,
    node_file_has_header: Option<bool>,
    validate_input_data: Option<bool>
}


impl GraphBuilder {

    pub fn new(edge_path: String,
        sources_column: String,
        destinations_column: String,
        directed: bool) -> GraphBuilder {
            
        }

    pub fn build(self) -> Graph {
        Graph::from_csv(
            self.edge_path,
            self.sources_column,
            self.destinations_column,
            self.directed,
            self.edge_types_column,
            self.weights_column,
            self.node_path,
            self.nodes_column,
            self.node_types_column,
            self.edge_sep,
            self.node_sep,
            self.edge_file_has_header,
            self.node_file_has_header,
            self.validate_input_data,
        }
}