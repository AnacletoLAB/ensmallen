use crate::*;
use express_measures::ThreadFloat;
use graph::{EdgeT, NodeT};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::AsPrimitive;

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

    fn get_dtype(&self) -> String {
        self.node2vec.get_dtype()
    }

    fn get_number_of_steps(&self) -> usize {
        self.node2vec.get_number_of_steps()
    }

    fn requires_random_initialization(&self) -> bool {
        self.node2vec.requires_random_initialization()
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

    fn _fit_transform<F: ThreadFloat + 'static>(
        &self,
        graph: &graph::Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
        EdgeT: AsPrimitive<F>,
    {
        let mut node2vec = self.node2vec.clone();
        node2vec.window_size = 1;
        let loading_bar = if self.is_verbose() {
            let pb = ProgressBar::new(self.get_window_size() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(&format!(
                        concat!(
                            "{}{{msg}} {{spinner:.green}} [{{elapsed_precise}}] ",
                            "[{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})"
                        ),
                        self.get_model_name()
                    ))
                    .unwrap(),
            );
            pb
        } else {
            ProgressBar::hidden()
        };
        (0..self.get_window_size())
            .zip(embedding.chunks_mut(2))
            .progress_with(loading_bar)
            .for_each(|(power, embedding)| {
                node2vec.walk_transformer = WalkletsWalkTransformer::new(power + 1).unwrap();
                node2vec.fit_transform(graph, embedding).unwrap();
            });
        Ok(())
    }
}
