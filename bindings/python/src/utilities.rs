impl EnsmallenGraph {
    /// Return start node and end node for given batch.
    fn get_batch_range(&self, idx: usize, batch_size: usize) -> (usize, usize) {
        let (start_node, end_node) = (idx * batch_size, (idx + 1) * batch_size);
        (
            start_node,
            if end_node > self.get_not_trap_nodes_number() {
                self.get_not_trap_nodes_number()
            } else {
                end_node
            },
        )
    }
}