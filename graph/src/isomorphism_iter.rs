use rayon::iter::plumbing::*;
use rayon::prelude::*;

pub struct EqualBucketsParIter<H, O> {
    degree_bounded_hash_and_node_ids: Vec<(H, O)>,
}

impl<H, O> EqualBucketsParIter<H, O> {
    pub unsafe fn new(degree_bounded_hash_and_node_ids: Vec<(H, O)>) -> Self {
        EqualBucketsParIter {
            degree_bounded_hash_and_node_ids,
        }
    }
}

impl<H: Send + Sync + Eq + Copy + 'static, O: Send + Sync + 'static> ParallelIterator
    for EqualBucketsParIter<H, O>
{
    type Item = &'static [(H, O)];

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            EqualBucketsIter::new(&self.degree_bounded_hash_and_node_ids),
            consumer,
        )
    }

    fn opt_len(&self) -> Option<usize> {
        None
    }
}

#[derive(Clone)]
/// Iter over the slices of contiguos values
pub struct EqualBucketsIter<'a, H, O> {
    degree_bounded_hash_and_node_ids: &'a [(H, O)],

    start: usize,
    end: usize,
}

impl<'a, H, O> core::fmt::Debug for EqualBucketsIter<'a, H, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EqualBucketsIter")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl<'a, H, O> EqualBucketsIter<'a, H, O> {
    pub fn new(degree_bounded_hash_and_node_ids: &'a [(H, O)]) -> Self {
        EqualBucketsIter {
            degree_bounded_hash_and_node_ids,

            start: 0,
            end: degree_bounded_hash_and_node_ids.len(),
        }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl<'a, H: Eq + Copy + 'static, O: Send + Sync + 'static> core::iter::Iterator
    for EqualBucketsIter<'a, H, O>
{
    type Item = &'static [(H, O)];

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let mut current_hash = self.degree_bounded_hash_and_node_ids[self.start as usize].0;

        // try to find a pair of consecutive indices that have the same hash
        while self.start + 1 < self.end {
            let next_hash = self.degree_bounded_hash_and_node_ids[self.start as usize + 1].0;

            // hash differs so go to the next index
            if next_hash != current_hash {
                self.start += 1;
                current_hash = next_hash;
                continue;
            }

            // start of a consecutive group!
            let mut idx = self.start + 2; // we already know that it's long at least 2
            while idx < self.end {
                let next_hash = self.degree_bounded_hash_and_node_ids[idx as usize].0;

                if next_hash != current_hash {
                    break;
                }
                idx += 1;
            }
            // now we have scrolled through the whole slice and idx is the first
            // item with a different hash

            // THIS IS REALLY UNSAFE, until the iter lives it's fine, but
            // once the iter dies it would reference freed memory.
            // Therefore It should never be collected!!!
            //
            let res = unsafe {
                core::slice::from_raw_parts(
                    self.degree_bounded_hash_and_node_ids
                        .as_ptr()
                        .add(self.start),
                    idx - self.start,
                )
            };

            // skip the slice for the next iteration
            self.start = idx;

            return Some(res);
        }
        None
    }
}

impl<'a, H: Send + Sync + Eq + Copy + 'static, O: Send + Sync + 'static> UnindexedProducer
    for EqualBucketsIter<'a, H, O>
{
    type Item = &'static [(H, O)];

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split
        if (self.end - self.start) < 2 {
            return (self, None);
        }

        let mut split_idx = (self.start + self.end) / 2;
        let mut current_hash = self.degree_bounded_hash_and_node_ids[split_idx].0;

        split_idx += 1;

        // check that we are not in a contiguous chunk and skip till the next
        // different hash
        while split_idx < self.end {
            let next_hash = self.degree_bounded_hash_and_node_ids[split_idx].0;
            if next_hash != current_hash {
                let new = Self {
                    degree_bounded_hash_and_node_ids: self.degree_bounded_hash_and_node_ids,

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
