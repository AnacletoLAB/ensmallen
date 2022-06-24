use graph::Graph;

pub trait GraphEmbedder<F> {
    /// Computes in the provided memory slice the graph embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &[&mut [F]] - The memory area where to write the embedding.
    fn fit_transform(&self, graph: &Graph, embedding: &mut [&mut [F]]) -> Result<(), String>;

    fn get_model_name(&self) -> String;

    /// Returns the sizes of the embeddings given the graph.
    fn get_embedding_sizes(&self, graph: &Graph) -> Vec<(usize, usize)>;
}
