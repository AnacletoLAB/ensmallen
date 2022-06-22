use graph::Graph;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct CooccurrenceEdgePrediction {
    window_size: u64,
    iterations: usize,
    random_state: u64,
}

impl CooccurrenceEdgePrediction {
    /// Return new instance of CooccurrenceEdgePrediction model.
    ///
    /// # Arguments
    /// * `window_size`: Option<u64> - Window size defining the contexts.
    /// * `iterations`: Option<usize> - Number of walks to run from each node. By default 50.
    /// * `random_state`: Option<u64> - The random state to reproduce the predictions. By default 42.
    pub fn new(
        window_size: Option<u64>,
        iterations: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let window_size = window_size.unwrap_or(10);
        let iterations = iterations.unwrap_or(50);
        let random_state = random_state.unwrap_or(42);

        if window_size == 0 {
            return Err(concat!("The window size cannot be equal to zero.").to_string());
        }

        if iterations == 0 {
            return Err(concat!("The iterations cannot be equal to zero.").to_string());
        }

        Ok(Self {
            window_size,
            random_state,
            iterations,
        })
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `support`: Option<&Graph> - The graph whose structure is to be used.
    pub fn predict(&self, predictions: &mut [f32], graph: &Graph, support: Option<&Graph>) -> Result<(), String> {
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

        let support = support.unwrap_or(graph);

        support.must_share_node_vocabulary(graph)?;
        graph
            .par_iter_directed_edge_node_ids()
            .zip(predictions.par_iter_mut())
            .for_each(|((edge_id, src, dst), pred)| {
                let mut random_state = splitmix64(self.random_state + edge_id);
                let mut encounters = 0;
                (0..self.iterations).for_each(|_| {
                    random_state = splitmix64(self.random_state + edge_id);
                    if unsafe {
                        support
                            .iter_uniform_walk(src, random_state, self.window_size)
                            .any(|node| node == dst)
                    } {
                        encounters += 1;
                    }
                });
                *pred = encounters as f32 / self.iterations as f32;
            });

        Ok(())
    }
}
