use crate::*;
use graph::{EdgeT, Graph, NodeT};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicAnchorsInferredNodeEmbedding {
    /// Number of features to be computed.
    embedding_size: usize,
    /// Whether to show a loading bar while computing the embedding.
    verbose: bool,
}

impl BasicAnchorsInferredNodeEmbedding {
    /// Return new instance of Basic inferred node embedding.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(embedding_size: Option<usize>, verbose: Option<bool>) -> Result<Self, String> {
        let embedding_size = must_not_be_zero(embedding_size, 100, "Embedding size")?;
        Ok(Self {
            embedding_size,
            verbose: verbose.unwrap_or(true),
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Returns whether to show loading bars size.
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
}

pub trait AnchorsInferredNodeEmbeddingProperties {
    fn get_model_name(&self) -> String;

    fn get_basic_inferred_node_embedding(&self) -> &BasicAnchorsInferredNodeEmbedding;

    fn get_embedding_size(&self, graph: &Graph) -> Result<usize, String>;

    fn is_verbose(&self) -> bool {
        self.get_basic_inferred_node_embedding().is_verbose()
    }
}

#[derive(PartialEq, Eq)]
pub enum AnchorFeatureTypes {
    Walks,
    ShortestPaths,
}

pub trait AnchorsBasedFeature<const AFT: AnchorFeatureTypes> {
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        bucket: Vec<NodeT>,
        features: &mut [Feature],
    ) where
        Feature: IntegerFeatureType;
}

#[derive(PartialEq, Eq)]
pub enum AnchorTypes {
    Degrees,
    NodeTypes,
    Scores,
}

pub trait AnchorsGenerator<const AT: AnchorTypes> {
    type AnchorsIterator<'a>: Iterator<Item = Vec<NodeT>> + 'a
    where
        Self: 'a;
    fn iter_anchor_nodes_buckets<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> Result<Self::AnchorsIterator<'a>, String>;
}

pub trait NodeTypesAnchorsGenerator {}

impl<M> AnchorsGenerator<{ AnchorTypes::NodeTypes }> for M
where
    M: NodeTypesAnchorsGenerator,
{
    type AnchorsIterator<'a> = impl Iterator<Item = Vec<NodeT>> + 'a where Self: 'a, M: 'a;

    /// Return vector of vectors of anchor node IDs.
    fn iter_anchor_nodes_buckets<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> Result<Self::AnchorsIterator<'a>, String> {
        Ok(graph
            .iter_unique_node_type_ids()?
            .map(move |node_type_id| graph.get_node_ids_from_node_type_id(node_type_id).unwrap()))
    }
}

pub trait DegreesAnchorsGenerator {}

impl<M> AnchorsGenerator<{ AnchorTypes::Degrees }> for M
where
    M: AnchorsInferredNodeEmbeddingProperties + DegreesAnchorsGenerator,
{
    type AnchorsIterator<'a> = impl Iterator<Item = Vec<NodeT>> + 'a where Self: 'a, M: 'a;

    /// Return vector of vectors of anchor node IDs.
    fn iter_anchor_nodes_buckets<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> Result<Self::AnchorsIterator<'a>, String> {
        let embedding_size = self.get_embedding_size(graph)?;
        let number_of_edge_per_bucket: EdgeT =
            ((graph.get_number_of_directed_edges() as f32 / 2.0 / embedding_size as f32).ceil()
                as EdgeT)
                .max(1);

        let mut node_ids: Vec<NodeT> = graph.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| unsafe {
            graph
                .get_unchecked_node_degree_from_node_id(b)
                .partial_cmp(&graph.get_unchecked_node_degree_from_node_id(a))
                .unwrap()
        });
        // Allocate the node scores
        let mut current_bucket_size = 0;
        let mut number_of_buckets = 0;
        let mut current_bucket: Vec<NodeT> = Vec::new();
        Ok(node_ids.into_iter().filter_map(move |node_id| unsafe {
            if number_of_buckets == embedding_size {
                return None;
            }
            if current_bucket_size > number_of_edge_per_bucket {
                current_bucket_size = 0;
                number_of_buckets += 1;
                return Some(core::mem::replace(&mut current_bucket, Vec::new()));
            }
            current_bucket_size += graph.get_unchecked_node_degree_from_node_id(node_id) as EdgeT;
            current_bucket.push(node_id);
            None
        }))
    }
}

pub trait ScoresAnchorsGenerator {
    fn get_scores(&self) -> &[f32];
}

impl<M> AnchorsGenerator<{ AnchorTypes::Scores }> for M
where
    M: AnchorsInferredNodeEmbeddingProperties + ScoresAnchorsGenerator,
{
    type AnchorsIterator<'a> = impl Iterator<Item = Vec<NodeT>> + 'a where Self: 'a, M: 'a;

    /// Return vector of vectors of anchor node IDs.
    fn iter_anchor_nodes_buckets<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> Result<Self::AnchorsIterator<'a>, String> {
        let scores = self.get_scores();
        if scores.len() != graph.get_number_of_nodes() as usize {
            return Err(format!(
                concat!(
                    "The provided scores length {} does not ",
                    "match the number of nodes in the provided graph {}."
                ),
                scores.len(),
                graph.get_number_of_nodes() as usize
            ));
        }
        let embedding_size = self.get_embedding_size(graph)?;
        let number_of_score_per_bucket: f32 =
            (scores.into_par_iter().sum::<f32>() / 2.0 / embedding_size as f32).max(1.0);

        let mut node_ids: Vec<NodeT> = graph.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| {
            scores[b as usize].partial_cmp(&scores[a as usize]).unwrap()
        });
        // Allocate the node scores
        let mut current_bucket_size = 0.0;
        let mut number_of_buckets = 0;
        let mut current_bucket: Vec<NodeT> = Vec::new();
        Ok(node_ids.into_iter().filter_map(move |node_id| {
            if number_of_buckets == embedding_size {
                return None;
            }
            if current_bucket_size > number_of_score_per_bucket {
                current_bucket_size = 0.0;
                number_of_buckets += 1;
                return Some(core::mem::replace(&mut current_bucket, Vec::new()));
            }
            current_bucket_size += scores[node_id as usize];
            current_bucket.push(node_id);
            None
        }))
    }
}

pub trait AnchorsInferredNodeEmbeddingModel<const AT: AnchorTypes, const AFT: AnchorFeatureTypes>
where
    Self: AnchorsInferredNodeEmbeddingProperties + AnchorsBasedFeature<AFT> + AnchorsGenerator<AT>,
{
    /// Computes in the provided slice of embedding the DegreeSPINE node embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [Feature] - The memory area where to write the embedding.
    fn fit_transform<Feature>(&self, graph: &Graph, embedding: &mut [Feature]) -> Result<(), String>
    where
        Feature: IntegerFeatureType,
    {
        let expected_embedding_len =
            self.get_embedding_size(graph)? * graph.get_number_of_nodes() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        // Check that the graph has edges.
        graph.must_have_edges()?;

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the computation of the features.
        let features_progress_bar = if self.is_verbose() {
            let pb = ProgressBar::new(self.get_embedding_size(graph)? as u64);
            pb.set_style(ProgressStyle::default_bar().template(&format!(
                concat!(
                    "{model_name} {{spinner:.green}} [{{elapsed_precise}}] ",
                    "[{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})"
                ),
                model_name = self.get_model_name()
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to compute the features
        embedding
            .chunks_mut(graph.get_number_of_nodes() as usize)
            .progress_with(features_progress_bar)
            .zip(self.iter_anchor_nodes_buckets(graph)?)
            .for_each(|(empty_feature, bucket)| unsafe {
                self.compute_unchecked_feature_from_bucket(graph, bucket, empty_feature);
            });

        Ok(())
    }
}
