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
}

impl GraphEmbedder<f32> for Walklets {
    fn get_model_name(&self) -> String {
        format!("Walklets {}", self.node2vec.get_model_name())
    }

    fn get_embedding_sizes(&self, graph: &graph::Graph) -> Vec<(usize, usize)> {
        vec![
            (
                graph.get_number_of_nodes() as usize,
                self.node2vec.embedding_size
            );
            self.node2vec.window_size
        ]
    }

    fn fit_transform(
        &self,
        graph: &graph::Graph,
        embedding: &mut [&mut [f32]],
    ) -> Result<(), String> {
        if embedding.len() != self.node2vec.window_size {
            return Err(format!(
                concat!(
                    "The expected number of embedding was {}, ",
                    "like the model window size but was {}."
                ),
                self.node2vec.window_size,
                embedding.len()
            ));
        }
        let mut node2vec = self.node2vec.clone();
        node2vec.window_size = 1;
        (0..self.node2vec.window_size)
            .zip(embedding.iter_mut())
            .for_each(|(power, embedding)| {
                node2vec.walk_transformer = WalkletsWalkTransformer::new(power + 1).unwrap();
                node2vec.fit_transform(graph, &mut [embedding]).unwrap();
            });
        Ok(())
    }
}
