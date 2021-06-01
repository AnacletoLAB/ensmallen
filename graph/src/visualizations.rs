use super::*;

impl Graph {

    /// Print the current graph in a format compatible with Graphviz dot's format.
    pub fn to_dot(&self, use_node_names: Option<bool>) -> String {
        let use_node_names = use_node_names.unwrap_or(false);

        let (graph_type, divider) = if self.is_directed() {
            ("digraph", "->")
        } else {
            ("graph", "--")
        };

        let mut result = format!("{} G {{\n", graph_type);

        for (_, src_id, src, dst_id, dst, _, _, weight) in self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true) {
            let src_name = if use_node_names {
                src
            } else {
                format!("{}", src_id)
            };

            let dst_name = if use_node_names {
                dst
            } else {
                format!("{}", dst_id)
            };
            match weight {
                Some(w) => {
                    result.extend(format!(
                        "\t{src} {divider} {dst} [label=\"{weight}\"];\n",
                        src=src_name,
                        divider=divider,
                        dst=dst_name,
                        weight=w,
                    ).chars());
                },
                None => {
                    result.extend(format!(
                        "\t{src} {divider} {dst};\n",
                        src=src_name,
                        divider=divider,
                        dst=dst_name,
                    ).chars());
                }
            }
        }

        result.extend("\n}".chars());

        result
    }
}
