use super::Graph;
use std::fs::File;
use std::io;
use std::io::prelude::*;  


/// # Holdouts.
impl Graph {
    /// Save the nodes to a loadable csv / tsv.
    ///
    /// # Arguments
    ///
    /// * nodes_path: str,
    ///     Where to save the nodes csv.
    /// * separator: str = "\t",
    ///     The separator to use for the csv or tsv file.
    /// * nodes_column: str = "id",
    ///     The name of the column with the names of the nodes.
    /// * node_types_column: str = "category",
    ///     The name of the column with the types of the nodes.
    /// * header: bool = True,
    ///     If false, the csv will have no header
    pub fn to_nodes_csv(
        &self,
        nodes_path: &str,
        separator: Option<&str>,
        nodes_column: Option<&str>,
        node_types_column: Option<&str>,
        header: Option<bool>
    ) -> io::Result<()> {
        let _sep = separator.unwrap_or("\t");
        let _header = header.unwrap_or(true);
        let mut file = File::create(nodes_path)?;
        if _header {
            // write columns
            let mut headers = if let Some(nc) = &nodes_column {
                    nc
                } else {
                    "id"
                }.to_string();

            if self.node_types.is_some() {
                headers.push_str(_sep);
                headers.push_str(
                    if let Some(nc) = &node_types_column {
                        nc
                    } else {
                        "category"
                    }
                );
            }

            headers.push_str("\n");
            file.write_all(headers.as_bytes())?;
        }

        for (id, node) in self.nodes_reverse_mapping.iter().enumerate() {
            let mut line = String::from(node);
            if let Some(nt) = &self.node_types {
                if let Some(ntrm) = &self.node_types_reverse_mapping {
                    line.push_str(_sep);
                    line.push_str(&ntrm[nt[id] as usize]);
                }
            }
            line.push('\n');
            file.write_all(line.as_bytes())?;
        }

        file.sync_all()
    }

    /// Save the edges to a loadable csv / tsv.
    ///
    /// # Arguments
    /// 
    /// * edges_path: str,
    ///     Where to save the nodes csv.
    /// * separator: str = "\t",
    ///     The separator to use for the csv or tsv file.
    /// * sources_column: str = "subject",
    ///     The name of the column with the names of the sources nodes.
    /// * destinations_column: str = "object",
    ///     The name of the column with the names of the destinations nodes.
    /// * edge_types_column: str = "edge_label",
    ///     The name of the column with the types of the edges.
    /// * weights_column: str = "weight",
    ///     The name of the column with the weight of the edges.
    /// * header: bool = True,
    ///     If false, the csv will have no header
    pub fn to_edges_csv(
        &self,
        edges_path: &str,
        separator: Option<&str>,
        sources_column: Option<&str>,
        destinations_column: Option<&str>,
        edge_types_column: Option<&str>,
        weights_column: Option<&str>,
        header: Option<bool>
    ) -> io::Result<()> {
        
        let _sep = separator.unwrap_or("\t");
        let _header = header.unwrap_or(true);
        let mut file = File::create(edges_path)?;
        if _header {
            // write columns
            let mut columns = String::new();
            columns.push_str(
                if let Some(sc) = &sources_column {
                    sc
                } else {
                    "subject"
                }
            );
            columns.push_str(_sep);
            columns.push_str(
                if let Some(dc) = &destinations_column {
                    dc
                } else {
                    "object"
                },
            );

            if self.edge_types.is_some() {
                columns.push_str(_sep);
                columns.push_str(
                    if let Some(ec) = &edge_types_column {
                        ec
                    } else {
                        "edge_label"
                    }
                );
            }

            if self.weights.is_some() {
                columns.push_str(_sep);
                columns.push_str(
                    if let Some(wc) = &weights_column {
                        wc
                    } else {
                        "weight"
                    }
                );
            }

            columns.push('\n');
            file.write_all(columns.as_bytes())?;
        }

        for (id, (src, dst)) in self.sources.iter().zip(self.destinations.iter() ).enumerate() {
            if !self.is_directed && (src > dst) {
                continue;
            }
            let mut line = String::from(&self.nodes_reverse_mapping[*src]);
            line.push_str(_sep);
            line.push_str(&self.nodes_reverse_mapping[*dst]);

            if let Some(et) = &self.edge_types {
                if let Some(etrm) = &self.edge_types_reverse_mapping {
                    line.push_str(_sep);
                    line.push_str(&etrm[et[id as usize] as usize]);
                }
            }
            if let Some(w) = &self.weights {
                line.push_str(_sep);
                line.push_str(&format!("{:.6}", w[id]));
            }

            line.push('\n');
            file.write_all(line.as_bytes())?;
        }

        file.sync_all()
    }

}