use super::*;

/// # Conversion of the graph.
impl Graph {
    /// Convert inplace the graph to directed.
    ///
    /// # Implementative details
    /// The conversion to a directed graph is trivial as only requires to
    /// switch the flag for directed to true.
    pub fn to_directed_inplace(&mut self) -> &mut Graph {
        self.directed = true;
        self
    }

    /// Return a new instance of the current graph as directed.
    pub fn to_directed(&self) -> Graph {
        let mut new_graph = self.clone();
        new_graph.to_directed_inplace();
        new_graph
    }
}
