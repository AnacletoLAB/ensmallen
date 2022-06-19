use express_measures::{
    cosine_similarity_sequential_unchecked, euclidean_distance_sequential_unchecked,
};

#[derive(Clone, Debug, Copy)]
pub enum EdgeEmbeddingMethod {
    CosineSimilarity,
    EuclideanDistance,
    Hadamard,
}

impl TryFrom<String> for EdgeEmbeddingMethod {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "CosineSimilarity" => Ok(EdgeEmbeddingMethod::CosineSimilarity),
            "EuclideanDistance" => Ok(EdgeEmbeddingMethod::EuclideanDistance),
            "Hadamard" => Ok(EdgeEmbeddingMethod::Hadamard),
            _ => Err(format!(
                concat!(
                    "The provided edge embedding method name {} is not supported. ",
                    "The supported edge embedding method names are `CosineSimilarity`, ",
                    "`EuclideanDistance` and `Hadamard`."
                ),
                value
            )),
        }
    }
}

impl EdgeEmbeddingMethod {
    pub fn get_dimensionality(&self, dimension: usize) -> usize {
        match self {
            EdgeEmbeddingMethod::CosineSimilarity => 1,
            EdgeEmbeddingMethod::EuclideanDistance => 1,
            EdgeEmbeddingMethod::Hadamard => dimension,
        }
    }

    /// Returns method to compute the edge embedding.
    pub fn get_method(&self) -> fn(&[f32], &[f32]) -> Vec<f32> {
        match self {
            EdgeEmbeddingMethod::CosineSimilarity => {
                |a: &[f32], b: &[f32]| vec![unsafe { cosine_similarity_sequential_unchecked(a, b) }]
            }
            EdgeEmbeddingMethod::EuclideanDistance => |a: &[f32], b: &[f32]| {
                vec![unsafe { euclidean_distance_sequential_unchecked(a, b) }]
            },
            EdgeEmbeddingMethod::Hadamard => |a: &[f32], b: &[f32]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a * feature_b)
                    .collect::<Vec<f32>>()
            },
        }
    }
}
