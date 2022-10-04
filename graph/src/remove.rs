use crate::constructors::build_graph_from_strings_without_type_iterators;

use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use rayon::iter::ParallelIterator;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # remove.
impl Graph {
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// # Arguments
    /// * `node_names`: Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types`: Option<&[Option<&str>]> - The types of the nodes of which components to keep.
    /// * `edge_types`: Option<&[Option<&str>]> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<NodeT> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<NodeT> - Optional, number of components to keep sorted by number of nodes.
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<&[Option<&str>]>,
        edge_types: Option<&[Option<&str>]>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        let verbose = verbose.unwrap_or(false);
        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_connected_component_ids(Some(verbose));

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
                self.get_number_of_directed_edges() as usize,
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

        let nodes_iterator: ItersWrapper<_, _, rayon::iter::Empty<_>> =
            ItersWrapper::Sequential(self.iter_node_names_and_node_type_names().filter_map(
                |(node_id, node_name, _, node_type_names)| {
                    match keep_components.contains(components_vector[node_id as usize]) {
                        // We put as row 0 as it will not be dense because of the filter
                        // It may be possible to get it to be dense with the proper offsets
                        true => Some(Ok((0, (node_name, node_type_names)))),
                        false => None,
                    }
                },
            ));

        let edges_iterator: ItersWrapper<_, std::iter::Empty<_>, _> = ItersWrapper::Parallel(
            self.par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight()
                .filter_map(
                    |(_, src, src_name, _, dst_name, _, edge_type_name, weight)| {
                        // we just check src because dst is trivially in the same component as src
                        match keep_components.contains(components_vector[src as usize]) {
                            true => Some(Ok((
                                0,
                                (
                                    src_name,
                                    dst_name,
                                    edge_type_name,
                                    weight.unwrap_or(WeightT::NAN),
                                ),
                            ))),
                            false => None,
                        }
                    },
                ),
        );

        build_graph_from_strings_without_type_iterators(
            self.has_node_types(),
            Some(nodes_iterator),
            None,
            true,
            false,
            false,
            None,
            self.has_edge_types(),
            Some(edges_iterator),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(true),
            Some(false),
            // Even though the edges are sortof
            // sorted, the filtering procedure makes
            // it impossible to actually know the edge ID
            // of each edge, and therefore it is not possible
            // to construct the graph in parallel directly.
            Some(false),
            None,
            None,
            None,
            None,
            None,
            true,
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
    }
}
