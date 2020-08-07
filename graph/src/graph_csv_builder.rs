use super::*;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::prelude::*, io::BufReader};

use super::graph_csv_helpers::*;

struct NodesArguments {
    node_path: String,
    nodes_column: String,
    node_types_column: String,
    default_node_type: Option<String>,
    node_sep: String,
    ignore_duplicated_nodes: bool,
}

pub struct FromCsvBuilder {
    edge_path: String,

    sources_column: String,
    destinations_column: String,
    directed: bool,

    edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    weights_column: Option<String>,
    default_weight: Option<WeightT>,

    edge_sep: String,
    ignore_duplicated_edges: bool,
    force_conversion_to_undirected: bool,

    nodes_args: Option<NodesArguments>,
}

impl FromCsvBuilder {

    pub fn new(
        edge_path: &str,
        sources_column: &str,
        destinations_column: &str,
        directed: bool,
        edge_sep: Option<&str>,
    ) -> Result<FromCsvBuilder, String> {
        let _edge_sep = edge_sep.unwrap_or_else(|| "\t").to_string();
        if _edge_sep.is_empty() {
            return Err(String::from("The edge separator is empty"));
        }

        Ok(FromCsvBuilder {
            directed,
            nodes_args: None,
            edge_path: edge_path.to_string(),

            sources_column: sources_column.to_string(),
            destinations_column: destinations_column.to_string(),
            edge_sep: _edge_sep,

            edge_types_column: None,
            default_edge_type: None,
            weights_column: None,
            default_weight: None,

            ignore_duplicated_edges: false,
            force_conversion_to_undirected: false,
        })
    }

    pub fn set_edge_types(
        mut self,
        edge_types_column: &str,
        default_edge_type: Option<&str>,
    ) -> FromCsvBuilder {
        self.edge_types_column = Some(edge_types_column.to_string());
        self.default_edge_type = match default_edge_type {
            Some(g) => Some(g.to_string()),
            None => None,
        };
        self
    }

    pub fn set_weights(
        mut self,
        weights_column: &str,
        default_weight: Option<WeightT>,
    ) -> FromCsvBuilder {
        self.weights_column = Some(weights_column.to_string());
        self.default_weight = default_weight;
        self
    }

    pub fn load_nodes_csv(
        mut self,
        node_path: &str,
        nodes_column: &str,
        node_types_column: &str,
        default_node_type: Option<&str>,
        node_sep: Option<&str>,
        ignore_duplicated_nodes: Option<bool>,
    ) -> Result<FromCsvBuilder, String> {
        if self.nodes_args.is_some() {
            return Err("Cannot load two node files.".to_string());
        }
        let _ignore_duplicated_nodes = ignore_duplicated_nodes.unwrap_or(false);
        self.nodes_args = Some(NodesArguments {
            node_path: node_path.to_string(),
            ignore_duplicated_nodes: _ignore_duplicated_nodes,
            nodes_column: if nodes_column.is_empty() {
                Err(String::from("The nodes column is empty"))
            } else {
                Ok(nodes_column.to_string())
            }?,
            node_types_column: if node_types_column.is_empty() {
                Err(String::from("The nodes type column is empty"))
            } else {
                Ok(node_types_column.to_string())
            }?,
            node_sep: match node_sep {
                Some(g) => {
                    if g.is_empty() {
                        Err(String::from("The nodes separator is empty"))
                    } else {
                        Ok(g)
                    }
                }
                None => Ok("\t"),
            }?
            .to_string(),
            default_node_type: match default_node_type {
                Some(g) => {
                    if g.is_empty() {
                        Err(String::from("The nodes type column is empty"))
                    } else {
                        Ok(Some(g.to_string()))
                    }
                }
                None => Ok(None),
            }?,
        });
        Ok(self)
    }

    pub fn set_ignore_duplicated_edges(mut self) -> FromCsvBuilder {
        self.ignore_duplicated_edges = true;
        self
    }

    pub fn set_force_conversion_to_undirected(mut self) -> FromCsvBuilder {
        self.force_conversion_to_undirected = true;
        self
    }

    // reference here
    // https://stackoverflow.com/questions/37028476/how-to-combine-stdstrlines-and-stdiolines
    fn parse_nodes(
        &mut self,
    ) -> Result<
        (
            HashMap<String, NodeT>,
            Vec<String>,
            Option<Vec<NodeTypeT>>,
            Option<HashMap<String, NodeTypeT>>,
            Option<Vec<String>>,
        ),
        String,
    > {
        let args = self.nodes_args.as_ref().ok_or("Cannot parse nodes iterator without it's arugments, This exception should not be possible.")?;
        // open the file
        let file = match File::open(&args.node_path) {
            Ok(g) => Ok(g),
            Err(_) => Err(format!("Cannot open the nodes file at {}", &args.node_path)),
        }?;
        let node_buf_reader = BufReader::new(file);
        let mut lines = node_buf_reader.lines();
        // read the first line
        let header_line: String = lines
            .next()
            .ok_or("The given nodes file or string has no lines!")?
            .unwrap();
        let columns: Vec<&str> = header_line.split(&args.node_sep).collect();
        let number_of_separators = columns.len();

        if !columns.contains(&&args.node_types_column[..]) {
            return Err(format!(
                concat!(
                    "The column for the node types {} was not found.\n",
                    "From the header the parser found the columns {:?}\n"
                ),
                args.node_types_column, columns
            ));
        }

        if !columns.contains(&&args.nodes_column[..]) {
            return Err(format!(
                concat!(
                    "The column for the node names {} was not found.\n",
                    "From the header the parser found the columns {:?}\n"
                ),
                args.nodes_column, columns
            ));
        }

        // initialize the variables
        let mut nodes: Vec<NodeT> = Vec::new();
        let mut nodes_mapping: HashMap<String, NodeT> = HashMap::new();
        let mut nodes_reverse_mapping: Vec<String> = Vec::new();
        // types related
        let mut node_types: Vec<NodeTypeT> = Vec::new();
        let mut node_types_mapping: HashMap<String, NodeTypeT> = HashMap::new();
        let mut node_types_reverse_mapping: Vec<String> = Vec::new();

        // parse each line and update the results
        for (line_index, _line) in lines.enumerate() {
            let line = _line.unwrap();
            // get a dictionary of the current line
            let parsed: HashMap<String, String> = columns
                .iter()
                .map(|column| column.to_string())
                .zip(line.split(&args.node_sep).map(|v| v.to_string()))
                .collect();

            // check the correctness of the line
            check_line_consistency(&parsed, number_of_separators, &line, line_index)?;

            // compote the id of the node
            let node = parsed.get(&args.nodes_column).unwrap();
            let node_id = nodes_mapping.len();

            // check for duplicates
            if nodes_mapping.contains_key(node) {
                if args.ignore_duplicated_nodes {
                    continue;
                }
                return Err(format!(
                    concat!(
                        "\nFound duplicated line in nodes file!\n",
                        "Specifically, the duplicated line is the number {j}.\n",
                        "The node is {node}.\n",
                        "The complete line in question is:\n{line}\n"
                    ),
                    j = line_index,
                    node = node,
                    line = line
                ));
            }

            // add the node to the mappings
            nodes.push(node_id);
            nodes_mapping.insert(node.to_string(), node_id);
            nodes_reverse_mapping.push(node.to_string());

            // get and set default for the node type
            let mut node_type = parsed.get(&args.node_types_column).unwrap();
            // check if the node_type is rasonable
            if node_type.is_empty() {
                if let Some(dnt) = &args.default_node_type {
                    node_type = &dnt;
                } else {
                    return Err(format!(
                        concat!(
                            "Found empty node type but no default node ",
                            "type to use was provided.",
                            "Specifically, the line is the number {j}.\n",
                            "The complete line in question is:\n{line}\n"
                        ),
                        j = line_index,
                        line = line
                    ));
                }
            }

            // update node_types_mapping with the new node type
            if !node_types_mapping.contains_key(node_type) {
                node_types_mapping.insert(
                    String::from(node_type),
                    node_types_reverse_mapping.len() as NodeTypeT,
                );
                node_types_reverse_mapping.push(String::from(node_type));
            }

            // update the array of types which map each node to it's type id
            node_types.push(*node_types_mapping.get(node_type).unwrap());
        }

        Ok((
            nodes_mapping,
            nodes_reverse_mapping,
            Some(node_types),
            Some(node_types_mapping),
            Some(node_types_reverse_mapping),
        ))
    }

    // reference here
    // https://stackoverflow.com/questions/37028476/how-to-combine-stdstrlines-and-stdiolines
    pub fn build(&mut self) -> Result<Graph, String> {
        // do the nodes parsing / initializzation
        let (
            mut nodes_mapping,
            mut nodes_reverse_mapping,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
        ) = if self.nodes_args.is_some() {
            self.parse_nodes()?
        } else {
            (HashMap::new(), Vec::new(), None, None, None)
        };
        // read the file
        let file = match File::open(&self.edge_path) {
            Ok(g) => Ok(g),
            Err(_) => Err(format!("Cannot open the edge file at {}", &self.edge_path)),
        }?;

        let edge_buf_reader = BufReader::new(file);
        let mut lines = edge_buf_reader.lines();
        // vector where we save the edges
        let mut sources: Vec<NodeT> = Vec::new();
        let mut destinations: Vec<NodeT> = Vec::new();
        // edge types related structs
        let mut edge_types: Vec<EdgeTypeT> = Vec::new();
        let mut edge_types_mapping: HashMap<String, EdgeTypeT> = HashMap::new();
        let mut edge_types_reverse_mapping: Vec<String> = Vec::new();
        // weights vector
        let mut weights: Vec<WeightT> = Vec::new();
        // helper function, this might be optimizable away
        let mut unique_edges_set: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();

        // read the first line
        let header_line = lines
            .next()
            .ok_or("The given edges file or string has no lines!")?
            .unwrap();
        let columns: Vec<&str> = header_line.split(&self.edge_sep).collect();
        let number_of_separators = columns.len();
        // cast headers columns to number to parse them in the same way of positional one

        if !columns.contains(&&self.sources_column[..]) {
            return Err(format!(
                concat!(
                    "The column for the sources names {} was not found.\n",
                    "From the header the parser found the columns {:?}\n"
                ),
                self.sources_column, columns
            ));
        }

        if !columns.contains(&&self.destinations_column[..]) {
            return Err(format!(
                concat!(
                    "The column for the destinations names {} was not found.\n",
                    "From the header the parser found the columns {:?}\n"
                ),
                self.destinations_column, columns
            ));
        }

        if let Some(etc) = &self.edge_types_column {
            if !columns.contains(&&etc[..]) {
                return Err(format!(
                    concat!(
                        "The column for the edge types {} was not found.\n",
                        "From the header the parser found the columns {:?}\n"
                    ),
                    etc, columns
                ));
            }
        }

        if let Some(wc) = &self.weights_column {
            if !columns.contains(&&wc[..]) {
                return Err(format!(
                    concat!(
                        "The column for the weights {} was not found.\n",
                        "From the header the parser found the columns {:?}\n"
                    ),
                    wc, columns
                ));
            }
        }

        // parse each line and update the results
        for (line_index, _line) in lines.enumerate() {
            let line = _line.unwrap();
            // get a dictionary of the current line
            let parsed: HashMap<String, String> = columns
                .iter()
                .map(|column| column.to_string())
                .zip(line.split(&self.edge_sep).map(|v| v.to_string()))
                .collect();

            // check the correctness of the line
            check_line_consistency(&parsed, number_of_separators, &line, line_index)?;

            // get the src and dst
            let src_name = parsed[&self.sources_column].to_string();
            let dst_name = parsed[&self.destinations_column].to_string();

            // get the id for the nodes
            let (src, dst) = get_nodes_ids_and_map(
                &node_types,
                &mut nodes_mapping,
                &mut nodes_reverse_mapping,
                &src_name,
                &dst_name,
                &line,
                line_index,
            )?;

            // get the weight if present
            let weight: Option<WeightT> = parse_weight(
                &parsed,
                &self.weights_column,
                &self.default_weight,
                &line,
                line_index,
            )?;

            // get the name of the edge type
            let edge_type_name = parse_edge_type_name(
                &parsed,
                &self.edge_types_column,
                &self.default_edge_type,
                &line,
                line_index,
            )?;

            // convert the name of edge type to it's ID
            let edge_type = match &edge_type_name {
                None => None,
                Some(et_name) => Some(match edge_types_mapping.get(&et_name.clone()) {
                    Some(et) => *et,
                    None => {
                        let new_id = edge_types_reverse_mapping.len() as NodeTypeT;
                        edge_types_mapping.insert(et_name.clone(), new_id);
                        edge_types_reverse_mapping.push(et_name.clone());
                        new_id
                    }
                }),
            };

            let triple = (src, dst, edge_type);
            if !unique_edges_set.contains(&triple) {
                unique_edges_set.insert(triple);
                sources.push(src);
                destinations.push(dst);
                if let Some(et) = edge_type {
                    edge_types.push(et);
                }
                if let Some(w) = weight {
                    weights.push(w);
                }
            } else if !self.ignore_duplicated_edges {
                return Err(format!(
                    concat!(
                        "\nFound duplicated line in edges file!\n",
                        "Specifically, the duplicated line is the number {line_index}.\n",
                        "The source node is {source} and destination node is {destination}.\n",
                        "{edge_type_string}\n",
                        "The complete line in question:\n{line}\n"
                    ),
                    line_index = line_index,
                    source = &src_name,
                    destination = &dst_name,
                    edge_type_string = if let Some(etn) = &edge_type_name {
                        format!("The edge type of the row is {}.", etn)
                    } else {
                        String::from("No edge type was detected.")
                    },
                    line = line
                ));
            }
        }

        let mut graph = Graph::builder(sources, destinations, self.directed).add_nodes(
            nodes_mapping,
            nodes_reverse_mapping,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
        );

        if self.weights_column.is_some() {
            graph = graph.add_weights(weights);
        }

        if self.edge_types_column.is_some() {
            graph =
                graph.add_edge_types(edge_types, edge_types_mapping, edge_types_reverse_mapping);
        }

        graph.build(Some(self.force_conversion_to_undirected))
    }
}
