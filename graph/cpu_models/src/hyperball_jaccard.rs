use crate::must_not_be_zero;
use graph::{Graph, NodeT};
use hyperloglog_rs::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct HyperBallJaccard<const PRECISION: usize, const BITS: usize>
where
    [(); ceil(1 << PRECISION, 32 / BITS)]:,
{
    /// Vector of HyperLogLog counters
    counters: Vec<HyperLogLog<PRECISION, BITS>>,
    /// Cached cardinalities of HyperLogLog counters
    cardinalities: Vec<f32>,
    /// The number of hops to execute.
    number_of_hops: usize,
}

impl<const PRECISION: usize, const BITS: usize> HyperBallJaccard<PRECISION, BITS>
where
    [(); ceil(1 << PRECISION, 32 / BITS)]:,
{
    /// Creates a new HyperBallJaccard model.
    ///
    /// # Arguments
    /// * `number_of_hops`: Option<usize> - The number of hops for the Jaccard. By default, `1`.
    pub fn new(number_of_hops: Option<usize>) -> Result<Self, String> {
        let number_of_hops = must_not_be_zero(number_of_hops, 1, "number of convolutions")?;

        Ok(Self {
            counters: Vec::new(),
            cardinalities: Vec::new(),
            number_of_hops,
        })
    }

    fn must_be_trained(&self) -> Result<(), String> {
        if self.counters.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(())
    }

    /// Fit the HyperBall model to the provided graph.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub fn fit(&mut self, graph: &Graph) -> Result<(), String> {
        // Create HyperLogLog counters for all nodes in the graph
        let mut counters: Vec<HyperLogLog<PRECISION, BITS>> = graph
            .par_iter_node_ids()
            .map(|node_id| node_id.into())
            .collect::<Vec<_>>();

        // Create copies of the counters to keep track of the previous iteration's state
        let mut previous_counters = counters.clone();

        (0..self.number_of_hops).for_each(|_| {
            // Iterate over all nodes
            counters
                .par_iter_mut()
                .enumerate()
                .for_each(|(node_id, counter)| {
                    // Iterate over all neighbors of the current node
                    *counter = unsafe {
                        graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .map(|dst| &previous_counters[dst as usize])
                    .union()
                        | &previous_counters[node_id];
                });

            core::mem::swap(&mut counters, &mut previous_counters);
        });

        self.counters = previous_counters;

        Ok(())
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub unsafe fn get_jaccard_unchecked(&self, src: usize, dst: usize) -> f32 {
        self.counters[src].estimate_jaccard_cardinality(&self.counters[dst])
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub fn get_jaccard(&self, src: usize, dst: usize) -> Result<f32, String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // We check whether the two provided nodes are lower
        // than the expected number of nodes in the graph
        if src >= self.counters.len() || dst >= self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided nodes {} and {} are not lower than the ",
                    "expected number of nodes in the graph `{}`."
                ),
                src,
                dst,
                self.counters.len()
            ));
        }

        Ok(unsafe{self.get_jaccard_unchecked(src, dst)})
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub fn predict(&self, predictions: &mut [f32], graph: &Graph) -> Result<(), String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // Check that the provided predictions slice has the same size of the number of edges in the graph
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

        // Check that the graph has the same number of nodes as the model
        if graph.get_number_of_nodes() as usize != self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided graph has `{}` nodes ",
                    "but the model has been trained on a graph with `{}` nodes."
                ),
                graph.get_number_of_nodes(),
                self.counters.len()
            ));
        }

        // Iterate over all edges in the graph and compute the Jaccard similarity
        predictions
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .for_each(|(prediction, (_, src, dst))| unsafe {
                *prediction = self.get_jaccard_unchecked(src as usize, dst as usize);
            });

        Ok(())
    }

    pub fn dump(&self, path: &str) -> Result<(), String> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
