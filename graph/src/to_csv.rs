use super::Graph;
use std::fs::File;
use std::io;
use std::io::prelude::*;  

/// # Holdouts.
impl Graph {
    pub fn to_nodes_csv(
        &self,
        nodes_path: &str,
        separator: Option<&str>,
        nodes_column: Option<&str>,
        node_types_column: Option<&str>,
    ) -> io::Result<()> {
        let _sep = separator.unwrap_or("\t");
        let mut file = File::create(nodes_path)?;
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

    pub fn to_edges_csv(
        &self,
        edges_path: &str,
        separator: Option<&str>,
        sources_column: Option<&str>,
        destinations_column: Option<&str>,
        edge_types_column: Option<&str>,
        weights_column: Option<&str>,
    ) -> io::Result<()> {
        
        let _sep = separator.unwrap_or("\t");
        let mut file = File::create(edges_path)?;
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