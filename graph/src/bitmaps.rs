use super::*;
use roaring::RoaringBitmap;

/// # Drop.
impl Graph {
    /// Return a roaringbitmap with the node ids to keep.
    /// 
    /// If both node\_names and node\_types are specified the result will be the
    /// union of both queries.
    /// 
    /// # Arguments
    /// * `node_names` : Option<Vec<String>> - The nodes to keep as strings
    /// * `node_types` : Option<Vec<String>> - The nodes types to keep as strings
    /// 
    pub(crate) fn get_filter_bitmap(
        &self,
        mut node_names: Option<Vec<String>>,
        mut node_types: Option<Vec<String>>
    ) -> Result<RoaringBitmap, String> {
        let mut node_ids = RoaringBitmap::new();

        if let Some(ns) = node_names {
            node_ids.extend(
                ns.iter()
                    .map(|node_name| self.get_node_id(node_name))
                    .collect::<Result<Vec<NodeT>, String>>()?,
            );
        }

        if let Some(ndt) = node_types {
            if !self.has_node_types(){
                return Err("The current graph has no node types.".to_owned());
            }
            let node_types_ids = self.translate_node_types(ndt)?;
            node_ids.extend(self.get_nodes_iter().filter_map(|(node_id, node_type)| {
                if node_types_ids.contains(&node_type.unwrap()) {
                    return Some(node_id);
                }
                None
            }));
        }

        Ok(node_ids)
    }
}