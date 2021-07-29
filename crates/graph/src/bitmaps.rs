use crate::graph::Graph;
use shared::*;
use roaring::RoaringBitmap;

/// # Drop.
/// The naming convention we follow is:
/// * `.*bitmap.*`
impl Graph {
    /// Return a roaringbitmap with the node ids to keep.
    ///
    /// If both node\_names and node\_types are specified the result will be the
    /// union of both queries.
    ///
    /// # Arguments
    /// * `node_names` - The nodes to keep as strings
    /// * `node_types` - The nodes types to keep as strings
    ///
    pub(crate) fn get_filter_bitmap(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
    ) -> Result<Option<RoaringBitmap>> {
        let mut node_ids = RoaringBitmap::new();

        if let Some(ns) = node_names {
            node_ids.extend(
                ns.iter()
                    .map(|node_name| self.get_node_id_from_node_name(node_name))
                    .collect::<Result<Vec<NodeT>>>()?,
            );
        }

        if let Some(ndt) = node_types {
            let node_type_ids = self.get_node_type_ids_from_node_type_names(ndt)?;
            node_ids.extend(
                self.iter_node_ids_and_node_type_ids()
                    .filter_map(|(node_id, nts)| {
                        if nts.map_or_else(
                            //DEFAULT
                            || node_type_ids.contains(&None),
                            // If some
                            |ns| {
                                ns.into_iter().any(|node_type_name| {
                                    node_type_ids.contains(&Some(node_type_name))
                                })
                            },
                        ) {
                            Some(node_id)
                        } else {
                            None
                        }
                    }),
            );
        }

        Ok(optionify!(node_ids))
    }
}
