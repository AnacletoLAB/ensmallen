use super::*;
use rayon::prelude::*;
use rayon::{iter::plumbing::*, prelude::IntoParallelRefMutIterator};
use std::sync::Arc;

#[derive(Clone)]
pub struct EdgesIterUndirected<'a> {
    father: &'a CSR,

    outbounds: Arc<Vec<u64>>,

    start_src: NodeT,
    // inclusive
    start_edge_id: EdgeT,

    end_src: NodeT,
    // exclusive
    end_edge_id: EdgeT,
}

impl<'a> EdgesIterUndirected<'a> {
    pub fn new(father: &'a CSR) -> Self {
        // compute the outdegrees of the graph after removing the undirected edges
        // keeping only the lower triangular matrix
        let mut outbounds = vec![0; (father.get_number_of_nodes() + 1) as usize];
        outbounds[1..]
            .par_iter_mut()
            .enumerate()
            .for_each(|(node_id, outdegree)| {
                *outdegree = unsafe {
                    father.get_unchecked_neighbours_node_ids_from_src_node_id(node_id as NodeT)
                }
                .iter()
                .filter(|neighbour_id| **neighbour_id <= node_id as NodeT)
                .count() as u64
            });

        // comptue the cumulative sum
        let mut prefix_sum = 0;
        outbounds.iter_mut().for_each(|outdegree| {
            let tmp = *outdegree;
            *outdegree += prefix_sum;
            prefix_sum += tmp;
        });

        EdgesIterUndirected {
            father,
            outbounds: Arc::new(outbounds),

            start_src: 0,
            start_edge_id: 0,

            end_src: father.get_number_of_nodes().saturating_sub(1),
            end_edge_id: prefix_sum,
        }
    }

    pub fn len(&self) -> usize {
        (self.end_edge_id - self.start_edge_id) as usize
    }
}

impl<'a> core::iter::ExactSizeIterator for EdgesIterUndirected<'a> {}

impl<'a> core::iter::Iterator for EdgesIterUndirected<'a> {
    type Item = (NodeT, NodeT);

    fn next(&mut self) -> Option<Self::Item> {
        // end condition
        if self.start_edge_id >= self.end_edge_id {
            return None;
        }

        // if we finished the current src, skip singletons and go to the next
        loop {
            let src_limit = self.outbounds[1 + self.start_src as usize];

            if self.start_edge_id == src_limit {
                self.start_src += 1;
                continue;
            }

            break;
        }
        // convert undirected edge id to directed edge id
        let src_start = self.outbounds[self.start_src as usize];
        let edge_id =
            self.start_edge_id - src_start + self.father.outbounds_degrees[self.start_src as usize];

        // return the result
        let dst = self.father.destinations[edge_id as usize];
        let result = (self.start_src, dst);
        self.start_edge_id += 1;
        Some(result)
    }

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a> core::iter::DoubleEndedIterator for EdgesIterUndirected<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // end condition
        if self.start_edge_id >= self.end_edge_id {
            return None;
        }

        self.end_edge_id -= 1;

        // if we finished the current src, skip singletons and go to the next
        let mut src_start;
        loop {
            src_start = self.outbounds[self.end_src as usize];

            if self.end_edge_id > src_start {
                self.end_src -= 1;
                continue;
            }

            break;
        }

        // convert undirected edge id to directed edge id
        let edge_id =
            self.end_edge_id - src_start + self.father.outbounds_degrees[self.end_src as usize];

        // return the result
        let dst = self.father.destinations[edge_id as usize];
        let result = (self.end_src, dst);
        Some(result)
    }
}

impl<'a> UnindexedProducer for EdgesIterUndirected<'a> {
    type Item = (NodeT, NodeT);

    /// Split the file in two approximately balanced streams
    fn split(self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split
        if self.len() < 2 {
            return (self, None);
        }

        let split_idx = self.len() / 2;

        let (low, high) = self.split_at(split_idx as _);
        (low, Some(high))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}

impl<'a> Producer for EdgesIterUndirected<'a> {
    type Item = (NodeT, NodeT);
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(mut self, split_idx: usize) -> (Self, Self) {
        // debug_assert!(split_idx < self.len(), "{} {}", split_idx, self.len());
        let split_idx = self.start_edge_id + split_idx as EdgeT;
        //let split_idx = split_idx as EdgeT; //self.start_edge_id + split_idx as EdgeT;
        // check that we are in a reasonable state
        debug_assert!(
            split_idx < self.end_edge_id,
            "{} {} < {}",
            self.start_edge_id,
            split_idx,
            self.end_edge_id
        );
        debug_assert!(
            split_idx < self.father.get_number_of_directed_edges(),
            "start_idx: {} end_idx: {} split_idx: {} father len:{}",
            self.start_edge_id,
            self.end_edge_id,
            split_idx,
            self.father.get_number_of_directed_edges(),
        );

        let split_src = self.outbounds.partition_point(|&x| x <= split_idx) as NodeT - 1;

        // high part
        let new_iter = Self {
            father: self.father,
            outbounds: self.outbounds.clone(),

            start_src: split_src,
            start_edge_id: split_idx,

            end_src: self.end_src,
            end_edge_id: self.end_edge_id,
        };

        // low part
        self.end_src = split_src;
        self.end_edge_id = split_idx;

        // return the two halfs
        debug_assert_ne!(self.len(), 0);
        debug_assert_ne!(new_iter.len(), 0);
        (self, new_iter)
    }
}
