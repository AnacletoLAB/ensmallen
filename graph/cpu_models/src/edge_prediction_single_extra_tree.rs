use crate::{EdgeEmbeddingMethod, NodeFeaturesBasedEdgePrediction};
use express_measures::{BinaryConfusionMatrix, BinaryMetricName, ThreadUnsigned};
use graph::Graph;
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fmt::Debug;
use std::ops::AddAssign;
use vec_rand::{random_f32, sample_uniform, splitmix64};

pub trait LinearInterpolation {
    /// Returns the interpolation of weight amount between self and the other value.
    ///
    /// # Implementation details
    /// The interpolation is linear and follow the linear interpolation
    /// formula, that is `self * weight + other * (1.0 - weight)`.
    ///
    /// # Arguments
    /// * `other` - The other value with which to linearly interpolate self.
    /// * `weight`: f32 - A weight between 0 and 1 to use in the interpolation.
    ///
    fn linear_interpolation(self, other: Self, weight: f32) -> Self;
}

impl<T> LinearInterpolation for T
where
    T: Into<f32> + From<f32>,
{
    fn linear_interpolation(self, other: Self, weight: f32) -> Self {
        let self_f32: f32 = self.into();
        let other_f32: f32 = other.into();
        (self_f32 * weight + (1.0 - weight) * other_f32).into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Split<NodeFeaturePositionType, AttributeType>
where
    NodeFeaturePositionType: ThreadUnsigned,
{
    attribute_position: NodeFeaturePositionType,
    /// Split position.
    /// We use a generic that supports comparison.
    attribute_split_value: AttributeType,
    /// Precision of this node. Equal to positive predictive value.
    positive_predictive_value: f64,
    /// Negative predictive value of this node.
    negative_predictive_value: f64,
    /// Sign of the attributes.
    sign: bool,
}

impl<NodeFeaturePositionType, AttributeType> Split<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    pub fn new(
        attribute_position: usize,
        attribute_split_value: AttributeType,
        positive_predictive_value: f64,
        negative_predictive_value: f64,
        sign: bool,
    ) -> Self {
        Self {
            attribute_position: NodeFeaturePositionType::try_from(attribute_position).unwrap(),
            attribute_split_value,
            positive_predictive_value,
            negative_predictive_value,
            sign,
        }
    }

    /// Return prediction of the edge according to this splitter.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    pub fn predict(&self, edge_embedding: &[AttributeType]) -> (f64, bool) {
        let edge_feature = edge_embedding[self.attribute_position.try_into().unwrap()];
        let prediction = self.sign && edge_feature > self.attribute_split_value
            || !self.sign && edge_feature <= self.attribute_split_value;
        let prediction_score = if prediction {
            self.positive_predictive_value
        } else {
            self.negative_predictive_value
        };
        (prediction_score, prediction)
    }

    /// Updates the provide attribute slices using the object split.
    fn bound_attribute_values(
        &self,
        minimum_attribute_values: &mut [AttributeType],
        maximum_attribute_values: &mut [AttributeType],
        mut label: bool,
    ) {
        if !self.sign {
            label = !label;
        }
        if label {
            minimum_attribute_values[self.attribute_position.try_into().unwrap()] =
                self.attribute_split_value;
        } else {
            maximum_attribute_values[self.attribute_position.try_into().unwrap()] =
                self.attribute_split_value;
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    NodeFeaturePositionType: ThreadUnsigned,
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
{
    split: Split<NodeFeaturePositionType, AttributeType>,
    confusion_matrix: BinaryConfusionMatrix,
    metric: BinaryMetricName,
}

impl<NodeFeaturePositionType, AttributeType> SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    /// Return a new Split Builder object.
    ///
    /// # Arguments
    /// * `metric`: BinaryMetricName - The binary metric name to use as score.
    /// * `random_state`: u64 - The random state to use.
    /// * `minimum_attribute_values`: &[AttributeType] - Slice of the minimum values of the attributes.
    /// * `maximum_attribute_values`: &[AttributeType] - Slice of the maximum values of the attributes.
    pub fn new(
        metric: BinaryMetricName,
        random_state: u64,
        minimum_attribute_values: &[AttributeType],
        maximum_attribute_values: &[AttributeType],
    ) -> Self {
        let attribute_position =
            sample_uniform(minimum_attribute_values.len() as u64, random_state);
        let attribute_split_value = minimum_attribute_values[attribute_position]
            .linear_interpolation(
                maximum_attribute_values[attribute_position],
                random_f32(random_state),
            );
        Self {
            metric,
            split: Split::new(
                attribute_position,
                attribute_split_value,
                0.0,
                0.0,
                splitmix64(random_state) > u64::MAX / 2,
            ),
            confusion_matrix: BinaryConfusionMatrix::default(),
        }
    }

    /// Update the state of the current confusion matrix by running the split prediction.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    /// * `label`: bool - The
    pub fn update(&mut self, edge_embedding: &[AttributeType], label: bool) {
        self.confusion_matrix +=
            BinaryConfusionMatrix::from_tuple(label, self.split.predict(edge_embedding).1);
    }

    /// Return number of updates.
    pub fn get_updates_number(&self) -> usize {
        self.confusion_matrix.get_number_of_samples()
    }
}

impl<NodeFeaturePositionType, AttributeType> PartialOrd
    for SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.confusion_matrix
            .get_binary_metric(self.metric)
            .partial_cmp(&other.confusion_matrix.get_binary_metric(other.metric))
    }
}

impl<NodeFeaturePositionType, AttributeType> Eq
    for SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
}

impl<NodeFeaturePositionType, AttributeType> Ord
    for SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<NodeFeaturePositionType, AttributeType> Into<Split<NodeFeaturePositionType, AttributeType>>
    for SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn into(mut self) -> Split<NodeFeaturePositionType, AttributeType> {
        self.split.negative_predictive_value =
            self.confusion_matrix.get_binary_negative_predictive_value();
        self.split.positive_predictive_value = self.confusion_matrix.get_binary_precision();
        self.split
    }
}

impl<NodeFeaturePositionType, AttributeType> core::ops::AddAssign
    for SplitBuilder<NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.metric, other.metric);
        self.confusion_matrix += other.confusion_matrix;
    }
}

#[derive(Clone, Debug)]
pub struct Node<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned,
{
    /// The position of the node in the tree.
    /// We use a generic as, when generating a significant amount of
    /// small trees for a large forest, using 2 bytes or 8 bytes (for an usize)
    /// can make a large difference.
    id: NodeIdType,
    /// The splitter for this node.
    split: Split<NodeFeaturePositionType, AttributeType>,
    /// Child node ID for the lower or equal values, the negative edges class.
    left_child_node_id: Option<NodeIdType>,
    /// Child node ID for the higher values, the positive edges class.
    right_child_node_id: Option<NodeIdType>,
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType>
    Node<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned,
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    /// Returns whether the current node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.left_child_node_id.is_none() && self.right_child_node_id.is_none()
    }

    /// Return prediction of the edge according to this node.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    pub fn predict(&self, edge_embedding: &[AttributeType]) -> (f64, bool) {
        self.split.predict(edge_embedding)
    }

    /// Return child node id according to prediction.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    pub fn get_best_child_node_id(
        &self,
        edge_embedding: &[AttributeType],
    ) -> (f64, bool, Option<NodeIdType>) {
        let (prediction_score, prediction) = self.predict(edge_embedding);
        if prediction {
            (prediction_score, prediction, self.left_child_node_id)
        } else {
            (prediction_score, prediction, self.right_child_node_id)
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned,
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
{
    /// Node Id of the parent node. A root node does not have a parent Id.
    parent_id: Option<NodeIdType>,
    /// Node Id of the node builder.
    id: NodeIdType,
    /// The minimum number of samples before considering the node ready.
    number_of_samples: usize,
    /// The attribute position used in this node.
    split_builders: Vec<SplitBuilder<NodeFeaturePositionType, AttributeType>>,
    /// Child node ID for the lower or equal values, the negative edges class.
    left_child_node_id: Option<NodeIdType>,
    /// Child node ID for the higher values, the positive edges class.
    right_child_node_id: Option<NodeIdType>,
    /// The best split.
    best_split: Option<Split<NodeFeaturePositionType, AttributeType>>,
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType>
    NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned,
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    /// Return a new Node Builder object.
    ///
    /// # Arguments
    /// * `parent_id`: Option<NodeIdType> - Node Id of the parent node. A root node does not have a parent Id.
    /// * `id`: Option<NodeIdType> - Node Id of the parent node. A root node does not have a parent Id.
    /// * `metric`: BinaryMetricName - The binary metric name to use as score.
    /// * `number_of_splits`: usize - The number of splits to create.
    /// * `number_of_samples`: usize - The minimum number of samples before considering the node ready.
    /// * `random_state`: u64 - The random state to use.
    /// * `minimum_attribute_values`: &[AttributeType] - Slice of the minimum values of the attributes.
    /// * `maximum_attribute_values`: &[AttributeType] - Slice of the maximum values of the attributes.
    pub fn new(
        parent_id: Option<NodeIdType>,
        id: NodeIdType,
        metric: BinaryMetricName,
        number_of_splits: usize,
        number_of_samples: usize,
        mut random_state: u64,
        minimum_attribute_values: &[AttributeType],
        maximum_attribute_values: &[AttributeType],
    ) -> Self {
        Self {
            parent_id,
            id,
            number_of_samples,
            split_builders: (0..number_of_splits)
                .map(|_| {
                    random_state = splitmix64(random_state);
                    SplitBuilder::new(
                        metric,
                        random_state,
                        minimum_attribute_values,
                        maximum_attribute_values,
                    )
                })
                .collect(),
            left_child_node_id: None,
            right_child_node_id: None,
            best_split: None,
        }
    }

    /// Update the state of the split builders by running the split predictions.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    /// * `label`: bool - The
    pub fn update(&mut self, edge_embedding: &[AttributeType], label: bool) {
        self.split_builders.iter_mut().for_each(|split_builder| {
            split_builder.update(edge_embedding, label);
        });
    }

    /// Return number of updates.
    pub fn get_updates_number(&self) -> usize {
        self.split_builders[0].get_updates_number()
    }

    /// Returns whether the node in building is considered ready.
    pub fn is_ready(&self) -> bool {
        self.get_updates_number() > self.number_of_samples
    }

    /// Returns whether the node builder is rasterized
    pub fn is_rasterized(&self) -> bool {
        self.split_builders.is_empty()
    }

    /// Returns whether the current node is a leaf.
    fn is_leaf(&self) -> bool {
        self.left_child_node_id.is_none() && self.right_child_node_id.is_none()
    }

    /// Returns whether the current node is a root node.
    fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Rasterize the builder.
    ///
    /// # Arguments
    /// * `left_child_node_id`: NodeIdType - The ID of the left child node (for the negative class).
    /// * `right_child_node_id`: NodeIdType - The ID of the right child node (for the positive class).
    fn rasterize(&mut self, left_child_node_id: NodeIdType, right_child_node_id: NodeIdType) {
        self.left_child_node_id = Some(left_child_node_id);
        self.right_child_node_id = Some(right_child_node_id);
        self.best_split = Some(
            core::mem::replace(&mut self.split_builders, Vec::new())
                .into_iter()
                .max()
                .unwrap()
                .into(),
        );
        self.split_builders = Vec::new();
    }

    /// Return prediction of the edge according to this node.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    fn predict(&self, edge_embedding: &[AttributeType]) -> (f64, bool) {
        if let Some(split) = &self.best_split {
            return split.predict(edge_embedding);
        }
        panic!("The node builder was not rasterized!");
    }

    /// Return child node id according to prediction.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    fn get_best_child_node_id(&self, edge_embedding: &[AttributeType]) -> Option<NodeIdType> {
        if self.predict(edge_embedding).1 {
            self.right_child_node_id
        } else {
            self.left_child_node_id
        }
    }

    /// Updates the provide attribute slices using the object split.
    fn bound_attribute_values(
        &self,
        child_node_id: NodeIdType,
        minimum_attribute_values: &mut [AttributeType],
        maximum_attribute_values: &mut [AttributeType],
    ) {
        if let (Some(left_child_node_id), Some(right_child_node_id), Some(best_split)) = (
            self.left_child_node_id,
            self.right_child_node_id,
            &self.best_split,
        ) {
            if child_node_id == left_child_node_id {
                best_split.bound_attribute_values(
                    minimum_attribute_values,
                    maximum_attribute_values,
                    false,
                );
            } else if child_node_id == right_child_node_id {
                best_split.bound_attribute_values(
                    minimum_attribute_values,
                    maximum_attribute_values,
                    true,
                );
            } else {
                unreachable!(
                    concat!(
                        "The provided node id {:?} is not a child node. ",
                        "Available nodes were {:?} and {:?}"
                    ),
                    child_node_id, left_child_node_id, right_child_node_id
                )
            }
        }
    }
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType>
    Into<Node<NodeIdType, NodeFeaturePositionType, AttributeType>>
    for NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn into(self) -> Node<NodeIdType, NodeFeaturePositionType, AttributeType> {
        Node {
            id: self.id,
            split: self.best_split.unwrap(),
            left_child_node_id: self.left_child_node_id,
            right_child_node_id: self.right_child_node_id,
        }
    }
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType> core::ops::AddAssign
    for NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn add_assign(&mut self, other: Self) {
        if !self.is_rasterized() {
            assert_eq!(self.number_of_samples, other.number_of_samples);
            self.split_builders
                .iter_mut()
                .zip(other.split_builders.into_iter())
                .for_each(|(a, b)| {
                    *a += b;
                });
        }
    }
}

unsafe impl<NodeIdType, NodeFeaturePositionType, AttributeType> Sync
    for NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
}
unsafe impl<NodeIdType, NodeFeaturePositionType, AttributeType> Send
    for NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
}

#[derive(Clone, Debug)]
struct TreeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
{
    metric: BinaryMetricName,
    number_of_splits: usize,
    number_of_samples: usize,
    random_state: u64,
    depth: NodeIdType,
    tree: Vec<NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>>,
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType>
    TreeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation + Send + Sync,
    NodeIdType: ThreadUnsigned + AddAssign<NodeIdType>,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    /// Return a new Root Node Builder object.
    ///
    /// # Arguments
    /// * `metric`: BinaryMetricName - The binary metric name to use as score.
    /// * `number_of_splits`: usize - The number of splits to create.
    /// * `number_of_samples`: usize - The minimum number of samples before considering the node ready.
    /// * `random_state`: u64 - The random state to use.
    /// * `minimum_attribute_values`: &[AttributeType] - Slice of the minimum values of the attributes.
    /// * `maximum_attribute_values`: &[AttributeType] - Slice of the maximum values of the attributes.
    pub fn new(
        metric: BinaryMetricName,
        number_of_splits: usize,
        number_of_samples: usize,
        random_state: u64,
        minimum_attribute_values: &[AttributeType],
        maximum_attribute_values: &[AttributeType],
    ) -> Self {
        Self {
            metric,
            number_of_splits,
            number_of_samples,
            random_state,
            depth: NodeIdType::try_from(0).unwrap(),
            tree: vec![NodeBuilder::new(
                None,
                NodeIdType::try_from(0).unwrap(),
                metric,
                number_of_splits,
                number_of_samples,
                random_state,
                minimum_attribute_values,
                maximum_attribute_values,
            )],
        }
    }

    /// Update the state of the split builders by running the split predictions.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    /// * `label`: bool - The
    pub fn update(&mut self, edge_embedding: &[AttributeType], label: bool) {
        let mut node_id = 0;
        while self.tree[node_id].is_rasterized() {
            // Recursive step: predict which of the leaf nodes
            // should be explored.
            node_id = self.tree[node_id]
                .get_best_child_node_id(edge_embedding)
                .unwrap()
                .try_into()
                .unwrap();
        }
        self.tree[node_id].update(edge_embedding, label);
    }

    /// Rasterize node builders that are in ready state.
    pub fn rasterize_ready_nodes(
        &mut self,
        minimum_attribute_values: &[AttributeType],
        maximum_attribute_values: &[AttributeType],
    ) {
        let mut new_nodes_count = 0;
        let new_nodes = (0..self.tree.len())
            .filter_map(|current_node_id| {
                if self.tree[current_node_id].is_ready()
                    && !self.tree[current_node_id].is_rasterized()
                {
                    let mut minimum_attribute_values = minimum_attribute_values.to_vec();
                    let mut maximum_attribute_values = maximum_attribute_values.to_vec();
                    let mut node_id: usize = current_node_id;
                    while !self.tree[node_id].is_root() {
                        let child_node_id = NodeIdType::try_from(node_id).unwrap();
                        node_id = self.tree[node_id].parent_id.unwrap().try_into().unwrap();
                        self.tree[current_node_id].bound_attribute_values(
                            child_node_id,
                            minimum_attribute_values.as_mut_slice(),
                            maximum_attribute_values.as_mut_slice(),
                        );
                    }
                    let left_child_node_id =
                        NodeIdType::try_from(self.tree.len() + new_nodes_count).unwrap();
                    new_nodes_count += 1;
                    let right_child_node_id =
                        NodeIdType::try_from(self.tree.len() + new_nodes_count).unwrap();
                    new_nodes_count += 1;

                    self.tree[current_node_id].rasterize(left_child_node_id, right_child_node_id);

                    let mut left_minimum_attribute_values = minimum_attribute_values.clone();
                    let mut left_maximum_attribute_values = maximum_attribute_values.clone();
                    self.tree[current_node_id].bound_attribute_values(
                        left_child_node_id,
                        left_minimum_attribute_values.as_mut_slice(),
                        left_maximum_attribute_values.as_mut_slice(),
                    );

                    let mut right_minimum_attribute_values = minimum_attribute_values.clone();
                    let mut right_maximum_attribute_values = maximum_attribute_values.clone();
                    self.tree[current_node_id].bound_attribute_values(
                        right_child_node_id,
                        right_minimum_attribute_values.as_mut_slice(),
                        right_maximum_attribute_values.as_mut_slice(),
                    );

                    self.random_state = splitmix64(self.random_state);
                    let left_child_node: NodeBuilder<
                        NodeIdType,
                        NodeFeaturePositionType,
                        AttributeType,
                    > = NodeBuilder::new(
                        Some(NodeIdType::try_from(current_node_id).unwrap()),
                        left_child_node_id,
                        self.metric,
                        self.number_of_splits,
                        self.number_of_samples,
                        self.random_state,
                        left_minimum_attribute_values.as_slice(),
                        left_maximum_attribute_values.as_slice(),
                    );
                    self.random_state = splitmix64(self.random_state);
                    let right_child_node: NodeBuilder<
                        NodeIdType,
                        NodeFeaturePositionType,
                        AttributeType,
                    > = NodeBuilder::new(
                        Some(NodeIdType::try_from(current_node_id).unwrap()),
                        right_child_node_id,
                        self.metric,
                        self.number_of_splits,
                        self.number_of_samples,
                        self.random_state,
                        right_minimum_attribute_values.as_slice(),
                        right_maximum_attribute_values.as_slice(),
                    );
                    Some(vec![left_child_node, right_child_node])
                } else {
                    None
                }
            })
            .flat_map(|vector| vector)
            .collect::<Vec<NodeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>>>();

        if !new_nodes.is_empty() {
            self.depth += NodeIdType::try_from(1).unwrap();
        }

        self.tree.extend(new_nodes);
    }
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType> core::ops::AddAssign
    for TreeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.metric, other.metric);
        assert_eq!(self.number_of_splits, other.number_of_splits);
        assert_eq!(self.random_state, other.random_state);
        assert_eq!(self.depth, other.depth);
        self.tree
            .iter_mut()
            .zip(other.tree.into_iter())
            .for_each(|(a, b)| {
                *a += b;
            });
    }
}

impl<NodeIdType, NodeFeaturePositionType, AttributeType>
    Into<Vec<Node<NodeIdType, NodeFeaturePositionType, AttributeType>>>
    for TreeBuilder<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation + Send + Sync,
    NodeIdType: ThreadUnsigned + AddAssign<NodeIdType>,
    NodeFeaturePositionType: ThreadUnsigned + Debug,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn into(mut self) -> Vec<Node<NodeIdType, NodeFeaturePositionType, AttributeType>> {
        let mut counter = 0;
        // We need to remap the nodes to a dense set as we do not know which of them is actually
        // ready to use. Some of them may have not seen enough samples to be considered ready.
        self.tree.iter_mut().for_each(|node_builder| {
            if node_builder.is_ready() {
                node_builder.id = NodeIdType::try_from(counter).unwrap();
                counter += 1;
            }
        });
        // We proceed to apply the remapping and we prune tree branches to child nodes
        // that did not see enough samples and are therefore not ready.
        (0..self.tree.len()).for_each(|node_id| {
            // If the node has a left child, we try to update it, or if it is not
            // ready we prune it.
            self.tree[node_id].left_child_node_id =
                self.tree[node_id]
                    .left_child_node_id
                    .and_then(|left_child_id| {
                        if self.tree[left_child_id.try_into().unwrap()].is_ready() {
                            Some(self.tree[left_child_id.try_into().unwrap()].id)
                        } else {
                            None
                        }
                    });
            // If the node has a right child, we try to update it, or if it is not
            // ready we prune it.
            self.tree[node_id].right_child_node_id = self.tree[node_id]
                .right_child_node_id
                .and_then(|right_child_id| {
                    if self.tree[right_child_id.try_into().unwrap()].is_ready() {
                        Some(self.tree[right_child_id.try_into().unwrap()].id)
                    } else {
                        None
                    }
                });
        });
        // We are now ready to convert the tree builder into a tree!
        self.tree
            .into_iter()
            .filter_map(|node_builder| {
                if node_builder.is_ready() {
                    Some(node_builder.into())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct EdgePredictionSingleExtraTree<NodeIdType, NodeFeaturePositionType, AttributeType>
where
    AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned,
    NodeFeaturePositionType: ThreadUnsigned,
{
    /// The binary metric name to use as score.
    metric: BinaryMetricName,
    /// The tree structure
    tree: Vec<Node<NodeIdType, NodeFeaturePositionType, AttributeType>>,
    /// The name of the method to use to compute the edge embedding.
    edge_embedding_method_name: EdgeEmbeddingMethod,
    /// Whether to train this model by sampling only edges with nodes with different node types.
    sample_only_edges_with_heterogeneous_node_types: bool,
    /// Number of edges to sample to compute each of the nodes.
    number_of_edges_to_sample_per_tree_node: usize,
    /// Number of splits to sample for each tree node.
    number_of_splits_per_tree_node: usize,
    /// Rate of negative edges over total, equal to negative / (positive + negative).
    negative_edges_rate: f64,
    /// The expected size of the input.
    input_size: NodeFeaturePositionType,
    /// Maximal depth of tree.
    depth: NodeIdType,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
}

impl<NodeIdType, NodeFeaturePositionType>
    EdgePredictionSingleExtraTree<NodeIdType, NodeFeaturePositionType, f32>
where
    NodeIdType: ThreadUnsigned + AddAssign<NodeIdType>,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    /// Return new instance of Single Extra Tree for edge prediction.
    ///
    /// # Arguments
    /// * `metric`: Option<BinaryMetricName> - The binary metric name to use as score. By default, F1 Score.
    /// * `edge_embedding_method_name`: Option<EdgeEmbeddingMethod> - The embedding method to use. By default the cosine similarity is used.
    /// * `number_of_edges_to_sample_per_tree_node`: Option<usize> - Number of edges to sample to compute each of the nodes. By default 2048.
    /// * `number_of_splits_per_tree_node`: Option<usize> - Number of splits to sample for each tree node. By default 10.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types. By default false.
    /// * `negative_edges_rate`: Option<f64> - The rate of negative edges over the total, by default 0.5.
    /// * `depth`: Option<NodeIdType> - The maximum depth of the three, by default 10.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(
        metric: Option<BinaryMetricName>,
        edge_embedding_method_name: Option<EdgeEmbeddingMethod>,
        number_of_edges_to_sample_per_tree_node: Option<usize>,
        number_of_splits_per_tree_node: Option<usize>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        negative_edges_rate: Option<f64>,
        depth: Option<NodeIdType>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        let number_of_edges_to_sample_per_tree_node =
            number_of_edges_to_sample_per_tree_node.unwrap_or(2048);
        let number_of_splits_per_tree_node = number_of_splits_per_tree_node.unwrap_or(10);
        if number_of_splits_per_tree_node == 0 {
            return Err(concat!(
                "The provided number of splits to sample per tree node is zero. ",
                "The number of splits should be strictly greater than zero."
            )
            .to_string());
        }
        if number_of_edges_to_sample_per_tree_node == 0 {
            return Err(concat!(
                "The provided number of edges to sample per tree node is zero. ",
                "The number of edges should be strictly greater than zero."
            )
            .to_string());
        }

        let edge_embedding_method_name =
            edge_embedding_method_name.unwrap_or(EdgeEmbeddingMethod::CosineSimilarity);

        Ok(Self {
            metric: metric.unwrap_or(BinaryMetricName::F1Score),
            tree: Vec::new(),
            edge_embedding_method_name,
            number_of_edges_to_sample_per_tree_node,
            number_of_splits_per_tree_node,
            sample_only_edges_with_heterogeneous_node_types:
                sample_only_edges_with_heterogeneous_node_types.unwrap_or(false),
            negative_edges_rate: negative_edges_rate.unwrap_or(0.5),
            input_size: NodeFeaturePositionType::try_from(0).unwrap(),
            depth: depth.unwrap_or(NodeIdType::try_from(10).unwrap()),
            random_state: random_state.unwrap_or(42),
        })
    }

    /// * `minimum_attribute_values`: &[AttributeType] - Slice of the minimum values of the attributes.
    /// * `maximum_attribute_values`: &[AttributeType] - Slice of the maximum values of the attributes.
    pub(crate) unsafe fn internal_fit(
        &mut self,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
        minimum_attribute_values: &[f32],
        maximum_attribute_values: &[f32],
    ) -> Result<(), String> {
        self.input_size = NodeFeaturePositionType::try_from(
            self.get_edge_embedding_method()
                .get_dimensionality(dimension),
        )
        .unwrap();

        let mut random_state = splitmix64(self.random_state);

        let mut tree: TreeBuilder<NodeIdType, NodeFeaturePositionType, f32> = TreeBuilder::new(
            self.metric,
            self.number_of_splits_per_tree_node,
            self.number_of_edges_to_sample_per_tree_node,
            random_state,
            minimum_attribute_values,
            maximum_attribute_values,
        );

        let method = self.edge_embedding_method_name.get_method();

        while tree.depth != self.depth {
            random_state = splitmix64(random_state);
            tree = graph
                .par_iter_edge_prediction_mini_batch(
                    self.random_state,
                    graph.get_number_of_directed_edges() as usize,
                    self.sample_only_edges_with_heterogeneous_node_types,
                    Some(self.negative_edges_rate),
                    Some(true),
                    None,
                    Some(true),
                    support,
                    graph_to_avoid,
                )?
                .map(|(src, dst, label)| {
                    let src = src as usize;
                    let dst = dst as usize;
                    let src_features = &node_features[src * dimension..(src + 1) * dimension];
                    let dst_features = &node_features[dst * dimension..(dst + 1) * dimension];
                    let edge_embedding = method(src_features, dst_features);
                    let mut tree = tree.clone();
                    tree.update(&edge_embedding, label);
                    tree
                })
                .reduce(
                    || tree.clone(),
                    |mut a, b| {
                        a += b;
                        a
                    },
                );
            tree.rasterize_ready_nodes(minimum_attribute_values, maximum_attribute_values);
        }

        self.tree = tree.into();

        Ok(())
    }

    /// Return prediction of the edge according to this splitter.
    ///
    /// # Arguments
    /// * `edge_embedding`: &[AttributeType] - Edge features of the edge to predict.
    pub(crate) fn predict_edge(&self, edge_embedding: &[f32]) -> (f64, bool) {
        let mut current_node_id = 0;
        loop {
            let (prediction_score, prediction, maybe_child_node) =
                self.tree[current_node_id].get_best_child_node_id(edge_embedding);
            if let Some(child_node_id) = maybe_child_node {
                current_node_id = child_node_id.try_into().unwrap();
            } else {
                return (prediction_score, prediction);
            }
        }
    }

    pub(crate) fn get_minimum_and_maximum_attributes(
        random_state: u64,
        negative_edges_rate: f64,
        sample_only_edges_with_heterogeneous_node_types: bool,
        edge_embedding_method_name: EdgeEmbeddingMethod,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(Vec<f32>, Vec<f32>), String> {
        let method = edge_embedding_method_name.get_method();
        Ok(graph
            .par_iter_edge_prediction_mini_batch(
                random_state,
                graph.get_number_of_directed_edges() as usize,
                sample_only_edges_with_heterogeneous_node_types,
                Some(negative_edges_rate),
                Some(true),
                None,
                Some(true),
                support,
                graph_to_avoid,
            )?
            .map(|(src, dst, _)| {
                let src = src as usize;
                let dst = dst as usize;
                let src_features = &node_features[src * dimension..(src + 1) * dimension];
                let dst_features = &node_features[dst * dimension..(dst + 1) * dimension];
                let edge_embedding = method(src_features, dst_features);
                (edge_embedding.clone(), edge_embedding)
            })
            .reduce(
                || (Vec::new(), Vec::new()),
                |(mut min_a, mut max_a), (min_b, max_b)| {
                    min_a.iter_mut().zip(min_b.into_iter()).for_each(|(a, b)| {
                        *a = a.min(b);
                    });
                    max_a.iter_mut().zip(max_b.into_iter()).for_each(|(a, b)| {
                        *a = a.max(b);
                    });
                    (min_a, max_a)
                },
            ))
    }
}

impl<NodeIdType, NodeFeaturePositionType> NodeFeaturesBasedEdgePrediction
    for EdgePredictionSingleExtraTree<NodeIdType, NodeFeaturePositionType, f32>
where
    //AttributeType: PartialOrd + Copy + Debug + LinearInterpolation,
    NodeIdType: ThreadUnsigned + AddAssign<NodeIdType>,
    NodeFeaturePositionType: ThreadUnsigned,
    <NodeIdType as TryFrom<usize>>::Error: Debug,
    <NodeIdType as TryInto<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryFrom<usize>>::Error: Debug,
    <NodeFeaturePositionType as TryInto<usize>>::Error: Debug,
{
    fn fit(
        &mut self,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(), String> {
        self.validate_features(graph, node_features, dimension)?;
        self.input_size = NodeFeaturePositionType::try_from(
            self.get_edge_embedding_method()
                .get_dimensionality(dimension),
        )
        .unwrap();

        let (minimum_attribute_values, maximum_attribute_values): (Vec<f32>, Vec<f32>) =
            Self::get_minimum_and_maximum_attributes(
                self.random_state,
                self.negative_edges_rate,
                self.sample_only_edges_with_heterogeneous_node_types,
                self.edge_embedding_method_name,
                graph,
                node_features,
                dimension,
                support,
                graph_to_avoid,
            )?;

        unsafe {
            self.internal_fit(
                graph,
                node_features,
                dimension,
                verbose,
                support,
                graph_to_avoid,
                &minimum_attribute_values,
                &maximum_attribute_values,
            )?
        };

        Ok(())
    }

    fn predict(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
    ) -> Result<(), String> {
        self.validate_features_for_prediction(predictions, graph, node_features, dimension)?;
        let method = self.edge_embedding_method_name.get_method();

        predictions
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .for_each(|(prediction, (_, src, dst))| {
                let src = src as usize;
                let dst = dst as usize;
                let src_features = &node_features[src * dimension..(src + 1) * dimension];
                let dst_features = &node_features[dst * dimension..(dst + 1) * dimension];
                let edge_embedding = method(src_features, dst_features);
                if self.predict_edge(edge_embedding.as_slice()).1 {
                    *prediction = 1.0;
                } else {
                    *prediction = 0.0;
                }
            });

        Ok(())
    }

    fn get_edge_embedding_method(&self) -> &EdgeEmbeddingMethod {
        &self.edge_embedding_method_name
    }

    fn is_trained(&self) -> bool {
        !self.tree.is_empty()
    }

    fn get_input_size(&self) -> usize {
        self.input_size.try_into().unwrap()
    }
}
