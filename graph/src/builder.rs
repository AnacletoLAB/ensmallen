use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use rayon::iter::Empty as ParEmpty;
use std::iter::Empty as SeqEmpty;
use std::collections::BTreeSet;

/// Quadrule of string edge data
struct EdgeQuadruple(pub String, pub String, pub Option<String>, pub WeightT);

impl PartialEq for EdgeQuadruple {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2) && (self.3.total_cmp(&other.3).is_eq())
    }
}

impl Eq for EdgeQuadruple {}

impl PartialOrd for EdgeQuadruple {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(
            self.0.partial_cmp(&other.0)?
                .then(self.1.partial_cmp(&other.1)?)
                .then(self.2.partial_cmp(&other.2)?)
                .then(self.3.total_cmp(&other.3))
        )
    }
}

impl Ord for EdgeQuadruple {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
            .then(self.1.cmp(&other.1))
            .then(self.2.cmp(&other.2))
            .then(self.3.total_cmp(&other.3))
    }
}

pub struct GraphBuilder {
    edges: BTreeSet<EdgeQuadruple>,
    nodes: BTreeSet<(String, Option<Vec<String>>)>,

    has_node_types: bool,
    has_edge_types: bool,
    has_edge_weights: bool,
    directed: bool,
    name: String,

    default_weight: f32,
}

impl GraphBuilder {
    pub fn new(name: Option<String>, directed: bool) -> Self {
        Self {
            directed,
            name: name.unwrap_or("Graph".to_string()),

            has_edge_weights: false,
            has_edge_types: false,
            has_node_types: false,

            nodes: BTreeSet::new(),
            edges: BTreeSet::new(),

            default_weight: 1.0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_directed(&mut self, is_directed: bool) {
        self.directed = is_directed;
    }

    pub fn set_default_weight(&mut self, default_weight: f32) {
        self.default_weight = default_weight;
    }

    pub fn add_edge(
        &mut self, 
        src: String,
        dst: String,
        edge_type: Option<String>,
        weight: Option<WeightT>,
    ) -> Result<()> {
        if let Some(w) = weight {
            if !w.is_finite() {
                return Err(format!("The weight {} is not a finite numnber!", w));
            }
            self.has_edge_weights = true;
        }
        if edge_type.is_some() {
            self.has_edge_types = true;
        }
        self.edges.insert(EdgeQuadruple(src, dst, edge_type, weight.unwrap_or(self.default_weight)));
        Ok(())
    }

    pub fn remove_edge(
        &mut self, 
        src: String,
        dst: String,
        edge_type: Option<String>,
        weight: Option<WeightT>,
    ) -> Result<()> {
        if let Some(w) = weight {
            if !w.is_finite() {
                return Err(format!("The weight {} is not a finite numnber!", w));
            }
        }
        self.edges.remove(&EdgeQuadruple(src, dst, edge_type, weight.unwrap_or(self.default_weight)));
        Ok(())
    }

    pub fn add_node(&mut self, 
        name: String, node_type: Option<Vec<String>>) -> Result<()> {
        if node_type.is_some() {
            self.has_node_types = true;
        }
        self.nodes.insert((name, node_type));
        Ok(())
    }

    pub fn remove_node(&mut self, 
        name: String, node_type: Option<Vec<String>>) -> Result<()> {
        self.nodes.remove(&(name, node_type));
        Ok(())
    }

    pub fn build(self) -> Result<Graph> {

        let edges_number = self.edges.len();

        let nodes_iterator = if self.nodes.is_empty() {
            None
        } else {
            Some(ItersWrapper::Sequential::<_, _, ParEmpty<_>>(self.nodes.into_iter().enumerate().map(|x| Result::Ok(x))))
        };

        let edges_iterator = ItersWrapper::Sequential::<_, _, ParEmpty<_>>(
            self.edges.into_iter().enumerate().map(|(idx, x)| 
                Result::Ok((idx, (x.0, x.1, x.2, x.3)))
            )
        );

        build_graph_from_strings(
            None::<ItersWrapper<_, SeqEmpty<_>, ParEmpty<_>>>, // node_types_iterator
            None, // node_types_number
            Some(false), // numeric_node_type_ids
            None, // minimum_node_type_id
            self.has_node_types, // has_node_types
            Some(false), // node_types_list_is_correct
            nodes_iterator, // nodes_iterator
            None, // nodes_number
            false, // node_list_is_correct
            false, // numeric_node_ids
            false, // numeric_node_list_node_type_ids
            None, // minimum_node_ids
            None::<ItersWrapper<_, SeqEmpty<_>, ParEmpty<_>>>, // edge_types_iterator
            None, // edge_types_number
            Some(false), // numeric_edge_type_ids
            None, // minimum_edge_type_id
            self.has_edge_types, // has_edge_types
            Some(false), // edge_types_list_is_correct
            Some(edges_iterator),
            self.has_edge_weights, // has_edge_weights
            self.directed, // directed
            Some(false), // correct
            Some(false), // complete
            Some(false), // duplicates
            Some(true), // sorted
            Some(edges_number as _), // edges_number
            Some(false), // numeric_edge_list_node_ids
            Some(false), // numeric_edge_list_edge_type_ids
            Some(true), // skip_node_types_if_unavailable
            Some(true), // skip_edge_types_if_unavailable
            true, // may_have_singletons
            true, // may_have_singleton_with_selfloops
            self.name, // name
        )
    }
}