use super::*;
use rayon::iter::plumbing::*;

#[derive(Clone)]
pub struct EdgesIter<'a> {
    father: &'a CSR,

    start_src: NodeT,
    // inclusive
    start_edge_id: EdgeT,

    end_src: NodeT,
    // exclusive
    end_edge_id: EdgeT,
}

impl<'a> core::fmt::Debug for EdgesIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EdgesIter")
            .field("start_src", &self.start_src)
            .field("start_edge_id", &self.start_edge_id)
            .field("end_src", &self.end_src)
            .field("end_edge_id", &self.end_edge_id)
            .finish()
    }
}

impl<'a> EdgesIter<'a> {
    pub fn new(father: &'a CSR) -> Self {
        EdgesIter {
            father,

            start_src: 0,
            start_edge_id: 0,

            end_src: father.get_number_of_nodes().saturating_sub(1),
            end_edge_id: father.get_number_of_directed_edges(),
        }
    }

    pub fn len(&self) -> usize {
        (self.end_edge_id - self.start_edge_id) as usize
    }
}

impl<'a> core::iter::ExactSizeIterator for EdgesIter<'a> {}

impl<'a> core::iter::Iterator for EdgesIter<'a> {
    type Item = (EdgeT, NodeT, NodeT);

    fn next(&mut self) -> Option<Self::Item> {
        // end condition
        if self.start_edge_id >= self.end_edge_id {
            return None;
        }

        // if we finished the current src, skip singletons and go to the next
        loop {
            let src_limit = self.father.outbounds_degrees[1 + self.start_src as usize];

            if self.start_edge_id == src_limit {
                self.start_src += 1;
                continue;
            }

            break;
        }

        let dst = self.father.destinations[self.start_edge_id as usize];
        let result = (self.start_edge_id, self.start_src, dst);
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

impl<'a> core::iter::DoubleEndedIterator for EdgesIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // end condition
        if self.start_edge_id >= self.end_edge_id {
            return None;
        }

        self.end_edge_id -= 1;

        // if we finished the current src, skip singletons and go to the next
        loop {
            let src_limit = self.father.outbounds_degrees[self.end_src as usize];

            if self.end_edge_id < src_limit {
                self.end_src -= 1;
                continue;
            }

            break;
        }

        let dst = self.father.destinations[self.end_edge_id as usize];
        let result = (self.end_edge_id, self.end_src, dst);
        Some(result)
    }
}

impl<'a> UnindexedProducer for EdgesIter<'a> {
    type Item = (EdgeT, NodeT, NodeT);

    /// Split the file in two approximately balanced streams
    fn split(self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split
        if self.len() < 2 {
            return (self, None);
        }

        let split_idx = (self.start_edge_id + self.end_edge_id) / 2;

        let (high, low) = self.split_at(split_idx as _);
        (high, Some(low))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}

impl<'a> Producer for EdgesIter<'a> {
    type Item = (EdgeT, NodeT, NodeT);
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(mut self, split_idx: usize) -> (Self, Self) {
        let split_idx = self.start_edge_id + split_idx as EdgeT;
        // check that we are in a reasonable state
        debug_assert!(
            split_idx < self.end_edge_id,
            concat!(
                "We expected the split index to be smaller than the end index, ",
                "but it was not. start_idx: {} end_idx: {} split_idx: {}"
            ),
            self.start_edge_id,
            self.end_edge_id,
            split_idx
        );
        debug_assert!(self.start_edge_id < self.end_edge_id);
        debug_assert!(
            split_idx < self.father.get_number_of_directed_edges(),
            "start_idx: {} end_idx: {} split_idx: {} father len:{}",
            self.start_edge_id,
            self.end_edge_id,
            split_idx,
            self.father.get_number_of_directed_edges(),
        );

        let split_src = unsafe {
            self.father
                .get_unchecked_source_node_id_from_edge_id(split_idx) as NodeT
        };

        // high part
        let new_iter = Self {
            father: self.father,

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
