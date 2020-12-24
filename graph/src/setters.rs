use super::*;

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * name: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the embedding of the graph.
    ///
    /// # Arguments
    ///
    /// * embedding: Vec<Vec<f64>> - Embedding of the graph.
    pub fn set_embedding(&mut self, embedding: Vec<Vec<f64>>) -> Result<(), String> {
        if embedding.len() != self.get_nodes_number() as usize {
            return Err(format!(
                "Given embedding has {} rows but the graph has {} nodes.",
                embedding.len(),
                self.get_nodes_number()
            ));
        }
        self.embedding = Some(embedding);
        Ok(())
    }
}
