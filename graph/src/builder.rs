use super::*;
use rayon::iter::Empty as ParEmpty;
use std::iter::Empty as SeqEmpty;
use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufWriter, Write};
use std::fs::File;

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

    /// Get a sorted iterator over the edges of the graph
    pub fn iter_edges(&self) -> impl Iterator<Item=EdgeQuadruple> + '_ {
        self.edges.iter().cloned()
    }

    /// Get a sorted iterator over the nodes of the graph
    pub fn iter_nodes(&self) -> impl Iterator<Item=(String, Option<Vec<String>>)> + '_ {
        self.nodes.iter().map(|(k, v)| (k.clone(), v.clone()))
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
            None, // number_of_node_types
            Some(false), // numeric_node_type_ids
            None, // minimum_node_type_id
            self.has_node_types, // has_node_types
            Some(false), // node_types_list_is_correct
            nodes_iterator, // nodes_iterator
            None, // number_of_nodes
            false, // node_list_is_correct
            false, // numeric_node_ids
            false, // numeric_node_list_node_type_ids
            None, // minimum_node_id
            None::<ItersWrapper<_, SeqEmpty<_>, ParEmpty<_>>>, // edge_types_iterator
            None, // number_of_edge_types
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
            None, // number_of_edges
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

impl core::iter::Extend<EdgeQuadruple> for GraphBuilder {
    fn extend<T: IntoIterator<Item=EdgeQuadruple>>(&mut self, iter: T) {
        for edge in iter {
            let _ = self.add_edge(edge.0, edge.1, edge.2, Some(edge.3));
        }
    }
}

impl core::iter::Extend<(String, Option<Vec<String>>)> for GraphBuilder {
    fn extend<T: IntoIterator<Item=(String, Option<Vec<String>>)>>(&mut self, iter: T) {
        for edge in iter {
            let _ = self.add_node(edge.0, edge.1);
        }
    }
}

impl core::ops::AddAssign<Self> for GraphBuilder {
    fn add_assign(&mut self, other: Self) {
        self.extend(other.iter_edges());
        self.extend(other.iter_nodes());

        self.has_node_types |= other.has_node_types;
        self.has_edge_types |= other.has_edge_types;
        self.has_edge_weights |= other.has_edge_weights;
        self.directed |= other.directed;
        self.name = format!("{} | {}", self.name, other.name);
    }
}

impl core::iter::Sum for GraphBuilder {
    fn sum<I: Iterator<Item=Self>>(mut iter: I) -> Self {
        let first = iter.next();
        if first.is_none() {
            return Self::new(None, None);
        }
        let mut res = first.unwrap();
        for i in iter {
            res += i;
        }
        res
    }
}

#[derive(Debug)]
pub struct GraphCSVBuilder {
    pub(crate) edges_path: String,
    pub(crate) nodes_path: String,
    pub(crate) edges: BufWriter<File>,
    pub(crate) nodes: BufWriter<File>,
    
    pub(crate) has_node_types: bool,
    pub(crate) has_edge_types: bool,
    pub(crate) has_edge_weights: bool,
}

impl core::fmt::Display for GraphCSVBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("GraphCSVBuilder")
            .field("edges_path", &self.edges_path)
            .field("nodes_path", &self.nodes_path)
            .finish()
    }
}

impl GraphCSVBuilder {
    /// Write a csv file loadable from GRAPE like a NetworkX graph.
    /// 
    /// This is optional, but might help some users.
    /// 
    /// # Arguments
    /// * `path`: String - The name of the graph
    pub fn new(path: &str) -> Result<Self> {
        let edges_path = format!("{}_edges.csv", path);
        let nodes_path = format!("{}_nodes.csv", path);
        let mut edges = BufWriter::new(File::create(&edges_path).map_err(|x| x.to_string())?);
        let mut nodes = BufWriter::new(File::create(&nodes_path).map_err(|x| x.to_string())?);

        edges.write("source,destination,edge_type,weight\n".as_bytes()).map_err(|x| x.to_string())?;
        nodes.write("node_name,node_type\n".as_bytes()).map_err(|x| x.to_string())?;

        Ok(Self {
            edges_path,
            nodes_path,
            nodes,
            edges,

            has_edge_weights: false,
            has_edge_types: false,
            has_node_types: false,
        })
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
        self.edges.write(format!("\"{src}\",\"{dst}\",\"{et}\",{w}\n", 
            src=src,
            dst=dst,
            et=edge_type.unwrap_or("".into()),
            w=weight.map(|x| x.to_string()).unwrap_or("".into()),
        ).as_bytes()).map_err(|x| x.to_string())?;
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
        self.nodes.write(format!("\"{name}\",\"{nt}\"\n", 
            name=name,
            nt=node_type.map(|x| x.join("|")).unwrap_or("".into()),
        ).as_bytes()).map_err(|x| x.to_string())?;
        Ok(())
    }

    /// Flush the changes to the files and print the example code on how the 
    /// graph can be loaded using `Graph.from_csv`
    pub fn finish(&mut self) -> Result<String> {
        self.edges.flush().map_err(|x| x.to_string())?;
        self.nodes.flush().map_err(|x| x.to_string())?;

// edges.write("source,destination,edge_type,weight\n".as_bytes());
// nodes.write("node_name,node_type\n".as_bytes());

        let nt_str = if self.has_node_types {
r#"
    # Node type related settings

    ## The column with the type of each node.
    node_list_node_types_column="node_type",
    ## How multiple node_types are separated
    node_types_separator="|",
    ## If a node misses a node_type it's ok
    skip_node_types_if_unavailable=True,
    ## The node_type to assign to nodes with missing type
    # default_node_type=1.0,
"#
        } else {
            ""
        };

        let w_str = if self.has_edge_weights {
            r#"
    # Edge weight related settings

    ## The weights are in the third column
    weights_column="weight",
    ##  Whether to skip the weights without raising an error if these are unavailable.
    skip_weights_if_unavailable=True,
    ## The weight to assign to edges with missing weights
    # default_weight=1.0,
            "#
        } else {
            ""
        };

        let et_str = if self.has_node_types {
r#"
    
    # Edge type related settings

    ## The column with the type of each node.
    edge_list_edge_types_column="edge_type",
    ## If a node misses a node_type it's ok
    skip_edge_types_if_unavailable=True,
    # Edge type to assign to edges with missing edge type
    # default_edge_type="Default",
"#
        } else {
            ""
        };

        Ok(format!(
r#"
# To load the generated graph you can run the following code:
from grape import Graph

graph = Graph.from_csv(
    # Change these as needed
    directed=False,
    name="MyGraph",

    # Edges related settings

    ## The path to the edges list tsv
    edge_path={edge_path:?},
    ## Set the tab as the separator between values
    edge_list_separator=",",
    ## The first rows should be used as the columns names
    edge_list_header=True,
    ## The source nodes are in the first nodes
    sources_column="source",
    ## The destination nodes are in the second column
    destinations_column="destination",

    {et_str}

    {w_str}

    # Nodes related settings

    # Nodes related parameters
    ## The path to the nodes list tsv
    node_path={node_path:?},
    ## Set the tab as the separator between values
    node_list_separator=",",
    ## The first rows should be used as the columns names
    node_list_header=True,
    ## The column with the node names is the one with name "node_name".
    nodes_column="node_name",

    {nt_str}
)
"#,
    edge_path=self.edges_path,
    node_path=self.nodes_path,
    nt_str=nt_str,
    w_str=w_str,
    et_str=et_str,
        ))
    }
}