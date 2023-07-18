use crate::constructors::build_graph_from_strings_without_type_iterators;

use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use rayon::prelude::*;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # remove.
impl Graph {
    /// Return a new graph with solely the requested connected components.
    ///
    /// # Arguments
    /// * `node_names`: Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types`: Option<&[Option<&str>]> - The types of the nodes of which components to keep.
    /// * `edge_types`: Option<&[Option<&str>]> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<NodeT> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<NodeT> - Optional, number of components to keep sorted by number of nodes.
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<&[Option<&str>]>,
        edge_types: Option<&[Option<&str>]>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
    ) -> Result<Graph> {
        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_connected_component_ids(None);

        let number_of_components = components_vector.par_iter().copied().max().unwrap_or(0);

        // Extend the components so the include the given node Ids and node types.
        if let Some(node_ids) = self.get_filter_bitmap(node_names, node_types)? {
            keep_components.extend(
                node_ids
                    .iter()
                    .map(|node_id| components_vector[node_id as usize]),
            );
        }

        // Extend the components to keep those that include the given edge types.
        if let Some(ets) = edge_types {
            let edge_types_ids: HashSet<Option<EdgeTypeT>> = self
                .get_edge_type_ids_from_edge_type_names(ets)?
                .into_iter()
                .collect();

            self.iter_edge_node_ids_and_edge_type_id(self.directed)
                .for_each(|(_, src, dst, edge_type)| {
                    if edge_types_ids.contains(&edge_type) {
                        keep_components.insert(components_vector[src as usize]);
                        keep_components.insert(components_vector[dst as usize]);
                    }
                });
        }

        // Create the components counter
        let counter = Counter::init(components_vector.clone());
        let component_counts: Vec<(NodeT, NodeT)> = counter.most_common_ordered();

        // Insert the top k biggest components components
        if let Some(tkc) = top_k_components {
            for (i, (component_id, _)) in component_counts.iter().enumerate() {
                if i < tkc as usize {
                    keep_components.insert(*component_id);
                }
            }
        }

        // If there is no positive filter on the components
        // we initialize the vector of the components to
        // be kept as all of the components.
        if keep_components.len() == 0 {
            (0..number_of_components).for_each(|component_id| {
                keep_components.insert(component_id);
            });
        }

        // Remove components smaller than the given amount
        if let Some(mcs) = &minimum_component_size {
            component_counts
                .iter()
                .for_each(|(component, component_size)| {
                    if *component_size < *mcs {
                        keep_components.remove(*component);
                    }
                });
        }

        let node_ids_to_keep: Vec<NodeT> = components_vector
            .into_par_iter()
            .enumerate()
            .filter(|(_, component_id)| keep_components.contains(*component_id))
            .map(|(node_id, _)| node_id as NodeT)
            .collect();

        self.filter_from_ids(
            Some(node_ids_to_keep),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }
}
