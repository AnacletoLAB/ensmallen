use super::*;
use std::collections::{BTreeMap, HashSet};

// Types used to represent edges, nodes and their types.
/// Type used to index the Nodes.
pub type NodeT = usize;
/// Type used to index the Node Types.
pub type NodeTypeT = u16;
/// Type used to index the Edges.
pub type EdgeT = usize;
/// Type used to index the Edge Types.
pub type EdgeTypeT = u16;
/// Type used for the weights of the edges.
pub type WeightT = f64;
/// Type used for the parameters of the walk such as the return weight (p),
/// and the explore weight (q).
pub type ParamsT = f64;
/// Type used to save contexts used for Skipgram and CBOW.
pub type Contexts = Vec<Vec<NodeT>>;
/// Type used to save a group of words indices.
pub type Words = Vec<NodeT>;
/// Type used to save the frequencies of words
pub type Frequencies = Vec<f64>;
/// Triple of edge data
pub type Triple = (NodeT, NodeT, Option<EdgeTypeT>);
/// Quadruple of edge data
pub type Quadruple = (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>);

/// Custom BTreeMap with some helper methods
#[derive(Debug)]
pub(crate) struct GraphDictionary {
    tree: BTreeMap<(NodeT, NodeT), Option<ConstructorEdgeMetadata>>,
    edges: usize,
}

impl GraphDictionary {
    pub(crate) fn new() -> GraphDictionary {
        GraphDictionary {
            tree: BTreeMap::new(),
            edges: 0,
        }
    }

    /// Return number of elements currently present in tree.
    pub(crate) fn len(&self) -> usize {
        self.edges
    }

    /// Return ConstructorEdgeMetadata if present.
    ///
    /// # Arguments
    ///
    /// * key: &(NodeT, NodeT) - The tuple of nodes forming the dictionary key.
    pub(crate) fn get(&self, key: &(NodeT, NodeT)) -> Option<&Option<ConstructorEdgeMetadata>> {
        self.tree.get(key)
    }

    /// Return boolean representing if given key is present within tree.
    ///
    /// # Arguments
    ///
    /// * key: &(NodeT, NodeT) - The tuple of nodes to check existance for.
    pub(crate) fn contains_key(&self, key: &(NodeT, NodeT)) -> bool {
        self.tree.contains_key(key)
    }

    /// Return boolea representing if tree is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Returnì last value of the tree.
    pub(crate) fn last_key_value(
        &self,
    ) -> Option<(&(NodeT, NodeT), &Option<ConstructorEdgeMetadata>)> {
        self.tree.last_key_value()
    }

    /// Return first value in the tree.
    pub(crate) fn pop_first(
        &mut self,
    ) -> Option<((NodeT, NodeT), Option<ConstructorEdgeMetadata>)> {
        self.tree.pop_first()
    }

    /// Return mutable ConstructorEdgeMetadata if present.
    ///
    /// # Arguments
    ///
    /// * key: &(NodeT, NodeT) - The tuple of nodes forming the dictionary key.
    pub(crate) fn get_mut(
        &mut self,
        key: &(NodeT, NodeT),
    ) -> Option<&mut Option<ConstructorEdgeMetadata>> {
        self.tree.get_mut(key)
    }

    /// Extends tree with given data.
    ///
    /// # Arguments
    ///
    /// * graph: &Graph - Reference of graph from where to extract informations.
    /// * src: NodeT - The source node.
    /// * dst: NodeT - The destination node.
    /// * edge_type: Option<EdgeTypeT> - The optional edge type to insert.
    /// * weight: Option<WeightT> - The optional weight to insert.
    /// * include_all_edge_types: bool - Wether to insert all the original edge types. This is only relevant in multi-graphs.
    pub(crate) fn extend(
        &mut self,
        graph: &Graph,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
        weight: Option<WeightT>,
        include_all_edge_types: bool,
    ) {
        let (metadata, new_edges) = if let Some(md) = self.tree.get(&(src, dst)) {
            let mut metadata = md.to_owned();
            if let Some(md) = &mut metadata {
                md.add(weight, edge_type);
            }
            (metadata, 1)
        } else {
            let mut metadata = ConstructorEdgeMetadata::new(weight.is_some(), edge_type.is_some());
            let new_edges = if let Some(md) = &mut metadata {
                if include_all_edge_types {
                    md.set(
                        graph.get_unchecked_link_weights(src, dst),
                        graph.get_unchecked_link_edge_types(src, dst),
                    );
                } else {
                    md.add(weight, edge_type);
                }
                md.len()
            } else {
                1
            };
            (metadata, new_edges)
        };
        self.tree.insert((src, dst), metadata.clone());
        self.edges += new_edges;
        // If the current edge is not a self loop and the graph
        // is not directed, we add the simmetrical graph
        if !graph.directed && src != dst {
            self.tree.insert((dst, src), metadata);
            self.edges += new_edges;
        }
    }

    /// Extends tree with given data.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node.
    /// * dst: NodeT - The destination node.
    /// * edge_type: Option<EdgeTypeT> - The optional edge type to insert.
    /// * weight: Option<WeightT> - The optional weight to insert.
    pub(crate) fn simple_extend(
        &mut self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
        weight: Option<WeightT>,
        directed: bool,
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
                md.add(weight, edge_type);
            }
            metadata
        };
        self.tree.insert((src, dst), metadata.clone());
        self.edges += 1;
        // If the current edge is not a self loop and the graph
        // is not directed, we add the simmetrical graph
        if !directed && src != dst {
            self.tree.insert((dst, src), metadata);
            self.edges += 1;
        }
    }
}

/// Metadata of the edges of the graphs used for every graph.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeMetadata {
    /// the id of the FIRST edge with the src and dst matching
    pub edge_id: EdgeT,
    /// An optional set of the edge types of the edges between src and dst
    pub edge_types: Option<HashSet<EdgeTypeT>>,
}

/// Metadata of the edges used to describe both homogeneous and heterogeneous graphs and multi-graphs.
///
/// It used during the construction process of the graphs, while another smaller one is used for the actual structure.
#[derive(Clone, Debug)]
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

/// Trait used for the Vocabulary class.
/// It represent an unsigned integer that can be converted to and from usize.
/// This allows us to save memory using indicies of smaller size than u64
/// and it has no effects on performance because it's optimized away during
/// compilaton.
pub trait ToFromUsize {
    /// create the type from a usize
    fn from_usize(v: usize) -> Self;
    /// create an usize frm the type
    fn to_usize(v: Self) -> usize;
}

/// Automatically implement the methods needed to convert from and to usize
/// for the given numerical type.
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
