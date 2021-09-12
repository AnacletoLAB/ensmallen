use super::*;

impl Graph {
    /// Print the current graph in a format compatible with Graphviz dot's format.
    pub fn to_dot(&self, use_node_names: Option<bool>) -> String {
        let use_node_names = use_node_names.unwrap_or(false);
        
        // choose type of graph and if the edges should be directed or not
        let (graph_type, divider) = if self.is_directed() {
            ("digraph", "->")
        } else {
            ("graph", "--")
        };
    
        let mut result = format!("{} G {{\n", graph_type);

        // add the nodes info
        for (node_id, node_name, _node_types_id, _node_types) in self.iter_node_names_and_node_type_names() {
            result.extend(format!(
                "{node_id} [label=\"{node_name}\"];\n",
                node_id = node_id,
                node_name = node_name,
            ).chars());
        }

        // add the edges info
        for (_, src_id, _src, dst_id,_dst, _edge_type_id, _edge_type, weight) in
            self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
        {
            // avioid double edges in undirected graphs
            if !self.is_directed() && src_id > dst_id {
                continue;
            }

            let mut edge_options = Vec::new();
            // add weight label if needed
            if let Some(w) = weight {
                edge_options.push(format!(
                    "label=\"{weight}\"",
                    weight=w,
                ));
            }
            // properly add the options only if the there are any
            if edge_options.is_empty() {
                result.extend(
                    format!(
                        "\t{src} {divider} {dst};\n",
                        src = src_id,
                        divider = divider,
                        dst = dst_id,
                    )
                    .chars(),
                );
            } else {
                result.extend(
                    format!(
                        "\t{src} {divider} {dst} [{edge_options}];\n",
                        src = src_id,
                        divider = divider,
                        dst = dst_id,
                        edge_options=edge_options.join(" "),
                    )
                    .chars(),
                );
            }
        }
        // close the graph
        result.extend("\n}".chars());

        result
    }
}
