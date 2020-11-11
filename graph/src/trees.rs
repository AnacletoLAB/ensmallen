use super::*;
use indicatif::ProgressIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use vec_rand::xorshift::xorshift as rand_u64;

// Return component of given node, including eventual remapping.
fn get_node_component(component: usize, components_remapping: &HashMap<usize, usize>) -> usize {
    match components_remapping.get(&component) {
        Some(c) => *c,
        None => component,
    }
}

/// # Implementation of algorithms relative to trees.
impl Graph {
    fn iter_edges_from_random_state(
        &self,
        random_state: u64,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        let edges_number = self.get_edges_number();
        // We execute two times the xorshift to improve the randomness of the seed.
        let updated_random_state = rand_u64(rand_u64(random_state ^ SEED_XOR as u64));
        (updated_random_state..edges_number + updated_random_state).filter_map(move |i| {
            let edge_id = i % edges_number;
            let (src, dst) = self.get_edge_from_edge_id(edge_id);
            match src == dst || !self.directed && src > dst {
                true => None,
                false => Some((edge_id, src, dst)),
            }
        })
    }

    fn iter_on_edges_with_preference<'a>(
        &'a self,
        random_state: u64,
        verbose: bool,
        unwanted_edge_types: &'a Option<HashSet<EdgeTypeT>>,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + 'a {
        // TODO! FIX THIS CRASH if called with unwanted_edge_types and the graph does not have edge types.
        let result: Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)>> =
            if let Some(uet) = unwanted_edge_types {
                Box::new(
                    self.iter_edges_from_random_state(random_state)
                        .filter(move |(edge_id, _, _)| {
                            !uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                        })
                        .chain(self.iter_edges_from_random_state(random_state).filter(
                            move |(edge_id, _, _)| {
                                uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                            },
                        )),
                )
            } else {
                Box::new(self.iter_edges_from_random_state(random_state))
            };

        let pb = get_loading_bar(
            verbose,
            format!("Building spanning tree for {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );
        result.progress_with(pb)
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `random_state`:NodeT - The random_state to use for the holdout,
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `unwanted_edge_types`: &Option<HashSet<EdgeTypeT>> - Which edge types id to try to avoid.
    /// * `verbose`: bool - Wethever to show a loading bar or not.
    ///
    pub fn random_spanning_tree(
        &self,
        random_state: EdgeT,
        include_all_edge_types: bool,
        unwanted_edge_types: &Option<HashSet<EdgeTypeT>>,
        verbose: bool,
    ) -> (RoaringTreemap, Vec<RoaringBitmap>) {
        // Create vector of sets of the single nodes.
        let mut components: Vec<Option<RoaringBitmap>> = Vec::new();
        // Create vector of nodes component numbers.
        let mut nodes_components: Vec<Option<usize>> = vec![None; self.get_nodes_number() as usize];
        // Create the empty tree (this will be sparse on most graphs so roaring can save memory).
        let mut tree = RoaringTreemap::new();
        // Components remapping
        let mut components_remapping: HashMap<usize, usize> = HashMap::new();

        // Iterate over all the edges and add and edge to the mst
        // iff the edge create, expand or merge components.
        for (edge_id, src, dst) in
            self.iter_on_edges_with_preference(random_state, verbose, unwanted_edge_types)
        {
            let mut update_tree = false;
            let src_component = nodes_components[src as usize];
            let dst_component = nodes_components[dst as usize];
            // if both nodes are not covered then the edge is isolated
            // and must start its own component
            match (src_component, dst_component) {
                (None, None) => {
                    update_tree = true;
                    nodes_components[src as usize] = Some(components.len());
                    nodes_components[dst as usize] = Some(components.len());
                    components.push(Some(RoaringBitmap::from_iter(vec![src, dst])));
                }
                (Some(src_component), Some(dst_component)) => {
                    // if the components are different then we add it because it will merge them
                    if src_component == dst_component {
                        continue;
                    }
                    let src_component = get_node_component(src_component, &components_remapping);
                    let dst_component = get_node_component(dst_component, &components_remapping);
                    if src_component != dst_component {
                        let removed_component = components[src_component].clone().unwrap();
                        components[src_component] = None;
                        if let Some(component) = &mut components[dst_component] {
                            component.union_with(&removed_component);
                        }
                        components_remapping.par_iter_mut().for_each(
                            |(component, remapped_component)| {
                                if *component == src_component
                                    || *remapped_component == src_component
                                {
                                    *remapped_component = dst_component;
                                }
                            },
                        );
                        components_remapping.insert(src_component, dst_component);
                        update_tree = true;
                    }
                }
                _ => {
                    let (inserted_component, not_inserted, not_inserted_component) =
                        if src_component.is_some() {
                            (src_component, dst, &mut nodes_components[dst as usize])
                        } else {
                            (dst_component, src, &mut nodes_components[src as usize])
                        };
                    let inserted_component =
                        get_node_component(inserted_component.unwrap(), &components_remapping);
                    if let Some(component) = &mut components[inserted_component] {
                        component.insert(not_inserted);
                    }
                    *not_inserted_component = Some(inserted_component);
                    update_tree = true;
                }
            };

            if update_tree {
                tree.extend(self.compute_edge_ids_vector(edge_id, src, dst, include_all_edge_types))
            }
        }

        let components = components.iter().filter_map(|c| c.clone()).collect();

        (tree, components)
    }

    /// Returns set of edges composing a spanning tree and connected components.
    pub fn spanning_tree(&self) -> Vec<(NodeT, NodeT)> {
        let nodes_number = self.get_nodes_number();
        let colors = (0..nodes_number)
            .map(|_| AtomicU16::new(0))
            .collect::<Vec<AtomicU16>>();
        let parents = (0..nodes_number)
            .map(|_| AtomicU32::new(nodes_number))
            .collect::<Vec<AtomicU32>>();
        let cpus = (1..(num_cpus::get() as u16 + 1)).collect::<Vec<u16>>();
        loop {
            let roots = colors
                .iter()
                .enumerate()
                .filter_map(|(node_id, color)| {
                    if self.is_singleton(node_id as NodeT) {
                        colors[node_id as usize].store(1, Ordering::SeqCst);
                    } else if color.load(Ordering::SeqCst) == 0 {
                        return Some(node_id as NodeT);
                    }
                    None
                })
                .take(cpus.len())
                .collect::<Vec<NodeT>>();

            if roots.is_empty() {
                break;
            }

            cpus.par_iter()
                .zip(roots.par_iter())
                .for_each(|(color, root)| {
                    colors[*root as usize].store(*color, Ordering::SeqCst);
                    let mut stack: Vec<NodeT> = vec![*root];
                    while !stack.is_empty() {
                        let src = stack.pop().unwrap();
                        self.get_source_destinations_range(src).for_each(|dst| {
                            if colors[dst as usize].load(Ordering::SeqCst) == 0 {
                                colors[dst as usize].store(*color, Ordering::SeqCst);
                                parents[dst as usize].store(src, Ordering::SeqCst);
                                stack.push(dst);
                            }
                        });
                    }
                });
        }
        parents
            .iter()
            .enumerate()
            .filter_map(|(dst, src)| {
                if src.load(Ordering::SeqCst) != nodes_number {
                    return Some((src.load(Ordering::SeqCst), dst as NodeT));
                }
                None
            })
            .collect::<Vec<(NodeT, NodeT)>>()
    }
}
