use atomic_float::AtomicF32;
use express_measures::dot_product_sequential_unchecked;
use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f32, sample_uniform, splitmix64};

#[derive(Clone, Debug)]
pub struct Cooccurrence<'a> {
    walk_parameters: WalksParameters,
    window_size: usize,
    quantity: usize,
    graph: Option<&'a Graph>,
}

impl<'a> Cooccurrence<'a> {
    /// Return new instance of Cooccurrence model.
    ///
    /// # Arguments
    /// * `walk_parameters`: Option<WalksParameters> - Parameters to be used within the walks.
    /// * `window_size`: Option<usize> - Window size defining the contexts.
    /// * `quantity`: Option<usize> - Number of walks to run from each node. By default 10.
    pub fn new(
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        quantity: Option<usize>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let window_size = window_size.unwrap_or(10);
        let quantity = quantity.unwrap_or(10);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());

        if window_size == 0 {
            return Err(concat!("The window size cannot be equal to zero.").to_string());
        }

        if quantity == 0 {
            return Err(concat!("The quantity cannot be equal to zero.").to_string());
        }

        Ok(Self {
            walk_parameters,
            window_size,
            quantity,
            graph: None,
        })
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &'a Graph - The graph whose edges are to be learned.
    pub fn fit(&mut self, graph: &'a Graph) -> Result<(), String> {
        self.graph = Some(graph);
        Ok(())
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub fn predict(&self, predictions: &mut [f32], graph: &Graph) -> Result<(), String> {
        if predictions.len() != graph.get_number_of_directed_edges() as usize {
            return Err(format!(
                concat!(
                    "The provided predictions slice has size `{}` ",
                    "but it was expected to have the same ",
                    "size of the number of the directed edges in the graph `{}`."
                ),
                predictions.len(),
                graph.get_number_of_directed_edges()
            ));
        }

        if self.graph.is_none() {
            return Err("The model has not been fit!".to_string());
        }

        if let Some(support) = self.graph {
            support.must_share_node_vocabulary(graph)?;
            graph
                .par_iter_directed_edge_node_ids()
                .zip(predictions.par_iter_mut())
                .for_each(|((_, src, dst), pred)| {});
        }

        Ok(())
    }
}
