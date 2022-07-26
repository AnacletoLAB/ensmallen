use graph::{EdgeTypeT, Graph, NodeT};
use num::Zero;
use funty::Integral;
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

pub(crate) fn must_not_be_zero<F>(
    value: Option<F>,
    default: F,
    variable_name: &str,
) -> Result<F, String>
where
    F: Zero,
{
    let value = value.unwrap_or(default);
    if value.is_zero() {
        return Err(format!(
            concat!(
                "The provided {variable_name} is zero. ",
                "The {variable_name} should be strictly greater than zero."
            ),
            variable_name = variable_name
        ));
    }
    Ok(value)
}

// Initialize the model with weights and bias in the range (-1 / sqrt(k), +1 / sqrt(k))
fn get_random_weight(random_state: u64, scale_factor: f32) -> f32 {
    (2.0 * random_f32(splitmix64(random_state)) - 1.0) * 6.0 / scale_factor
}

pub(crate) fn populate_vectors(
    vectors: &mut [&mut [f32]],
    random_state: u64,
    scale_factors: &[f32],
) {
    vectors
        .iter_mut()
        .zip(scale_factors.iter().copied())
        .for_each(|(vector, scale_factor)| {
            vector.par_iter_mut().enumerate().for_each(|(i, weight)| {
                *weight = get_random_weight(random_state + i as u64, scale_factor);
            })
        });
}

pub(crate) fn get_random_vector(capacity: usize, random_state: u64, scale_factor: f32) -> Vec<f32> {
    (0..capacity)
        .map(|i| get_random_weight(random_state + i as u64, scale_factor))
        .collect()
}

pub(crate) fn compute_prior(subset_size: f32, total_size: f32) -> f32 {
    (1.0 + subset_size)
            / total_size
            // Adding the epsilon is necessary because the division may destroy enough
            // resolution to make the prior equal to zero.
            + f32::EPSILON
}

pub(crate) fn get_node_prior(graph: &Graph, node_id: NodeT, learning_rate: f32) -> f32 {
    compute_prior(
        unsafe { graph.get_unchecked_node_degree_from_node_id(node_id) as f32 },
        graph.get_number_of_nodes() as f32,
    ) / learning_rate
}

pub(crate) fn get_edge_type_prior(
    graph: &Graph,
    edge_type_id: EdgeTypeT,
    learning_rate: f32,
) -> f32 {
    compute_prior(
        unsafe { graph.get_unchecked_edge_count_from_edge_type_id(Some(edge_type_id)) as f32 },
        graph.get_number_of_directed_edges() as f32,
    ) / learning_rate
}

pub(crate) fn get_node_priors(graph: &Graph, node_ids: &[NodeT], learning_rate: f32) -> Vec<f32> {
    node_ids
        .iter()
        .copied()
        .map(|node_id| get_node_prior(graph, node_id, learning_rate))
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub enum MatrixShape {
    OneDimensional(usize),
    BiDimensional(usize, usize),
    ThreeDimensional(usize, usize, usize),
}

impl MatrixShape {
    pub fn size(&self) -> usize {
        match *self {
            MatrixShape::OneDimensional(one) => one,
            MatrixShape::BiDimensional(one, two) => one * two,
            MatrixShape::ThreeDimensional(one, two, three) => one * two * three,
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            MatrixShape::OneDimensional(_) => 1,
            MatrixShape::BiDimensional(_, _) => 2,
            MatrixShape::ThreeDimensional(_, _, _) => 3,
        }
    }
}

impl From<(usize,)> for MatrixShape {
    fn from(shape: (usize,)) -> Self {
        MatrixShape::OneDimensional(shape.0)
    }
}

impl From<(usize, usize)> for MatrixShape {
    fn from(shape: (usize, usize)) -> Self {
        MatrixShape::BiDimensional(shape.0, shape.1)
    }
}

impl From<(usize, usize, usize)> for MatrixShape {
    fn from(shape: (usize, usize, usize)) -> Self {
        MatrixShape::ThreeDimensional(shape.0, shape.1, shape.2)
    }
}

impl core::ops::Index<isize> for MatrixShape {
    type Output = usize;

    fn index(&self, mut index: isize) -> &Self::Output {
        let len = self.len() as isize;
        if index >= len || index <= -len {
            panic!(
                concat!(
                    "The provided index {} is not within the accepted bounds ",
                    "of the current shape {:?}."
                ),
                index, self
            )
        }

        index = (len + index) % len;

        match self {
            MatrixShape::OneDimensional(one) => one,
            MatrixShape::BiDimensional(one, two) => match index {
                0 => one,
                1 => two,
                _ => unreachable!("The shape is 2D."),
            },
            MatrixShape::ThreeDimensional(one, two, three) => match index {
                0 => one,
                1 => two,
                2 => three,
                _ => unreachable!("The shape is 3D."),
            },
        }
    }
}

pub trait FeatureType: Send + Sync + Integral + TryInto<usize> + TryFrom<usize> {}

impl FeatureType for u64 {}
impl FeatureType for u32 {}
impl FeatureType for u16 {}
impl FeatureType for u8 {}
