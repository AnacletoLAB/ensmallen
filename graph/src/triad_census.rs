use super::types::*;
use super::Graph;

/// # Triad census algorithm
impl Graph {
    // unsafe fn get_unchecked_triad_census_from_node_id(
    //     &self,
    //     node_id: NodeT,
    //     triad_census: &mut [EdgeT; 16],
    // ) {
    //     self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
    // }

    // pub fn get_triad_census(&self) {}

    // pub fn get_triad_census_from_node_id(&self, node_id: NodeT) -> Result<[EdgeT; 16]> {
    //     self.validate_node_id(node_id)?;
    //     let mut triad_census = [0; 16];
    //     unsafe { self.get_unchecked_triad_census_from_node_id(node_id, &mut triad_census) };
    //     Ok(triad_census)
    // }
}
