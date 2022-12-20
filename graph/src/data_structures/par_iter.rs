use super::*;
use rayon::prelude::*;

// Par iter
// Indexed ParIter

impl CSR {
    pub unsafe fn par_iter_unchecked_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl IndexedParallelIterator<Item = NodeT> + Send + '_ {
        self.destinations[self.iter_unchecked_edge_ids_from_source_node_id(src)]
            .par_iter()
            .cloned()
    }

    pub fn par_iter_edge_node_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.par_iter_directed_edge_node_ids()
            .filter(move |(_edge_id, src, dst)| {
                directed || src <= dst
            })
    }

    pub fn par_iter_directed_edge_node_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        // TODO!: this is stupid and can be made faster
        (0..self.get_number_of_directed_edges() as usize).into_par_iter()
            .map(move |edge_id| {
                let edge_id = edge_id as EdgeT;
                let (src, dst) = unsafe{
                    self.get_unchecked_node_ids_from_edge_id(edge_id)
                }; 
                (
                    edge_id,
                    src,
                    dst,
                )
            })
    }

}