use super::*;
use num_traits::Float;
use tags::*;

#[derive(Debug)]
/// Reference classic binary heap
#[no_binding]
pub struct DijkstraQueue<'a, F> {
    /// This is the actual heap, it contains the node_ids and are ordered based on
    /// self.distances[id]
    heap: Vec<usize>,

    /// The distance of every node in the graph
    distances: &'a mut [F],

    /// The mapping from each node to its position in the heap.
    /// This is only needed because we don't want to insert duplicated nodes.
    map: Vec<usize>,
}

impl<'a, F: Float> DijkstraQueue<'a, F> {
    /// Initialize the queue with the given root, in this case the capacity
    /// should always be equal to the number of nodes in the graph.
    pub fn with_capacity_from_roots(
        capacity: usize,
        root_node_ids: Vec<NodeT>,
        distances: &'a mut [F],
    ) -> Self {
        let mut res = DijkstraQueue {
            heap: Vec::with_capacity(capacity),
            distances,
            map: vec![usize::MAX; capacity],
        };
        for root_node_id in root_node_ids {
            res.heap.push(root_node_id as usize);
            res.map[root_node_id as usize] = 0;
            res.distances[root_node_id as usize] = F::zero();
        }
        res
    }

    /// Get the index of the father of the given node
    #[inline]
    fn parent(node: usize) -> usize {
        (node.saturating_sub(1)) >> 1
    }

    /// Get the index of the left child
    #[inline]
    fn left(node: usize) -> usize {
        (node << 1) + 1
    }

    /// Get the index of the right child
    #[inline]
    fn right(node: usize) -> usize {
        (node << 1) + 2
    }

    // If the heap is empty or not
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    // Returns number of elements in the heap.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// add a value to the heap
    pub fn push(&mut self, node_id: usize, distance: F) {
        // If the distance is finite, the node **IS** already present,
        // we check if the new distance is smaller, in that case we have to
        // fix the heap.
        if self.map[node_id as usize] != usize::MAX {
            let old_distance = self.distances[node_id as usize];
            if old_distance > distance {
                self.distances[node_id as usize] = distance;
                self.bubble_up(self.map[node_id as usize], distance);
            }
            return;
        }

        // otherwise its th
        // add the node as the last value in the tree
        self.heap.push(node_id);
        self.distances[node_id as usize] = distance;
        // fix the heap
        self.bubble_up(self.len() - 1, distance);
    }

    // bubble up the value until the heap property holds
    fn bubble_up(&mut self, mut idx: usize, distance: F) {
        loop {
            let parent_idx = Self::parent(idx);

            if distance >= self.distances[self.heap[parent_idx] as usize] {
                break;
            }

            // swap the parent and the child
            self.map[self.heap[idx]] = parent_idx;
            self.map[self.heap[parent_idx]] = idx;
            self.heap.swap(idx, parent_idx);

            // Update the mutables
            idx = parent_idx;
        }
    }

    /// remove and return the smallest value
    pub fn pop(&mut self) -> Option<usize> {
        // if the queue is empty we can early-stop.
        if self.is_empty() {
            return None;
        }

        // swap the minimum with the last value
        // this is done so we can pop from the end of the vector
        // so we are ensured O(1) complexity.
        let number_of_elements = self.len() - 1;
        // Reset its position in the map
        self.map[self.heap[0] as usize] = usize::MAX;
        self.map[self.heap[number_of_elements]] = 0;
        // swap the value with the last
        self.heap.swap(0, number_of_elements);
        // remove the minimum from the tree
        let result = self.heap.pop();

        if !self.is_empty() {
            self.bubble_down(0, self.distances[self.heap[0] as usize]);
        }

        result
    }

    fn bubble_down(&mut self, mut idx: usize, distance: F) {
        // fix the heap by bubbling down the value
        loop {
            // get the indices of the right and left child
            let left_i = Self::left(idx);
            let right_i = Self::right(idx);
            let left_v = self
                .heap
                .get(left_i)
                .map(|x| self.distances[*x])
                .unwrap_or(F::infinity());
            let right_v = self
                .heap
                .get(right_i)
                .map(|x| self.distances[*x])
                .unwrap_or(F::infinity());

            // find the smallest child
            let (smallest_i, smallest_v) = if left_v > right_v {
                (right_i, right_v)
            } else {
                (left_i, left_v)
            };

            // and the heap rule is violated
            if smallest_v < distance {
                // fix it and keep bubbling down
                self.map[self.heap[idx]] = smallest_i;
                self.map[self.heap[smallest_i]] = idx;
                self.heap.swap(idx, smallest_i);
                idx = smallest_i;
                continue;
            }

            // the min heap rule holds for both childs so we can exit.
            break;
        }
    }
}

use std::ops::{Index, IndexMut};
impl<'a, F> Index<usize> for DijkstraQueue<'a, F> {
    type Output = F;
    fn index(&self, node_id: usize) -> &F {
        &self.distances[node_id]
    }
}

impl<'a, F> IndexMut<usize> for DijkstraQueue<'a, F> {
    fn index_mut(&mut self, node_id: usize) -> &mut F {
        &mut self.distances[node_id]
    }
}
