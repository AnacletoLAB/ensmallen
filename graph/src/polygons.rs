use super::*;
use bitvec::prelude::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

impl Graph {
    /// Returns number of triangles in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    ///
    pub fn get_number_of_triangles(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<EdgeT> {
        let verbose = verbose.unwrap_or(true);

        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover = self.get_vertex_cover(
            Some(approach.unwrap_or("increasing_node_degree")), 
            Some(true),
            Some(insert_only_source.unwrap_or(true)), 
            None
        )?;

        let vertex_cover_size = vertex_cover.iter().filter(|cover| **cover).count();

        let pb = get_loading_bar(verbose, "Computing number of triangles", vertex_cover_size);

        let vertex_cover_reference = vertex_cover.as_slice();

        let is_multigraph = self.is_multigraph();

        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        Ok(vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(first, is_cover)| {
                if *is_cover {
                    Some(first as NodeT)
                } else {
                    None
                }
            })
            .progress_with(pb)
            // For each node in the cover
            .flat_map(|first| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let first_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(first)
                };

                let index = first_order_neighbours.partition_point(|&second| second < first);

                first_order_neighbours[..index]
                    .par_iter()
                    .filter_map(move |&second| {
                        if vertex_cover_reference[second as usize] {
                            Some((first, second, first_order_neighbours))
                        } else {
                            None
                        }
                    })
            })
            .map(|(first, second, first_order_neighbours)| {
                // We iterate over the neighbours
                // We compute the intersection of the neighbours.

                let mut first_neighbour_index = 0;
                let mut second_neighbour_index = 0;
                let mut partial_number_of_triangles: EdgeT = 0;

                let second_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(second)
                };

                while first_neighbour_index < first_order_neighbours.len()
                    && second_neighbour_index < second_order_neighbours.len()
                {
                    let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                    // If this is a self-loop, we march on forward

                    if first_order_neighbour == second || first_order_neighbour == first {
                        first_neighbour_index += 1;
                        continue;
                    }

                    // If this is not an intersection, we march forward
                    let second_order_neighbour = second_order_neighbours[second_neighbour_index];
                    if first_order_neighbour < second_order_neighbour {
                        first_neighbour_index += 1;
                        continue;
                    }
                    if first_order_neighbour > second_order_neighbour {
                        second_neighbour_index += 1;
                        continue;
                    }

                    // If we reach here, we are in an intersection.

                    let third = first_order_neighbour;

                    let mut first_multi_edge_counter = 1;
                    let mut second_multi_edge_counter = 1;

                    let factor = if is_multigraph {
                        while first_neighbour_index + 1 < first_order_neighbours.len()
                            && third == first_order_neighbours[first_neighbour_index + 1]
                        {
                            first_multi_edge_counter += 1;
                            first_neighbour_index += 1;
                        }

                        while second_neighbour_index + 1 < second_order_neighbours.len()
                            && third == second_order_neighbours[second_neighbour_index + 1]
                        {
                            second_multi_edge_counter += 1;
                            second_neighbour_index += 1;
                        }

                        first_multi_edge_counter * second_multi_edge_counter
                    } else {
                        1
                    };

                    first_neighbour_index += 1;
                    second_neighbour_index += 1;
                    // If the inner node is as well in the vertex cover
                    // we only count this as one, as we will encounter
                    // combinations of these nodes multiple times
                    // while iterating the vertex cover nodes
                    partial_number_of_triangles +=
                        if vertex_cover_reference[first_order_neighbour as usize] {
                            1 * factor
                        } else {
                            // Otherwise we won't encounter again this
                            // node and we need to count the triangles
                            // three times.
                            3 * factor
                        };
                }
                partial_number_of_triangles
            })
            .sum::<EdgeT>() / 3)
    }

    /// Returns number of squares in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_squares(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<EdgeT> {
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover = self.get_vertex_cover(approach, Some(true), insert_only_source, None)?;

        let vertex_cover_size = vertex_cover.iter().filter(|cover| **cover).count();

        let verbose = verbose.unwrap_or(true);

        let pb = get_loading_bar(verbose, "Computing number of squares", vertex_cover_size);

        let bitvecs = ThreadDataRaceAware::new(
            (0..rayon::current_num_threads())
                .map(|_| bitvec![u64, Lsb0; 0; self.get_number_of_nodes() as usize])
                .collect::<Vec<_>>(),
        );

        let vertex_cover_reference = vertex_cover.as_slice();

        let is_multigraph = self.is_multigraph();

        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        Ok(vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(first, is_cover)| {
                if *is_cover {
                    Some((first as NodeT, unsafe {
                        self.edges
                            .get_unchecked_neighbours_node_ids_from_src_node_id(first as NodeT)
                    }))
                } else {
                    None
                }
            })
            .progress_with(pb)
            .map(|(first, first_order_neighbours)|{
                let thread_id = rayon::current_thread_index().expect("current_thread_id not called from a rayon thread. This should not be possible because this is in a Rayon Thread Pool.");
                let bitvec = unsafe{&mut (&mut (*bitvecs.get()))[thread_id]};
                let mut partial_squares_number = 0;
                bitvec.fill(false);

                for &second in first_order_neighbours {
                    if first == second{
                        continue;
                    }
                    let second_order_neighbours = unsafe{self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(second as NodeT)};
                    for &third in second_order_neighbours {
                        if third >= first {
                            break;
                        }
                        if third == second{
                            continue;
                        }
                        if !vertex_cover_reference[third as usize] {
                            continue;
                        }
                        if unsafe{bitvec.replace_unchecked(third as usize, true)} {
                            continue;
                        }

                        let third_order_neighbours = unsafe{self.edges
                            .get_unchecked_neighbours_node_ids_from_src_node_id(third as NodeT)};
                        
                        let mut first_neighbour_index = 0;
                        let mut third_neighbour_index = 0;
                        let mut in_vertex_cover: EdgeT = 0;
                        let mut out_of_vertex_cover: EdgeT = 0;
                        let mut in_cover_summed_squares = 0;
                        let mut out_of_cover_summed_squares = 0;

                        while first_neighbour_index < first_order_neighbours.len()
                            && third_neighbour_index < third_order_neighbours.len()
                        {
                            let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                            // If this is a self-loop, we march on forward

                            if first_order_neighbour == third || first_order_neighbour == first {
                                first_neighbour_index += 1;
                                continue;
                            }

                            // If this is not an intersection, we march forward
                            let third_order_neighbour = third_order_neighbours[third_neighbour_index];
                            if first_order_neighbour < third_order_neighbour {
                                first_neighbour_index += 1;
                                continue;
                            }
                            if first_order_neighbour > third_order_neighbour {
                                third_neighbour_index += 1;
                                continue;
                            }

                            // If we reach here, we are in an intersection.

                            let fourth = first_order_neighbour;

                            let mut first_multi_edge_counter = 1;
                            let mut third_multi_edge_counter = 1;

                            let factor = if is_multigraph {
                                while first_neighbour_index + 1 < first_order_neighbours.len()
                                    && fourth == first_order_neighbours[first_neighbour_index + 1]
                                {
                                    first_multi_edge_counter += 1;
                                    first_neighbour_index += 1;
                                }

                                while third_neighbour_index + 1 < third_order_neighbours.len()
                                    && fourth == third_order_neighbours[third_neighbour_index + 1]
                                {
                                    third_multi_edge_counter += 1;
                                    third_neighbour_index += 1;
                                }

                                first_multi_edge_counter * third_multi_edge_counter
                            } else {
                                1
                            };

                            first_neighbour_index += 1;
                            third_neighbour_index += 1;

                            if vertex_cover_reference[fourth as usize] {
                                in_vertex_cover += factor;
                                in_cover_summed_squares += factor * factor;
                            } else {
                                // Otherwise we won't encounter again this node
                                out_of_vertex_cover += factor;
                                out_of_cover_summed_squares += factor * factor;
                            };
                        }

                        partial_squares_number += 
                            (out_of_vertex_cover + in_vertex_cover) * out_of_vertex_cover
                            + (2 * out_of_vertex_cover + in_vertex_cover) * in_vertex_cover / 2
                            - in_cover_summed_squares / 2
                            - out_of_cover_summed_squares;
                    }
                }
                partial_squares_number
            }).sum::<EdgeT>() / 2)
    }

    /// Returns number of squares in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_squares_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<EdgeT>> {
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover = self.get_vertex_cover(approach, Some(true), insert_only_source, None)?;

        let vertex_cover_size = vertex_cover.iter().filter(|cover| **cover).count();

        let node_squares_number = unsafe {
            std::mem::transmute::<Vec<EdgeT>, Vec<AtomicU64>>(vec![
                0;
                self.get_number_of_nodes()
                    as usize
            ])
        };

        let verbose = verbose.unwrap_or(true);

        let pb = get_loading_bar(verbose, "Computing number of squares", vertex_cover_size);

        let bitvecs = ThreadDataRaceAware::new(
            (0..rayon::current_num_threads())
                .map(|_| bitvec![u64, Lsb0; 0; self.get_number_of_nodes() as usize])
                .collect::<Vec<_>>(),
        );

        let vertex_cover_reference = vertex_cover.as_slice();

        let is_multigraph = self.is_multigraph();

        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(first, is_cover)| {
                if *is_cover {
                    Some((first as NodeT, unsafe {
                        self.edges
                            .get_unchecked_neighbours_node_ids_from_src_node_id(first as NodeT)
                    }))
                } else {
                    None
                }
            })
            .progress_with(pb)
            .for_each(|(first, first_order_neighbours)|{
                let thread_id = rayon::current_thread_index().expect("current_thread_id not called from a rayon thread. This should not be possible because this is in a Rayon Thread Pool.");
                let bitvec = unsafe{&mut (&mut (*bitvecs.get()))[thread_id]};
                bitvec.fill(false);

                let mut first_node_squares = 0;

                for &second in first_order_neighbours {
                    if first == second {
                        continue;
                    }

                    let second_order_neighbours = unsafe{self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(second as NodeT)};

                    for &third in second_order_neighbours {
                        if third >= first {
                            break;
                        }
                        if third == second{
                            continue;
                        }
                        if !vertex_cover_reference[third as usize] {
                            continue;
                        }
                        if unsafe{bitvec.replace_unchecked(third as usize, true)} {
                            continue;
                        }

                        let third_order_neighbours = unsafe{self.edges
                            .get_unchecked_neighbours_node_ids_from_src_node_id(third as NodeT)
                        };
                    
                        let mut first_neighbour_index = 0;
                        let mut third_neighbour_index = 0;
                        let mut in_vertex_cover: EdgeT = 0;
                        let mut out_of_vertex_cover: EdgeT = 0;

                        while first_neighbour_index < first_order_neighbours.len()
                            && third_neighbour_index < third_order_neighbours.len()
                        {
                            let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                            // If this is a self-loop, we march on forward

                            if first_order_neighbour == third || first_order_neighbour == first {
                                first_neighbour_index += 1;
                                continue;
                            }

                            // If this is not an intersection, we march forward
                            let third_order_neighbour = third_order_neighbours[third_neighbour_index];
                            if first_order_neighbour < third_order_neighbour {
                                first_neighbour_index += 1;
                                continue;
                            }
                            if first_order_neighbour > third_order_neighbour {
                                third_neighbour_index += 1;
                                continue;
                            }

                            // If we reach here, we are in an intersection.

                            let fourth = first_order_neighbour;

                            let mut first_multi_edge_counter = 1;
                            let mut third_multi_edge_counter = 1;

                            let factor = if is_multigraph {
                                while first_neighbour_index + 1 < first_order_neighbours.len()
                                    && fourth == first_order_neighbours[first_neighbour_index + 1]
                                {
                                    first_multi_edge_counter += 1;
                                    first_neighbour_index += 1;
                                }

                                while third_neighbour_index + 1 < third_order_neighbours.len()
                                    && fourth == third_order_neighbours[third_neighbour_index + 1]
                                {
                                    third_multi_edge_counter += 1;
                                    third_neighbour_index += 1;
                                }

                                first_multi_edge_counter * third_multi_edge_counter
                            } else {
                                1
                            };

                            first_neighbour_index += 1;
                            third_neighbour_index += 1;

                            if vertex_cover_reference[fourth as usize] {
                                in_vertex_cover += factor;
                            } else {
                                // Otherwise we won't encounter again this node
                                out_of_vertex_cover += factor;
                            };
                        }

                        first_neighbour_index = 0;
                        third_neighbour_index = 0;

                        while first_neighbour_index < first_order_neighbours.len()
                                && third_neighbour_index < third_order_neighbours.len()
                            {
                                let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                                // If this is a self-loop, we march on forward
    
                                if first_order_neighbour == third || first_order_neighbour == first {
                                    first_neighbour_index += 1;
                                    continue;
                                }
    
                                // If this is not an intersection, we march forward
                                let third_order_neighbour = third_order_neighbours[third_neighbour_index];
                                if first_order_neighbour < third_order_neighbour {
                                    first_neighbour_index += 1;
                                    continue;
                                }
                                if first_order_neighbour > third_order_neighbour {
                                    third_neighbour_index += 1;
                                    continue;
                                }
    
                                // If we reach here, we are in an intersection.
    
                                let fourth = first_order_neighbour;
    
                                let mut first_multi_edge_counter = 1;
                                let mut third_multi_edge_counter = 1;
    
                                let factor = if is_multigraph {
                                    while first_neighbour_index + 1 < first_order_neighbours.len()
                                        && fourth == first_order_neighbours[first_neighbour_index + 1]
                                    {
                                        first_multi_edge_counter += 1;
                                        first_neighbour_index += 1;
                                    }
    
                                    while third_neighbour_index + 1 < third_order_neighbours.len()
                                        && fourth == third_order_neighbours[third_neighbour_index + 1]
                                    {
                                        third_multi_edge_counter += 1;
                                        third_neighbour_index += 1;
                                    }
    
                                    first_multi_edge_counter * third_multi_edge_counter
                                } else {
                                    1
                                };
    
                                first_neighbour_index += 1;
                                third_neighbour_index += 1;
                                
                                node_squares_number[fourth as usize].fetch_add(if vertex_cover_reference[fourth as usize] {
                                    factor * (out_of_vertex_cover + in_vertex_cover - factor)
                                } else {
                                    factor * (2 * out_of_vertex_cover + in_vertex_cover - 2 * factor)
                                }, Ordering::Relaxed);
                            }

                            first_node_squares += out_of_vertex_cover * in_vertex_cover;
                            node_squares_number[third as usize].fetch_add(out_of_vertex_cover * in_vertex_cover, Ordering::Relaxed);
                        }
                    }
                    node_squares_number[first as usize].fetch_add(first_node_squares, Ordering::Relaxed);
                
            });
        Ok(unsafe { std::mem::transmute::<Vec<AtomicU64>, Vec<EdgeT>>(node_squares_number) })
    }

    /// Returns total number of triads in the graph without taking into account weights.
    pub fn get_number_of_triads(&self) -> EdgeT {
        self.par_iter_node_degrees()
            .map(|degree| (degree as EdgeT) * (degree.saturating_sub(1) as EdgeT))
            .sum()
    }

    /// Returns total number of triads in the weighted graph.
    pub fn get_number_of_weighted_triads(&self) -> Result<f64> {
        Ok(self
            .par_iter_weighted_node_degrees()?
            .map(|degree| {
                if degree > 1.0 {
                    degree * (degree - 1.0)
                } else {
                    0.0
                }
            })
            .sum())
    }

    /// Returns transitivity of the graph without taking into account weights.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn get_transitivity(&self, verbose: Option<bool>) -> f64 {
        self.get_number_of_triangles(None, None, verbose).unwrap() as f64
            / self.get_number_of_triads() as f64
    }

    /// Returns number of triangles for all nodes in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    ///
    pub fn get_number_of_triangles_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<EdgeT>> {
        let node_triangles_number = unsafe {
            std::mem::transmute::<Vec<EdgeT>, Vec<AtomicU64>>(vec![
                0;
                self.get_number_of_nodes()
                    as usize
            ])
        };

        let verbose = verbose.unwrap_or(true);

        let vertex_cover = self.get_vertex_cover(approach, None, insert_only_source, None)?;

        let cover_size = vertex_cover
            .par_iter()
            .filter(|&&is_cover| is_cover)
            .count();

        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles per node",
            cover_size,
        );

        let vertex_cover_reference = vertex_cover.as_slice();

        let is_multigraph = self.is_multigraph();

        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(node_id, is_cover)| {
                if *is_cover {
                    Some(node_id as NodeT)
                } else {
                    None
                }
            })
            .progress_with(pb)
            // For each node in the cover
            .flat_map(|first| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let first_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(first)
                };

                let index = first_order_neighbours.partition_point(|&second| second < first);

                first_order_neighbours[..index]
                    .par_iter()
                    .filter_map(move |&second| {
                        if vertex_cover_reference[second as usize] {
                            Some((first, second, first_order_neighbours))
                        } else {
                            None
                        }
                    })
            })
            .for_each(|(first, second, first_order_neighbours)| {
                // We iterate over the neighbours
                // We compute the intersection of the neighbours.

                let mut first_neighbour_index = 0;
                let mut second_neighbour_index = 0;

                let second_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(second)
                };

                let mut first_triangles = 0;
                let mut second_triangles = 0;

                while first_neighbour_index < first_order_neighbours.len()
                    && second_neighbour_index < second_order_neighbours.len()
                {
                    let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                    // If this is a self-loop, we march on forward
                    if first_order_neighbour == first || first_order_neighbour == second {
                        first_neighbour_index += 1;
                        continue;
                    }

                    // If this is not an intersection, we march forward
                    let second_order_neighbour = second_order_neighbours[second_neighbour_index];
                    if first_order_neighbour < second_order_neighbour {
                        first_neighbour_index += 1;
                        continue;
                    }
                    if first_order_neighbour > second_order_neighbour {
                        second_neighbour_index += 1;
                        continue;
                    }

                    // If we reach here, we are in an intersection.

                    let third = first_order_neighbour;

                    let mut first_multi_edge_counter = 1;
                    let mut second_multi_edge_counter = 1;

                    let factor = if is_multigraph {
                        while first_neighbour_index + 1 < first_order_neighbours.len()
                            && third == first_order_neighbours[first_neighbour_index + 1]
                        {
                            first_multi_edge_counter += 1;
                            first_neighbour_index += 1;
                        }

                        while second_neighbour_index + 1 < second_order_neighbours.len()
                            && third == second_order_neighbours[second_neighbour_index + 1]
                        {
                            second_multi_edge_counter += 1;
                            second_neighbour_index += 1;
                        }

                        first_multi_edge_counter * second_multi_edge_counter
                    } else {
                        1
                    };

                    first_neighbour_index += 1;
                    second_neighbour_index += 1;

                    // If the inner node is as well in the vertex cover
                    // we only count this as one, as we will encounter
                    // combinations of these nodes multiple times
                    // while iterating the vertex cover nodes
                    first_triangles += factor;
                    if !vertex_cover_reference[third as usize] {
                        // Otherwise we won't encounter again this
                        // node and we need to count the triangles
                        // three times.
                        second_triangles += factor;
                        node_triangles_number[third as usize].fetch_add(factor, Ordering::Relaxed);
                    }
                }
                node_triangles_number[first as usize].fetch_add(first_triangles, Ordering::Relaxed);
                node_triangles_number[second as usize]
                    .fetch_add(second_triangles, Ordering::Relaxed);
            });

        Ok(unsafe { std::mem::transmute::<Vec<AtomicU64>, Vec<EdgeT>>(node_triangles_number) })
    }

    /// Returns iterator over the clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    pub fn par_iter_clustering_coefficient_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> impl IndexedParallelIterator<Item = f64> + '_ {
        self.get_number_of_triangles_per_node(approach, insert_only_source, verbose)
            .unwrap()
            .into_par_iter()
            .zip(self.par_iter_node_degrees())
            .map(|(triangles_number, degree)| {
                if degree <= 1 {
                    0.0
                } else {
                    triangles_number as f64 / ((degree as EdgeT) * (degree as EdgeT - 1)) as f64
                }
            })
    }

    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    pub fn get_clustering_coefficient_per_node(
        &self, 
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>
    ) -> Vec<f64> {
        self.par_iter_clustering_coefficient_per_node(approach, insert_only_source, verbose)
            .collect()
    }

    /// Returns the graph clustering coefficient.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    pub fn get_clustering_coefficient(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>
    ) -> f64 {
        self.par_iter_clustering_coefficient_per_node(
            approach,
            insert_only_source,
            verbose
        ).sum()
    }

    /// Returns the graph average clustering coefficient.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the increasing node degree order is used.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Parallel Triangles and Squares Count for Multigraphs Using Vertex Covers"](https://davidbader.net/publication/2023-cfgb/2023-cfgb.pdf).
    pub fn get_average_clustering_coefficient(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>
    ) -> f64 {
        self.get_clustering_coefficient(
            approach,
            insert_only_source,
            verbose
        ) / self.get_number_of_nodes() as f64
    }
}
