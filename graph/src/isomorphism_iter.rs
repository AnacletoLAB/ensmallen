use super::*;
use rayon::prelude::*;
use rayon::iter::plumbing::*;

type HashType = u64;

pub struct EqualBucketsParIter {
    hashes: Vec<HashType>,
    indices: Vec<NodeT>,
    degree_bounded_node_ids: Vec<NodeT>,
}

impl EqualBucketsParIter {
    pub fn new(hashes: Vec<HashType>, indices: Vec<NodeT>, degree_bounded_node_ids: Vec<NodeT>) -> Self {
        EqualBucketsParIter{
            hashes,
            indices,
            degree_bounded_node_ids,
        }
    }
}

impl ParallelIterator for EqualBucketsParIter {
    type Item = Vec<NodeT>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            EqualBucketsIter::new(&self.hashes, &self.indices, &self.degree_bounded_node_ids),
            consumer,
        )
    }

    fn opt_len(&self) -> Option<usize> {
        None
    }
}

#[derive(Clone)]
/// Iter over the slices of contiguos values
pub struct EqualBucketsIter<'a> {
    hashes: &'a [HashType],
    indices: &'a [NodeT],
    degree_bounded_node_ids: &'a [NodeT],

    start: usize,
    end: usize,
}

impl<'a> core::fmt::Debug for EqualBucketsIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EqualBucketsIter")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl<'a> EqualBucketsIter<'a> {
    pub fn new(hashes: &'a [HashType], indices: &'a [NodeT], degree_bounded_node_ids: &'a [NodeT]) -> Self {
        EqualBucketsIter {
            hashes,
            indices,
            degree_bounded_node_ids,

            start: 0,
            end: indices.len(),
        }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl<'a> core::iter::Iterator for EqualBucketsIter<'a> {
    type Item = Vec<NodeT>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let mut current_hash = self.hashes[self.indices[
                self.start as usize
            ] as usize];

        // try to find a pair of consecutive indices that have the same hash
        while self.start < self.end {

            let next_hash = self.hashes[self.indices[
                    self.start as usize + 1
                ] as usize];

            // hash differs so go to the next index
            if next_hash != current_hash {
                self.start += 1;
                current_hash = next_hash;
                continue
            }

            // start of a consecutive group!
            let mut idx = self.start + 2; // we already know that it's long at least 2
            while idx < self.end {    
                let next_hash = self.hashes[self.indices[
                    idx
                ] as usize];

                if next_hash != current_hash {
                    break
                }
                idx += 1;
            }
            // now we have scrolled through the whole slice and idx is the first
            // item with a different hash

            // take a reference to the computed slice to return it later
            let res = self.indices[self.start..idx].iter().map(|i| {
                self.degree_bounded_node_ids[*i as usize]
            }).collect::<Vec<_>>();
            // skip the slice for the next iteration
            self.start = idx;

            return Some(res);
        }
        None
    }
}

impl<'a> UnindexedProducer for EqualBucketsIter<'a> {
    type Item = Vec<NodeT>;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split
        if (self.end - self.start) < 2 {
            return (self, None);
        }

        let mut split_idx = (self.start + self.end) / 2;
        let mut current_hash = self.hashes[self.indices[split_idx] as usize];

        split_idx += 1;

        // check that we are not in a contiguous chunk and skip till the next 
        // different hash
        while split_idx < self.end {

            let next_hash = self.hashes[self.indices[split_idx] as usize];
            if next_hash != current_hash {
                let new = Self {
                    indices: self.indices,
                    hashes: self.hashes,
                    degree_bounded_node_ids: self.degree_bounded_node_ids,

                    start: split_idx,
                    end: self.end,
                };
                self.end = split_idx;
                return (self, Some(new));
            }

            split_idx += 1;
            current_hash = next_hash;
        }

        (self, None)
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
