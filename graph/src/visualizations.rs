use super::*;

impl Graph {
    /// Print the current graph in a format compatible with Graphviz dot's format.
    pub fn to_dot(&self) -> String {
        // choose type of graph and if the edges should be directed or not
        let (graph_type, divider) = if self.is_directed() {
            ("digraph", "->")
        } else {
            ("graph", "--")
        };

        let mut result = format!("{} G {{\n", graph_type);

        if self.has_node_types() {
            result.extend(
                format!(
                    "node [colorscheme={} style=filled ]",
                    if self.get_node_types_number().unwrap() < 8 {
                        "set28"
                    } else {
                        "set312"
                    }
                )
                .chars(),
            );
        }

        if self.has_edge_types() {
            result.extend(
                format!(
                    "edge [colorscheme={} style=filled ]",
                    if self.get_edge_types_number().unwrap() < 8 {
                        "set28"
                    } else {
                        "set312"
                    }
                )
                .chars(),
            );
        }

        // add the nodes info
        for (node_id, node_name, node_types_id, _node_types) in
            self.iter_node_names_and_node_type_names()
        {
            result.extend(
                format!(
                    "{node_id} [label=\"{node_name}\"{color}];\n",
                    node_id = node_id,
                    node_name = node_name,
                    color = node_types_id.map_or("".to_string(), |node_types_id| {
                        if self.has_node_types() {
                            // For now we only support a single color.
                            format!(" fillcolor={}", node_types_id[0] + 1)
                        } else {
                            "".to_string()
                        }
                    })
                )
                .chars(),
            );
        }

        // add the edges info
        for (_, src_id, _src, dst_id, _dst, edge_type_id, _edge_type, weight) in
            self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
        {
            // avioid double edges in undirected graphs
            if !self.is_directed() && src_id > dst_id {
                continue;
            }

            let mut edge_options = Vec::new();
            // add weight label if needed
            if let Some(w) = weight {
                edge_options.push(format!("label=\"{weight}\"", weight = w,));
            }

            if let Some(edge_type_id) = edge_type_id {
                edge_options.push(format!("color=\"{}\"", edge_type_id + 1));
            }
            // properly add the options only if the there are any
            result.extend(
                format!(
                    "\t{src} {divider} {dst} {edge_options};\n",
                    src = src_id,
                    divider = divider,
                    dst = dst_id,
                    edge_options = if edge_options.is_empty() {
                        "".to_string()
                    } else {
                        format!("[{}]", edge_options.join(" "))
                    }
                )
                .chars(),
            );
        }
        // close the graph
        result.extend("\n}".chars());

        result
    }
}
