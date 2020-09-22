use super::*;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashSet};

// Types used to represent edges, nodes and their types.
pub type NodeT = usize;
pub type EdgeT = usize;
pub type WeightT = f64;
pub type ParamsT = f64;
pub type NodeTypeT = u16;
pub type EdgeTypeT = u16;
pub type Contexts = Vec<Vec<NodeT>>;
pub type Words = Vec<NodeT>;
pub type Frequencies = Vec<f64>;

/// Custom BTreeMap with some helper methods
pub(crate) struct GraphDictionary {
    tree: BTreeMap<(NodeT, NodeT), Option<ConstructorEdgeMetadata>>,
}

impl GraphDictionary {
    pub(crate) fn new() -> GraphDictionary {
        GraphDictionary {
            tree: BTreeMap::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tree.len()
    }

    pub(crate) fn get(&self, key: &(NodeT, NodeT)) -> Option<&Option<ConstructorEdgeMetadata>> {
        self.tree.get(key)
    }

    pub(crate) fn contains_key(&self, key: &(NodeT, NodeT)) -> bool {
        self.tree.contains_key(key)
    }

    pub(crate) fn insert(
        &mut self,
        key: (NodeT, NodeT),
        value: Option<ConstructorEdgeMetadata>,
    ) -> Option<Option<ConstructorEdgeMetadata>> {
        self.tree.insert(key, value)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub(crate) fn pop_first(
        &mut self,
    ) -> Option<((NodeT, NodeT), Option<ConstructorEdgeMetadata>)> {
        self.tree.pop_first()
    }

    pub(crate) fn entry(
        &mut self,
        key: (NodeT, NodeT),
    ) -> Entry<(NodeT, NodeT), Option<ConstructorEdgeMetadata>> {
        self.tree.entry(key)
    }

    pub(crate) fn get_mut(
        &mut self,
        key: &(NodeT, NodeT),
    ) -> Option<&mut Option<ConstructorEdgeMetadata>> {
        self.tree.get_mut(key)
    }

    pub(crate) fn extend(
        &mut self,
        graph: &Graph,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
        weight: Option<WeightT>,
        include_all_edge_types: bool,
    ) {
        let metadata = if let Some(md) = self.tree.get(&(src, dst)) {
            let mut metadata = md.to_owned();
            if let Some(md) = &mut metadata {
                md.add(weight, edge_type);
            }
            metadata
        } else {
            let mut metadata = ConstructorEdgeMetadata::new(weight.is_some(), edge_type.is_some());
            if let Some(md) = &mut metadata {
                if include_all_edge_types {
                    md.set(
                        graph.get_link_weights(src, dst),
                        graph.get_link_edge_types(src, dst),
                    );
                } else {
                    md.add(weight, edge_type);
                }
            }
            metadata
        };
        self.tree.insert((src, dst), metadata.clone());
        // If the current edge is not a self loop and the graph
        // is not directed, we add the simmetrical graph
        if !graph.is_directed && src != dst {
            self.tree.insert((dst, src), metadata);
        }
    }
}

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
            return sets.len();
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
