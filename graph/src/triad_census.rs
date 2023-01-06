use super::types::*;
use super::Graph;
use crate::manual_binding;
use num_traits::One;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};
use std::sync::atomic::{AtomicU64, Ordering};

// These are tricodes for triads with at least 2 edges between 3 different nodes
// These fold into the same code triads that are isomorphic to each-other.
// These will break if queried with triads with less than 2 edges.
// The encoding comes from the paper: [Node-Specific Triad Pattern Mining for Complex-Network Analysis](https://arxiv.org/pdf/1410.1594.pdf)
const BASE_13_TRICODES: [u8; 64] = [
    255, 255, 255, 255, 255, 0, 1, 2, 255, 1, 3, 6, 255, 2, 6, 7, 255, 1, 0, 2, 3, 4, 4, 5, 2, 8,
    4, 9, 6, 9, 10, 11, 255, 3, 1, 6, 1, 4, 8, 9, 1, 4, 4, 10, 2, 5, 9, 11, 255, 2, 2, 7, 6, 10, 9,
    11, 2, 9, 5, 11, 7, 11, 11, 12,
];

// These are tricodes for generic triads.
// These fold into the same code triads that are isomorphic to each-other.
// These will break if queried with triads with less than 2 edges.
// The encoding comes from the paper: [Node-Specific Triad Pattern Mining for Complex-Network Analysis](https://arxiv.org/pdf/1410.1594.pdf)
const BASE_16_TRICODES: [u8; 64] = [
    0, 1, 1, 2, 1, 3, 5, 7, 1, 5, 4, 6, 2, 7, 6, 10, 1, 5, 3, 7, 4, 8, 8, 12, 5, 9, 8, 13, 6, 13,
    11, 14, 1, 4, 5, 6, 5, 8, 9, 13, 3, 8, 8, 11, 7, 12, 13, 14, 2, 6, 7, 10, 6, 11, 13, 14, 7, 13,
    12, 14, 10, 14, 14, 15,
];

// These are tricodes for triads with at least 2 edges between 3 different nodes.
// These include also triads that are isomorphic to each-other.
// These will break if queried with triads with less than 2 edges.
// The encoding comes from the paper: [Node-Specific Triad Pattern Mining for Complex-Network Analysis](https://arxiv.org/pdf/1410.1594.pdf)
const BASE_30_TRICODES: [u8; 64] = [
    255, 255, 255, 255, 255, 20, 9, 21, 255, 9, 10, 11, 255, 21, 11, 27, 255, 0, 1, 2, 3, 22, 15,
    23, 4, 12, 13, 14, 5, 24, 16, 28, 255, 3, 4, 5, 0, 22, 12, 24, 1, 15, 13, 16, 2, 23, 14, 28,
    255, 6, 7, 8, 6, 25, 17, 26, 7, 17, 18, 19, 8, 26, 19, 29,
];

// These are tricodes for generic triads.
const BASE_64_TRICODES: [u8; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
];

/// # Triad census algorithm
impl Graph {
    unsafe fn get_unchecked_single_tricode_from_node_ids<T: std::ops::Index<usize, Output = u8>>(
        &self,
        first_to_second: usize,
        second_to_first: usize,
        first_to_third: usize,
        third_to_first: usize,
        second_to_third: usize,
        third_to_second: usize,
        map: T,
    ) -> u8 {
        map[first_to_second
            + 2 * (second_to_first
                + 2 * (first_to_third
                    + 2 * (third_to_first + 2 * (second_to_third + 2 * third_to_second))))]
    }

    /// Returns the tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    /// * `map`: T - The map to use for the associated tricode.
    ///
    /// # Returns
    /// Triple with tricode for the following triads orders:
    /// * (first, second, third)
    /// * (second, third, first)
    /// * (third, first, second)
    /// This is done to avoid having to look up the existance of the various
    /// edges multiple times.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    unsafe fn get_unchecked_tricodes_from_node_ids<T: std::ops::Index<usize, Output = u8> + Copy>(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
        map: T,
    ) -> (u8, u8, u8) {
        let first_to_second = self.has_edge_from_node_ids(first, second) as usize;
        let second_to_first = if self.is_directed() {
            self.has_edge_from_node_ids(second, first) as usize
        } else {
            first_to_second
        };
        let first_to_third = self.has_edge_from_node_ids(first, third) as usize;
        let third_to_first = if self.is_directed() {
            self.has_edge_from_node_ids(third, first) as usize
        } else {
            first_to_third
        };
        let second_to_third = self.has_edge_from_node_ids(second, third) as usize;
        let third_to_second = if self.is_directed() {
            self.has_edge_from_node_ids(third, second) as usize
        } else {
            second_to_third
        };

        (
            self.get_unchecked_single_tricode_from_node_ids(
                first_to_second,
                second_to_first,
                first_to_third,
                third_to_first,
                second_to_third,
                third_to_second,
                map,
            ),
            self.get_unchecked_single_tricode_from_node_ids(
                second_to_third,
                third_to_second,
                second_to_first,
                first_to_second,
                third_to_first,
                first_to_third,
                map,
            ),
            self.get_unchecked_single_tricode_from_node_ids(
                third_to_first,
                first_to_third,
                third_to_second,
                second_to_third,
                first_to_second,
                second_to_first,
                map,
            ),
        )
    }

    /// Returns the base 16 tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    /// * `map`: T - The map to use for the associated tricode.
    ///
    /// # Returns
    /// Triple with tricode for the following triads orders:
    /// * (first, second, third)
    /// * (second, third, first)
    /// * (third, first, second)
    /// This is done to avoid having to look up the existance of the various
    /// edges multiple times.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    unsafe fn get_unchecked_base_16_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> (u8, u8, u8) {
        self.get_unchecked_tricodes_from_node_ids(first, second, third, BASE_16_TRICODES)
    }

    /// Returns the base 13 tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    /// * `map`: T - The map to use for the associated tricode.
    ///
    /// # Returns
    /// Triple with tricode for the following triads orders:
    /// * (first, second, third)
    /// * (second, third, first)
    /// * (third, first, second)
    /// This is done to avoid having to look up the existance of the various
    /// edges multiple times.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    unsafe fn get_unchecked_base_13_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> (u8, u8, u8) {
        self.get_unchecked_tricodes_from_node_ids(first, second, third, BASE_13_TRICODES)
    }

    /// Returns the base 30 tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    /// * `map`: T - The map to use for the associated tricode.
    ///
    /// # Returns
    /// Triple with tricode for the following triads orders:
    /// * (first, second, third)
    /// * (second, third, first)
    /// * (third, first, second)
    /// This is done to avoid having to look up the existance of the various
    /// edges multiple times.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    unsafe fn get_unchecked_base_30_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> (u8, u8, u8) {
        self.get_unchecked_tricodes_from_node_ids(first, second, third, BASE_30_TRICODES)
    }

    /// Returns the base 64 tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    /// * `map`: T - The map to use for the associated tricode.
    ///
    /// # Returns
    /// Triple with tricode for the following triads orders:
    /// * (first, second, third)
    /// * (second, third, first)
    /// * (third, first, second)
    /// This is done to avoid having to look up the existance of the various
    /// edges multiple times.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    unsafe fn get_unchecked_base_64_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> (u8, u8, u8) {
        self.get_unchecked_tricodes_from_node_ids(first, second, third, BASE_64_TRICODES)
    }

    /// Returns the base 16, i.e. using 16 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node IDs,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_16_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> Result<(u8, u8, u8)> {
        self.validate_node_id(first)?;
        self.validate_node_id(second)?;
        self.validate_node_id(third)?;
        Ok(unsafe { self.get_unchecked_base_16_tricodes_from_node_ids(first, second, third) })
    }

    /// Returns the base 13, i.e. using 13 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node IDs,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_13_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> Result<(u8, u8, u8)> {
        self.validate_node_id(first)?;
        self.validate_node_id(second)?;
        self.validate_node_id(third)?;
        Ok(unsafe { self.get_unchecked_base_13_tricodes_from_node_ids(first, second, third) })
    }

    /// Returns the base 30, i.e. using 30 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node IDs,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_30_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> Result<(u8, u8, u8)> {
        self.validate_node_id(first)?;
        self.validate_node_id(second)?;
        self.validate_node_id(third)?;
        Ok(unsafe { self.get_unchecked_base_30_tricodes_from_node_ids(first, second, third) })
    }

    /// Returns the base 64, i.e. using 64 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node IDs,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_64_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> Result<(u8, u8, u8)> {
        self.validate_node_id(first)?;
        self.validate_node_id(second)?;
        self.validate_node_id(third)?;
        Ok(unsafe { self.get_unchecked_base_64_tricodes_from_node_ids(first, second, third) })
    }

    /// Returns the base 16, i.e. using 16 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// # Arguments
    /// * `first`: &str - The first node name of the triple.
    /// * `second`: &str - The second node name of the triple.
    /// * `third`: &str - The third node name of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node names,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_16_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(u8, u8, u8)> {
        Ok(unsafe {
            self.get_unchecked_base_16_tricodes_from_node_ids(
                self.get_node_id_from_node_name(first)?,
                self.get_node_id_from_node_name(second)?,
                self.get_node_id_from_node_name(third)?,
            )
        })
    }

    /// Returns the base 13, i.e. using 13 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// # Arguments
    /// * `first`: &str - The first node name of the triple.
    /// * `second`: &str - The second node name of the triple.
    /// * `third`: &str - The third node name of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node names,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_13_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(u8, u8, u8)> {
        Ok(unsafe {
            self.get_unchecked_base_13_tricodes_from_node_ids(
                self.get_node_id_from_node_name(first)?,
                self.get_node_id_from_node_name(second)?,
                self.get_node_id_from_node_name(third)?,
            )
        })
    }

    /// Returns the base 30, i.e. using 30 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// # Arguments
    /// * `first`: &str - The first node name of the triple.
    /// * `second`: &str - The second node name of the triple.
    /// * `third`: &str - The third node name of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node names,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_30_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(u8, u8, u8)> {
        Ok(unsafe {
            self.get_unchecked_base_30_tricodes_from_node_ids(
                self.get_node_id_from_node_name(first)?,
                self.get_node_id_from_node_name(second)?,
                self.get_node_id_from_node_name(third)?,
            )
        })
    }

    /// Returns the base 64, i.e. using 64 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// # Arguments
    /// * `first`: &str - The first node name of the triple.
    /// * `second`: &str - The second node name of the triple.
    /// * `third`: &str - The third node name of the triple.
    ///
    /// # Raises
    /// ValueError: If any of the three provided node names,
    /// namely `first`, `second` and `third`, are not within the maximum
    /// number of nodes in this graph.
    pub fn get_base_64_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(u8, u8, u8)> {
        Ok(unsafe {
            self.get_unchecked_base_64_tricodes_from_node_ids(
                self.get_node_id_from_node_name(first)?,
                self.get_node_id_from_node_name(second)?,
                self.get_node_id_from_node_name(third)?,
            )
        })
    }

    /// Returns slice with graph-wide triad census defined over 16 type of triads.
    ///
    /// # References
    /// The sequential version of this algorithm is described in the
    /// following paper: https://www.sciencedirect.com/science/article/pii/S0378873301000351?casa_token=Ir4wzRNpoeIAAAAA:ogvSkg8pC6MjRxhsFCbjX4klauMEIclNTHaVCxv7rlU45ENprX4XGtkaVSOTYmjLwmI4xTIp
    pub fn get_base_16_triad_census(&self) -> [u64; 16] {
        let number_of_nodes: u64 = self.get_number_of_nodes() as u64;
        let mut census = self
            .par_iter_node_ids()
            .flat_map(|first| {
                let first_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(first)
                };
                first_order_neighbours
                    .par_iter()
                    .copied()
                    .filter_map(move |second| {
                        if first < second {
                            Some((
                                (first, first_order_neighbours),
                                (second, unsafe {
                                    self.edges
                                        .get_unchecked_neighbours_node_ids_from_src_node_id(second)
                                }),
                            ))
                        } else {
                            None
                        }
                    })
            })
            .map(
                |((first, first_order_neighbours), (second, second_order_neighbours))| {
                    let mut first_index = 0;
                    let mut second_index = 0;
                    let mut census: [u64; 16] = [0; 16];
                    let mut union_cardinality: u64 = 0;

                    while first_index < first_order_neighbours.len()
                        && second_index < second_order_neighbours.len()
                    {
                        let first_order_neighbour = first_order_neighbours[first_index];
                        let second_order_neighbour = second_order_neighbours[second_index];

                        let first_collision =
                            first_order_neighbour == first || first_order_neighbour == second;
                        let second_collision =
                            second_order_neighbour == first || second_order_neighbour == second;

                        if first_collision {
                            first_index += 1;
                        }

                        if second_collision {
                            second_index += 1;
                        }

                        if first_collision || second_collision {
                            continue;
                        }

                        if first_order_neighbour == second_order_neighbour {
                            first_index += 1;
                            second_index += 1;
                        } else if first_order_neighbour < second_order_neighbour {
                            first_index += 1;
                        } else {
                            second_index += 1;
                        }

                        let third = first_order_neighbour;
                        union_cardinality += 1;

                        if second < third
                            || (first < third
                                && third < second
                                && !self.has_edge_from_node_ids(first, third))
                        {
                            let tricode = unsafe {
                                self.get_unchecked_base_16_tricodes_from_node_ids(
                                    first, second, third,
                                )
                                .0
                            } as usize;
                            census[tricode] += 1;
                        }
                    }

                    let tritype =
                        if !self.is_directed() || self.has_edge_from_node_ids(second, first) {
                            2
                        } else {
                            1
                        };

                    census[tritype] += number_of_nodes - union_cardinality - 2;

                    census
                },
            )
            .reduce(
                || [0; 16],
                |mut a, b| {
                    a.iter_mut().zip(b.iter().copied()).for_each(|(a, b)| {
                        *a += b;
                    });
                    a
                },
            );
        let total_triads = census[1..].iter().copied().sum::<u64>();
        census[0] =
            number_of_nodes * (number_of_nodes - 1) * (number_of_nodes - 2) / 6 - total_triads;
        census
    }

    /// Returns slice with graph-wide triad census defined over variable set of triads with 3 connected nodes.
    ///
    /// # References
    /// The sequential version of this algorithm is described in the
    /// following paper: https://arxiv.org/pdf/1410.1594.pdf
    unsafe fn get_triad_census<
        T: IndexMut<usize>
            + Index<usize>
            + Default
            + Send
            + std::iter::IntoIterator<Item = <T as Index<usize>>::Output>,
    >(
        &self,
        triadic_code: fn(&Self, NodeT, NodeT, NodeT) -> (u8, u8, u8),
    ) -> T
    where
        <T as Index<usize>>::Output:
            core::ops::AddAssign<<T as Index<usize>>::Output> + Sized + One,
    {
        self.par_iter_node_ids()
            .flat_map(|first| {
                let first_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(first)
                };
                first_order_neighbours
                    .par_iter()
                    .copied()
                    .filter_map(move |second| {
                        if first < second {
                            Some((
                                (first, first_order_neighbours),
                                (second, unsafe {
                                    self.edges
                                        .get_unchecked_neighbours_node_ids_from_src_node_id(second)
                                }),
                            ))
                        } else {
                            None
                        }
                    })
            })
            .map(
                |((first, first_order_neighbours), (second, second_order_neighbours))| {
                    let mut first_index = 0;
                    let mut second_index = 0;
                    let mut census: T = T::default();

                    while first_index < first_order_neighbours.len()
                        && second_index < second_order_neighbours.len()
                    {
                        let first_order_neighbour = first_order_neighbours[first_index];
                        let second_order_neighbour = second_order_neighbours[second_index];

                        let first_collision =
                            first_order_neighbour == first || first_order_neighbour == second;
                        let second_collision =
                            second_order_neighbour == first || second_order_neighbour == second;

                        if first_collision {
                            first_index += 1;
                        }

                        if second_collision {
                            second_index += 1;
                        }

                        if first_collision || second_collision {
                            continue;
                        }

                        if first_order_neighbour == second_order_neighbour {
                            first_index += 1;
                            second_index += 1;
                        } else if first_order_neighbour < second_order_neighbour {
                            first_index += 1;
                        } else {
                            second_index += 1;
                        }

                        let third = first_order_neighbour;

                        if second < third
                            || (first < third
                                && third < second
                                && !self.has_edge_from_node_ids(first, third))
                        {
                            let (first_tricode, second_tricode, third_tricode) =
                                triadic_code(&self, first, second, third);
                            census[first_tricode as usize] += <T as Index<usize>>::Output::one();
                            census[second_tricode as usize] += <T as Index<usize>>::Output::one();
                            census[third_tricode as usize] += <T as Index<usize>>::Output::one();
                        }
                    }

                    census
                },
            )
            .reduce(
                || T::default(),
                |mut a, b| {
                    b.into_iter().enumerate().for_each(|(i, b)| {
                        a[i] += b;
                    });
                    a
                },
            )
    }

    /// Returns slice with triad census per node defined over variable set of triads with 3 connected nodes.
    ///
    /// # References
    /// The sequential version of this algorithm is described in the
    /// following paper: https://arxiv.org/pdf/1410.1594.pdf
    unsafe fn get_triad_census_per_node<
        T: IndexMut<usize>
            + Index<usize, Output = u64>
            + Default
            + Send
            + std::iter::IntoIterator<Item = <T as Index<usize>>::Output>,
    >(
        &self,
        tradic_census: &mut [u64],
        triadic_code: fn(&Self, NodeT, NodeT, NodeT) -> (u8, u8, u8),
    ) -> Result<()> {
        let dimensionality: usize = T::default().into_iter().count();
        if tradic_census.len() != dimensionality * self.get_number_of_nodes() as usize {
            return Err(format!(
                concat!(
                    "You have provided a slice with size {size}, ",
                    "but the current triad census has dimensionality {dimensionality}, ",
                    "and the graph has {number_of_nodes} nodes. Please provide ",
                    "a slice with exactly {expected_size} elements."
                ),
                size = tradic_census.len(),
                dimensionality = dimensionality,
                number_of_nodes = self.get_number_of_nodes(),
                expected_size = dimensionality * self.get_number_of_nodes() as usize
            ));
        }

        let tradic_census =
            unsafe { core::mem::transmute::<&mut [u64], &[AtomicU64]>(tradic_census) };

        self.par_iter_node_ids()
            .zip(tradic_census.par_chunks(dimensionality))
            .flat_map(|(first, first_node_census)| {
                let first_order_neighbours = unsafe {
                    self.edges
                        .get_unchecked_neighbours_node_ids_from_src_node_id(first)
                };
                first_order_neighbours
                    .par_iter()
                    .copied()
                    .filter_map(move |second| {
                        if first < second {
                            Some((
                                (first, first_order_neighbours, first_node_census),
                                (
                                    second,
                                    unsafe {
                                        self.edges
                                            .get_unchecked_neighbours_node_ids_from_src_node_id(
                                                second,
                                            )
                                    },
                                    &tradic_census[dimensionality * second as usize
                                        ..dimensionality * (second + 1) as usize],
                                ),
                            ))
                        } else {
                            None
                        }
                    })
            })
            .for_each(
                |(
                    (first, first_order_neighbours, first_node_census),
                    (second, second_order_neighbours, second_node_census),
                )| {
                    let mut first_index = 0;
                    let mut second_index = 0;
                    let mut first_census: T = T::default();
                    let mut second_census: T = T::default();

                    while first_index < first_order_neighbours.len()
                        && second_index < second_order_neighbours.len()
                    {
                        let first_order_neighbour = first_order_neighbours[first_index];
                        let second_order_neighbour = second_order_neighbours[second_index];

                        let first_collision =
                            first_order_neighbour == first || first_order_neighbour == second;
                        let second_collision =
                            second_order_neighbour == first || second_order_neighbour == second;

                        if first_collision {
                            first_index += 1;
                        }

                        if second_collision {
                            second_index += 1;
                        }

                        if first_collision || second_collision {
                            continue;
                        }

                        if first_order_neighbour == second_order_neighbour {
                            first_index += 1;
                            second_index += 1;
                        } else if first_order_neighbour < second_order_neighbour {
                            first_index += 1;
                        } else {
                            second_index += 1;
                        }

                        let third = first_order_neighbour;

                        if second < third
                            || (first < third
                                && third < second
                                && !self.has_edge_from_node_ids(first, third))
                        {
                            let (first_tricode, second_tricode, third_tricode) =
                                triadic_code(&self, first, second, third);
                            tradic_census[dimensionality * third as usize + third_tricode as usize]
                                .fetch_add(1, Ordering::Relaxed);
                            first_census[first_tricode as usize] += 1;
                            second_census[second_tricode as usize] += 1;
                        }
                    }
                    for (census, census_atomics) in [
                        (first_census, first_node_census),
                        (second_census, second_node_census),
                    ] {
                        census.into_iter().zip(census_atomics.iter()).for_each(
                            |(count, census)| {
                                census.fetch_add(count, Ordering::Relaxed);
                            },
                        );
                    }
                },
            );
        Ok(())
    }

    pub fn get_base_13_triad_census(&self) -> [u64; 13] {
        unsafe {
            self.get_triad_census(|graph, first, second, third| {
                graph.get_unchecked_base_13_tricodes_from_node_ids(first, second, third)
            })
        }
    }

    pub fn get_base_30_triad_census(&self) -> [u64; 30] {
        unsafe {
            self.get_triad_census(|graph, first, second, third| {
                graph.get_unchecked_base_30_tricodes_from_node_ids(first, second, third)
            })
        }
    }

    #[manual_binding]
    pub fn get_base_13_triad_census_per_node(&self, tradic_census: &mut [u64]) -> Result<()> {
        unsafe {
            self.get_triad_census_per_node::<[u64; 13]>(
                tradic_census,
                |graph, first, second, third| {
                    graph.get_unchecked_base_13_tricodes_from_node_ids(first, second, third)
                },
            )
        }
    }

    #[manual_binding]
    pub fn get_base_30_triad_census_per_node(&self, tradic_census: &mut [u64]) -> Result<()> {
        unsafe {
            self.get_triad_census_per_node::<[u64; 30]>(
                tradic_census,
                |graph, first, second, third| {
                    graph.get_unchecked_base_30_tricodes_from_node_ids(first, second, third)
                },
            )
        }
    }

    // unsafe fn get_unchecked_triad_census_from_node_id(
    //     &self,
    //     node_id: NodeT
    // ) {
    //     self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id);
    // }

    // pub fn get_triad_census(&self, node_id: NodeT) -> Result<[u64; 16]> {
    //     self.validate_node_id(node_id)?;
    //     let mut triad_census = [0; 16];
    //     unsafe { self.get_unchecked_triad_census_from_node_id(node_id, &mut triad_census) };
    //     Ok(triad_census)
    // }
}
