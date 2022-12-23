use super::*;
use rayon::prelude::*;
use rayon::iter::plumbing::*;

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

    /// slower version, it's just used for correctness checking in the tests
    /// #[cfg(test)]
    pub fn par_iter_directed_edge_node_ids_naive(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
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

    pub fn par_iter_directed_edge_node_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        //EdgesParIter::new(self)
        self.par_iter_directed_edge_node_ids_naive()
    }

}

pub struct EdgesParIter<'a> {
    pub(crate) father: &'a CSR,
}

impl<'a> EdgesParIter<'a> {
    pub(crate) fn new(father: &'a CSR) -> Self {
        EdgesParIter{
            father,
        }
    }
}

impl<'a> ParallelIterator for EdgesParIter<'a> {
    type Item = (EdgeT, NodeT, NodeT);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            EdgesIter::new(self.father),
            consumer,
        )
    }

    fn opt_len(&self) -> Option<usize> {
        None
    }
}

impl<'a> IndexedParallelIterator for EdgesParIter<'a> {
    fn drive<C>(self, consumer: C) -> C::Result
        where C: Consumer<Self::Item>
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.father.get_number_of_directed_edges() as usize
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
        where CB: ProducerCallback<Self::Item>
    {
        // Drain every item, and then the vector only needs to free its buffer.
        callback.callback(EdgesIter::new(self.father))
    }
}