use crate::*;

#[derive(Clone, Debug)]
pub struct Walklets {
    node2vec: Node2Vec<WalkletsWalkTransformer>,
}

impl Walklets {
    /// Return new Walklets object.
    pub fn new(node2vec: Node2Vec<WalkletsWalkTransformer>) -> Self {
        Self { node2vec }
    }

    pub fn get_window_size(&self) -> usize {
        self.node2vec.window_size
    }
}

impl GraphEmbedder for Walklets {
    fn get_model_name(&self) -> String {
        format!("Walklets {}", self.node2vec.get_model_name())
    }

    fn is_verbose(&self) -> bool {
        self.node2vec.is_verbose()
    }

    fn get_number_of_epochs(&self) -> usize {
        self.node2vec.get_number_of_epochs()
    }

    fn get_embedding_shapes(&self, graph: &graph::Graph) -> Result<Vec<MatrixShape>, String> {
        let mut shapes = Vec::new();
        for _ in 0..self.node2vec.window_size {
            for shape in self.node2vec.get_embedding_shapes(graph)? {
                shapes.push(shape);
            }
        }
        Ok(shapes)
    }

    fn get_random_state(&self) -> u64 {
        self.node2vec.get_random_state()
    }

    fn _fit_transform(
        &self,
        graph: &graph::Graph,
        embedding: &mut [&mut [f32]],
    ) -> Result<(), String> {
        let mut node2vec = self.node2vec.clone();
        node2vec.window_size = 1;
        (0..self.get_window_size())
            .zip(embedding.chunks_mut(2))
            .for_each(|(power, embedding)| {
                node2vec.walk_transformer = WalkletsWalkTransformer::new(power + 1).unwrap();
                node2vec.fit_transform(graph, embedding).unwrap();
            });
        Ok(())
    }
}
