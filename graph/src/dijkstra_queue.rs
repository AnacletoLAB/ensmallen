
/// Reference classic binary heap
pub struct DijkstraQueue {
    heap: Vec<usize>,
    distances: Vec<f64>,
}

impl DijkstraQueue {
    /// Initialize a new empty heap which is guaranteed to hold at least 
    /// `capacity` elements without triggering a re-allocation.
    pub fn with_capacity(capacity: usize) -> Self {
        DijkstraQueue{
            heap: Vec::with_capacity(capacity),
            distances: vec![f64::INFINITY; capacity],
        }
    }

    /// Initialize the queue with the given root, in this case the capacity
    /// should always be equal to the number of nodes in the graph.
    pub fn with_capacity_from_root(capacity: usize, root_node_id: usize,) -> Self {
        let mut res = DijkstraQueue{
            heap: Vec::with_capacity(capacity),
            distances: vec![f64::INFINITY; capacity],
        };
        res.heap.push(root_node_id);
        res.distances[root_node_id] = 0.0;
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

    /// add a value to the heap
    pub fn push(&mut self, node_id: usize, distance: f64) {
        // Insert the value and get its index
        let mut idx = self.heap.len();
        self.heap.push(node_id);
        self.distances[node_id as usize] = distance;
        
        // bubble up the value until the heap property holds
        loop {
            let parent_idx = DijkstraQueue::parent(idx);

            // The heap condition is respected so we can stop.
            // This also handles the case of the node at the root since
            // self.parent(0) == 0 => current_value == parent_value
            if distance >= self.distances[self.heap[parent_idx] as usize] {
                break
            }

            // swap the parent and the child
            self.heap.swap(idx, parent_idx);

            // Update the mutables
            idx = parent_idx;
        }
    }

    /// Return the computed distances
    pub fn unwrap(self) -> Vec<f64> {
        self.distances
    }

    /// remove and return the smallest value 
    pub fn pop(&mut self) -> Option<usize> {
        // if the queue is empty we can early-stop.
        if self.heap.is_empty() {
            return None;
        }

        // swap the minimum with the last value
        // this is done so we can pop from the end of the vector
        // so we are ensured O(1) complexity.
        let number_of_elements = self.heap.len() - 1;
        self.heap.swap(0, number_of_elements);

        // remove the minimum from the tree
        let result = self.heap.pop();

        if self.heap.is_empty() {
            return result;
        }

        // fix the heap by bubbling down the value
        let mut idx = 0;
        let value = self.distances[self.heap[0]];
        loop {
            // get the indices of the right and left child
            let left_i = DijkstraQueue::left(idx);
            let right_i = DijkstraQueue::right(idx);
            let left_v = self.heap.get(left_i).map(|x| self.distances[*x]).unwrap_or(f64::INFINITY);
            let right_v = self.heap.get(right_i).map(|x| self.distances[*x]).unwrap_or(f64::INFINITY);

            // find the smallest child
            let (smallest_i, smallest_v) = if left_v > right_v {
                (right_i, right_v)
            } else {
                (left_i, left_v)
            };

            // and the heap rule is violated
            if smallest_v < value {
                // fix it and keep bubbling down
                self.heap.swap(idx, smallest_i);
                idx = smallest_i;   
                continue;
            }
            
            // the min heap rule holds for both childs so we can exit.
            break;
        }

        result
    }
}

use std::ops::{Index, IndexMut};
impl Index<usize> for DijkstraQueue {
    type Output = f64;
    fn index(&self, node_id: usize) -> &f64 {
        &self.distances[node_id]
    }
} 
impl IndexMut<usize> for DijkstraQueue {
    fn index_mut(&mut self, node_id: usize) -> &mut f64 {
        &mut self.distances[node_id]
    }
} 