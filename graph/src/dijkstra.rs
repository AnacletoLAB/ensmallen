use super::*;
use bitvec::prelude::*;

impl Graph {
    /// Returns vector of minimum paths distances and vector of nodes predecessors.
    ///
    /// # Arguments
    /// * `src`: NodeT - Root of the tree of minimum paths.
    pub fn dijkstra(&self, src: NodeT) -> Result<(Vec<WeightT>, Vec<NodeT>), String> {
        self.validate_node_id(src)?;
        let nodes_number = self.get_nodes_number() as usize;
        let mut distances = vec![WeightT::INFINITY; nodes_number];
        distances[src as usize] = 0.0;
        let mut parents: Vec<NodeT> = vec![NodeT::MAX; nodes_number];
        let mut visited: BitVec<Lsb0, u8> = bitvec![Lsb0, u8; 0; nodes_number as usize];
        let mut visited_nodes_counter = 0;

        while visited_nodes_counter != nodes_number {
            let closest_node_id = distances
                .iter()
                .enumerate()
                .min_by(|(_, weight1), (_, weight2)| weight1.partial_cmp(weight2).unwrap())
                .unwrap()
                .0 as NodeT;
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id)
            {
                let unvisited = !visited[neighbour_node_id as usize];
                let edge_weight = if self.has_edge_weights() {
                    self.get_unchecked_edge_weight_from_edge_id(
                        self.get_unchecked_edge_id_from_node_ids(
                            closest_node_id,
                            neighbour_node_id,
                        ),
                    )
                    .unwrap()
                } else {
                    1.0
                };
                let neighbour_node_distance = edge_weight + distances[closest_node_id as usize];
                if unvisited && neighbour_node_distance < distances[neighbour_node_id as usize] {
                    parents[neighbour_node_id as usize] = closest_node_id;
                    distances[neighbour_node_id as usize] = neighbour_node_distance;
                }
            }
            visited.insert(closest_node_id as usize, true);
            visited_nodes_counter += 1;
        }
        Ok((distances, parents))
    }
}
