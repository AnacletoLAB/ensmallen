use std::collections::{BTreeMap, HashSet};

// Types used to represent edges, nodes and their types.
pub type NodeT = usize;
pub type EdgeT = usize;
pub type WeightT = f64;
pub type ParamsT = f64;
pub type NodeTypeT = u16;
pub type EdgeTypeT = u16;
pub(crate) type GraphDictionary = BTreeMap<(NodeT, NodeT), Option<ConstructorEdgeMetadata>>;

/// Metadata of the edges of the graphs used for every graph.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeMetadata {
    pub edge_id: EdgeT,
    pub edge_types: Option<HashSet<EdgeTypeT>>,
}

/// Metadata of the edges used to describe both homogeneous and heterogeneous graphs and multi-graphs.
///
/// It used during the construction process of the graphs, while another smaller one is used for the actual structure.
#[derive(Clone)]
pub(crate) struct ConstructorEdgeMetadata {
    edge_types: Option<Vec<EdgeTypeT>>,
    weights: Option<Vec<WeightT>>,
}

impl ConstructorEdgeMetadata {
    /// Return built ConstructorEdgeMetadata object.
    ///
    /// When no meta-data is expected to be necessary, a None is returned instead.
    ///
    /// # Arguments
    ///
    /// * `has_weights`: bool - Wethever the graph has weights.
    /// * `has_edge_types`: bool - Wethever the graph has edge types.
    pub(crate) fn new(has_weights: bool, has_edge_types: bool) -> Option<ConstructorEdgeMetadata> {
        if has_edge_types || has_weights {
            Some(ConstructorEdgeMetadata {
                edge_types: if has_edge_types {
                    Some(Vec::new())
                } else {
                    None
                },
                weights: if has_weights { Some(Vec::new()) } else { None },
            })
        } else {
            None
        }
    }

    /// Add given metadata (when they are not None).
    ///
    /// # Arguments
    ///
    /// * `weight`: Option<WeightT> - Weight to be added.
    /// * `edge_type`: Option<EdgeTypeT> - Edge type to be added
    pub(crate) fn add(&mut self, weight: Option<WeightT>, edge_type: Option<EdgeTypeT>) {
        if let Some(w) = weight {
            if let Some(ws) = &mut self.weights {
                ws.push(w);
            }
        }
        if let Some(et) = edge_type {
            if let Some(ets) = &mut self.edge_types {
                ets.push(et)
            }
        }
    }

    /// Extend given metadata (when they are not None).
    ///
    /// # Arguments
    ///
    /// * `weight`: Option<WeightT> - Weight to be appended.
    /// * `edge_type`: Option<EdgeTypeT> - Edge type to be appended
    pub(crate) fn extend(
        &mut self,
        weights: Option<Vec<WeightT>>,
        edge_types: Option<Vec<EdgeTypeT>>,
    ) {
        if let Some(ws) = weights {
            if let Some(sws) = &mut self.weights {
                sws.extend(ws);
            }
        }
        if let Some(ets) = edge_types {
            if let Some(sets) = &mut self.edge_types {
                sets.extend(ets)
            }
        }
    }

    /// Set metadata.
    ///
    /// # Arguments
    ///
    /// * `weights`: Option<WeightT> - Weights to be set.
    /// * `edge_types`: Option<EdgeTypeT> - Edge types to be set
    pub(crate) fn set(
        &mut self,
        weights: Option<Vec<WeightT>>,
        edge_types: Option<Vec<EdgeTypeT>>,
    ) {
        self.weights = weights;
        self.edge_types = edge_types;
    }

    /// Return boolean representing if given edge type is present.
    ///
    /// # Arguments
    /// * `edge_type`: Option<EdgeTypeT> - The edge type to check for.
    pub(crate) fn contains_edge_type(&self, edge_type: Option<EdgeTypeT>) -> bool {
        if edge_type.is_none() && self.edge_types.is_none() {
            return true;
        }
        if let Some(et) = edge_type {
            if let Some(ets) = &self.edge_types {
                return ets.contains(&et);
            }
        }
        false
    }

    /// Returns vector of edge types as HashSet.
    pub(crate) fn to_edge_types_set(&self) -> Option<HashSet<EdgeTypeT>> {
        self.edge_types
            .clone()
            .map(|et| et.into_iter().collect::<HashSet<EdgeTypeT>>())
    }

    /// Return length of the vocabulary.
    pub fn len(&self) -> usize {
        if let Some(sws) = &self.weights {
            return sws.len();
        }
        if let Some(sets) = &self.edge_types {
            return sets.len()
        }
        unreachable!("Either the weights or the edge types are certainly set.");
    }
}

impl Iterator for ConstructorEdgeMetadata {
    type Item = (Option<WeightT>, Option<EdgeTypeT>);

    /// Returns new value when depopulating meta-data during building process.
    fn next(&mut self) -> Option<Self::Item> {
        // either weights or edge types MUST be some.`

        // if there are no edge types but there are weights, then we have only one weight for the edge.
        let edge_type = if let Some(ets) = &mut self.edge_types {
            ets.pop()
        } else {
            None
        };

        let weight = if let Some(ws) = &mut self.weights {
            ws.pop()
        } else {
            None
        };

        if weight.is_none() && edge_type.is_none() {
            None
        } else {
            Some((weight, edge_type))
        }
    }
}

pub trait ToFromUsize {
    fn from_usize(v: usize) -> Self;
    fn to_usize(v: Self) -> usize;
}

macro_rules! impl_to_from_usize {
    ($($ty:ty)*) => {
        $(
            impl ToFromUsize for $ty {
                #[inline(always)]
                fn from_usize(v: usize) -> $ty {
                    v as $ty
                }
                #[inline(always)]
                fn to_usize(v: $ty) -> usize {
                    v as usize
                }
            }
        )*
    }
}

impl_to_from_usize!(u8 u16 u32 u64 usize);
