use super::*;
use rayon::iter::Empty as ParEmpty;
use std::iter::Empty as SeqEmpty;
use std::collections::{BTreeSet, BTreeMap};

#[derive(Clone, Debug)]
pub struct GraphBuilder {
    pub(crate) edges: BTreeSet<EdgeQuadruple>,
    pub(crate) nodes: BTreeMap<String, Option<Vec<String>>>,

    pub(crate) has_node_types: bool,
    pub(crate) has_edge_types: bool,
    pub(crate) has_edge_weights: bool,
    pub(crate) directed: bool,
    pub(crate) name: String,

    pub(crate) default_weight: f32,
}

impl core::fmt::Display for GraphBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("GraphBuilder")
            .field("number_of_edges", &self.edges.len())
            .field("number_of_nodes", &self.nodes.len())
            .field("directed", &self.directed)
            .field("name", &self.name)
            .field("default_weight", &self.default_weight)
            .finish()
    }
}

impl GraphBuilder {
    /// Create a graph NetworkX style.
    /// 
    /// This is **NOT** the most efficient way because it will have to duplicate
    /// the memory. The most efficient way to build a graph is to create an
    /// appropriate CSV that can be loaded directly. This building will use MORE
    /// memory than the loaded graph.
    /// 
    /// # Arguments
    /// * `name`: String - The name of the graph
    /// * `directed`: bool - the generated graph will be directed if this is true, by default it's `false`
    pub fn new(name: Option<String>, directed: Option<bool>) -> Self {
        Self {
            directed: directed.unwrap_or(false),
            name: name.unwrap_or("Graph".to_string()),

            has_edge_weights: false,
            has_edge_types: false,
            has_node_types: false,

            nodes: BTreeMap::new(),
            edges: BTreeSet::new(),

            default_weight: 1.0,
        }
    }

    /// Set the name of the graph that will be created
    /// 
    /// # Arguments
    /// * `name`: &str - The name of the graph
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Set if the graph will be directed or undirected
    /// 
    /// # Arguments
    /// * `is_directed`: bool - the generated graph will be directed if this is true
    pub fn set_directed(&mut self, is_directed: bool) {
        self.directed = is_directed;
    }

    /// Set a default missing weight to be used if only some edges have weights
    /// 
    /// # Arguments
    /// * `default_weight`: WeightT - set the weight to assign by default at edges 
    pub fn set_default_weight(&mut self, default_weight: WeightT) {
        self.default_weight = default_weight;
    }

    /// Add an edge to the graph
    /// 
    /// # Arguments
    /// * `src`: String - The name of the source node
    /// * `dst`: String - The name of the destination node
    /// * `edge_type`: Option<String> - The name of the edge_type, if present
    /// * `weight`: Option<WeightT> - The weight of the edge, if present
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

    /// Remove an edge to the graph, if the edge is not present this will do nothing.
    /// 
    /// # Arguments
    /// * `src`: String - The name of the source node
    /// * `dst`: String - The name of the destination node
    /// * `edge_type`: Option<String> - The name of the edge_type, if present
    /// * `weight`: Option<WeightT> - The weight of the edge, if present
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

    /// Add a node to the graph, if the node is already present in the graph it will be overwritten
    /// 
    /// # Arguments
    /// * `name`: String - The name of the node
    /// * `node_type`: Option<Vec<String>> - List of node type names, if present
    pub fn add_node(&mut self, 
        name: String, node_type: Option<Vec<String>>) -> Result<()> {
        if node_type.is_some() {
            self.has_node_types = true;
        }
        self.nodes.insert(name, node_type);
        Ok(())
    }

    /// Remove a node from the graph, if the node does not exist, this method does nothing
    /// 
    /// # Arguments
    /// * `name`: String - The name of the node
    pub fn remove_node(&mut self, name: String) -> Result<()> {
        self.nodes.remove(&name);
        Ok(())
    }

    /// Consume the edges and nodes to create a new graph.
    pub fn build(&mut self) -> Result<Graph> {
        let nodes = core::mem::replace(&mut self.nodes, BTreeMap::new());
        let edges = core::mem::replace(&mut self.edges, BTreeSet::new());

        let nodes_iterator = if nodes.is_empty() {
            None
        } else {
            Some(ItersWrapper::Sequential::<_, _, ParEmpty<_>>(nodes.into_iter().enumerate().map(|x| Result::Ok(x))))
        };

        let edges_iterator = ItersWrapper::Sequential::<_, _, ParEmpty<_>>(
            edges.into_iter().enumerate().map(|(idx, x)| 
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
            Some(false), // sorted
            None, // edges_number
            Some(false), // numeric_edge_list_node_ids
            Some(false), // numeric_edge_list_edge_type_ids
            Some(true), // skip_node_types_if_unavailable
            Some(true), // skip_edge_types_if_unavailable
            true, // may_have_singletons
            true, // may_have_singleton_with_selfloops
            self.name.clone(), // name
        )
    }
}