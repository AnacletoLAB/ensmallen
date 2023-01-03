use super::types::*;
use super::Graph;
use rayon::prelude::*;

const TRYCODES: [u8; 64] = [
    0, 1, 1, 2, 1, 3, 5, 7, 1, 5, 4, 6, 2, 7, 6, 10, 1, 5, 3, 7, 4, 8, 8, 12, 5, 9, 8, 13, 6, 13,
    11, 14, 1, 4, 5, 6, 5, 8, 9, 13, 3, 8, 8, 11, 7, 12, 13, 14, 2, 6, 7, 10, 6, 11, 13, 14, 7, 13,
    12, 14, 10, 14, 14, 15,
];

/// # Triad census algorithm
impl Graph {
    /// Returns the trycode associated to the provided triple of node IDs.
    ///
    /// # Arguments
    /// * `first`: NodeT - The first node ID of the triple.
    /// * `second`: NodeT - The second node ID of the triple.
    /// * `third`: NodeT - The third node ID of the triple.
    ///
    /// # Safety
    /// This method will assume that the three provided node IDs,
    /// namely `first`, `second` and `third`, are within the maximum
    /// number of nodes in this graph. If you provide values higher
    /// than that, this method will panic.
    pub unsafe fn get_unchecked_trycode_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> u8 {
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

        TRYCODES[first_to_second
            + 2 * (second_to_first
                + 2 * (first_to_third
                    + 2 * (third_to_first + 2 * (second_to_third + 2 * third_to_second))))]
    }

    /// Returns the trycode associated to the provided triple of node IDs.
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
    pub fn get_trycode_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> Result<u8> {
        self.validate_node_id(first)?;
        self.validate_node_id(second)?;
        self.validate_node_id(third)?;
        Ok(unsafe { self.get_unchecked_trycode_from_node_ids(first, second, third) })
    }

    /// Returns the trycode associated to the provided triple of node names.
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
    pub fn get_trycode_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<u8> {
        Ok(unsafe {
            self.get_unchecked_trycode_from_node_ids(
                self.get_node_id_from_node_name(first)?,
                self.get_node_id_from_node_name(second)?,
                self.get_node_id_from_node_name(third)?,
            )
        })
    }

    pub fn get_triad_census(&self) -> Result<[EdgeT; 16]> {
        self.must_be_undirected()?;
        let number_of_nodes: EdgeT = self.get_number_of_nodes() as EdgeT;
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
                    let mut census: [EdgeT; 16] = [0; 16];
                    let mut union_cardinality: EdgeT = 0;

                    while first_index < first_order_neighbours.len()
                        && second_index < second_order_neighbours.len()
                    {
                        let first_order_neighbour = first_order_neighbours[first_index];
                        let second_order_neighbour = second_order_neighbours[second_index];

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

                        if first < third
                            || (first < third
                                && third < second
                                && !self.has_edge_from_node_ids(first, third))
                        {
                            let trycode = unsafe {
                                self.get_unchecked_trycode_from_node_ids(first, second, third)
                            } as usize;
                            census[trycode] += 1;
                        }
                    }

                    // Since this is currently only supported for undirected
                    // graphs, the following if-else is actually tautological
                    // and that is why it is commented.
                    // let tritype = if self.has_edge_from_node_ids(second, first){
                    //     2
                    // } else {
                    //     1
                    // };
                    // Therefore tritype is always equal to 2.
                    let tritype = 2;

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
        let total_triads = census[1..].iter().copied().sum::<EdgeT>();
        census[0] =
            number_of_nodes * (number_of_nodes - 1) * (number_of_nodes - 2) / 6 - total_triads;
        Ok(census)
    }
}
