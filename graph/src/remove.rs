use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # remove.
impl Graph {
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// # Arguments
    /// * `node_names`: Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types`: Option<Vec<Option<String>>> - The types of the nodes of which components to keep.
    /// * `edge_types`: Option<Vec<Option<String>>> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<NodeT> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<NodeT> - Optional, number of components to keep sorted by number of nodes.
    /// * `verbose`: bool - Whether to show the loading bar.
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_connected_component_ids(verbose);

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

            let pb = get_loading_bar(
                verbose,
                &format!(
                    "Computing which components are to keep for the graph {}",
                    &self.name
                ),
                self.get_directed_edges_number() as usize,
            );

            self.iter_edge_node_ids_and_edge_type_id(self.directed)
                .progress_with(pb)
                .for_each(|(_, src, dst, edge_type)| {
                    if edge_types_ids.contains(&edge_type) {
                        keep_components.insert(components_vector[src as usize]);
                        keep_components.insert(components_vector[dst as usize]);
                    }
                });
        }

        // Create the components counter
        let component_counts: Vec<(NodeT, NodeT)> =
            Counter::init(components_vector.clone()).most_common_ordered();

        // Insert the top k biggest components components
        if let Some(tkc) = top_k_components {
            for (i, (component_id, _)) in component_counts.iter().enumerate() {
                if i < tkc as usize {
                    keep_components.insert(*component_id);
                }
            }
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

        let pb = get_loading_bar(
            verbose,
            &format!(
                "Building edge list with only required components {}",
                &self.name
            ),
            self.get_directed_edges_number() as usize,
        );
        let pb_nodes = get_loading_bar(
            verbose,
            &format!(
                "Building node list with only required components {}",
                &self.name
            ),
            self.get_nodes_number() as usize,
        );

        let min_component_size = keep_components
            .iter()
            .map(|component_id| component_counts[component_id as usize].1)
            .min();

        Graph::from_string_sorted(
            self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                .progress_with(pb)
                .filter_map(
                    |(_, src, src_name, _, dst_name, _, edge_type_name, weight)| {
                        // we just check src because dst is trivially in the same component as src
                        match keep_components.contains(components_vector[src as usize]) {
                            true => Some(Ok((src_name, dst_name, edge_type_name, weight))),
                            false => None,
                        }
                    },
                ),
            Some(self.iter_node_names_and_node_type_names().progress_with(pb_nodes).filter_map(
                |(node_id, node_name, _, node_type_names)| {
                    match keep_components.contains(components_vector[node_id as usize]) {
                        true => Some(Ok((node_name, node_type_names))),
                        false => None,
                    }
                },
            )),
            self.directed,
            true,
            false,
            true,
            true,
            true,
            self.get_directed_edges_number() as usize, // Approximation of expected edges number.
            self.get_nodes_number(),                   // Approximation of expected nodes number.
            false,
            false,
            false,
            false,
            self.has_node_types(),
            self.has_edge_types(),
            self.has_edge_weights(),
            min_component_size.as_ref().map_or(true, |mcs| *mcs <= 1),
            self.has_singleton_nodes_with_selfloops()
                && min_component_size.as_ref().map_or(true, |mcs| *mcs <= 1),
            self.has_trap_nodes(),
            self.get_name(),
        )
    }
}
