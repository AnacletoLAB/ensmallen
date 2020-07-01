use super::*;
use crate::csv_utils::{check_consistent_lines, get_headers, has_columns};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::prelude::*, io::BufReader};

/// Construction of the graph from csv / tsv
impl Graph {
    fn read_edges_csv(
        path: &str,
        sep: &str,
        sources_column: &str,
        destinations_column: &str,
        edge_types_column: &Option<&str>,
        default_edge_type: &Option<&str>,
        weights_column: &Option<&str>,
        default_weight: &Option<WeightT>,
        ignore_duplicated_edges: bool
    ) -> Result<
        (
            Vec<NodeT>,
            Vec<NodeT>,
            HashMap<String, NodeT>,
            Vec<String>,
            Option<Vec<EdgeTypeT>>,
            Option<HashMap<String, EdgeTypeT>>,
            Option<Vec<String>>,
            Option<Vec<WeightT>>,
        ),
        String,
    > {
        // TODO figure out how to use references and lifetimes so that
        // we don't duplicate the strings in the mappings
        let mut sources: Vec<NodeT> = Vec::new();
        let mut destinations: Vec<NodeT> = Vec::new();

        let mut nodes_mapping: HashMap<String, NodeT> = HashMap::new();
        let mut nodes_reverse_mapping: Vec<String> = Vec::new();

        let mut edge_types: Vec<NodeTypeT> = Vec::new();
        let mut edge_types_mapping: HashMap<String, NodeTypeT> = HashMap::new();
        let mut edge_types_reverse_mapping: Vec<String> = Vec::new();

        let mut weights: Vec<WeightT> = Vec::new();

        let mut unique_edges_set: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();

        let headers = get_headers(path, sep);

        // open the file
        let error_message = &format!("Cannot open the nodes file at {}", path);
        let file = File::open(path).expect(error_message);
        let mut buf_reader = BufReader::new(file);
        // Skip header
        let mut line = String::new();
        let header_line = buf_reader.read_line(&mut line);
        if header_line.is_err() {
            return Err(String::from("Cannot read the header of the node files"));
        }
        // convert the csv to a dict of lists
        for (i, line) in buf_reader.lines().enumerate() {
            let _line = line.unwrap();

            let parsed: HashMap<String, &str> = headers
                .iter()
                .cloned()
                .zip(_line.split(sep))
                .collect();

            let src_name = String::from(parsed[sources_column]);
            let dst_name = String::from(parsed[destinations_column]);
            let edge_type_name = if let Some(etc) = edge_types_column {
                let etn = parsed[*etc];
                Some(String::from(if etn.is_empty() {
                    if let Some(det) = default_edge_type {
                        *det
                    } else {
                        return Err(format!(
                            concat!(
                                "Found empty edge type but no default edge ",
                                "type to use was provided.",
                                "Specifically, the line is the number {i}.\n",
                                "The source node is {source} and destination node is {destination}.\n",
                                "The path of the document was {path}.\n",
                                "The complete line in question is:\n{line}\n"
                            ),
                            i=i,
                            source=src_name,
                            destination=dst_name,
                            path=path,
                            line=_line
                        ))
                    }   
                } else {
                    etn
                }))
            } else {
                None
            };
            
            let weight:Option<WeightT> = if let Some(wc) = weights_column {
                let w = parsed[*wc];
                Some(if w.is_empty() {
                    if let Some(dw) = default_weight {
                        *dw
                    } else {
                        return Err(format!(
                            concat!(
                                "Found empty weight but no default weight ",
                                "to use was provided.",
                                "Specifically, the line is the number {i}.\n",
                                "The source node is {source} and destination node is {destination}.\n",
                                "{edge_type_string}\n",
                                "The path of the document was {path}.\n",
                                "The complete line in question is:\n{line}\n"
                            ),
                            i=i,
                            source=src_name,
                            destination=dst_name,
                            edge_type_string=if let Some(etn) = edge_type_name {
                                format!("The edge type of the row is {}.", etn)
                            } else {
                                String::from("No edge type was detected.")
                            },
                            path=path,
                            line=_line
                        ))
                    }  
                } else {
                    let parsed_weight = w.parse::<WeightT>();
                    if parsed_weight.is_err(){
                        return Err(format!(
                            concat!(
                                "Cannot parse {weight} as float.\n",
                                "Specifically, the line is the number {i}.\n",
                                "The source node is {source} and destination node is {destination}.\n",
                                "{edge_type_string}\n",
                                "The path of the document was {path}.\n",
                                "The complete line in question is:\n{line}\n"
                            ),
                            weight=w,
                            i=i,
                            source=src_name,
                            destination=dst_name,
                            edge_type_string=if let Some(etn) = edge_type_name {
                                format!("The edge type of the row is {}.", etn)
                            } else {
                                String::from("No edge type was detected.")
                            },
                            path=path,
                            line=_line
                        ))
                    }
                    parsed_weight.unwrap()
                })
            } else {
                None
            };

            let edge_type = if let Some(etn) = edge_type_name.clone() {
                if !edge_types_mapping.contains_key(&etn) {
                    edge_types_mapping.insert(
                        etn.clone(),
                        edge_types_reverse_mapping.len() as NodeTypeT,
                    );
                    edge_types_reverse_mapping.push(etn.clone());
                }
                Some(*edge_types_mapping.get(&etn).unwrap())
            } else {
                None
            };

            for node_name in [src_name.clone(), dst_name.clone()].iter(){
                if !nodes_mapping.contains_key(node_name) { 
                    nodes_mapping.insert(node_name.clone(), nodes_reverse_mapping.len());
                    nodes_reverse_mapping.push(node_name.clone());
                }
            }

            let src = *nodes_mapping.get(&src_name).unwrap();
            let dst = *nodes_mapping.get(&dst_name).unwrap();

            let triple = (src, dst, edge_type);
            if unique_edges_set.contains(&triple) {
                if !ignore_duplicated_edges {
                    return Err(format!(
                        concat!(
                            "\nFound duplicated line in edges file!\n",
                            "Specifically, the duplicated line is the number {i}.\n",
                            "The source node is {source} and destination node is {destination}.\n",
                            "{edge_type_string}\n",
                            "The path of the document was {path}.\n",
                            "The complete line in question is:\n{line}\n"
                        ),
                        i = i,
                        source = src_name,
                        destination = dst_name,
                        edge_type_string = if let Some(etn) = edge_type_name {
                            format!("The edge type of the row is {}.", etn)
                        } else {
                            String::from("No edge type was detected.")
                        },
                        path = path,
                        line = _line
                    ));
                }
            } else {
                unique_edges_set.insert(triple);
                sources.push(src);
                destinations.push(dst);
                if let Some(et) = edge_type{
                    edge_types.push(et);
                }
                if let Some(w) = weight{
                    weights.push(w);
                }
            }
        }

        Ok((
            sources,
            destinations,
            nodes_mapping,
            nodes_reverse_mapping,
            if edge_types_column.is_some() {
                Some(edge_types)
            } else {
                None
            },
            if edge_types_column.is_some() {
                Some(edge_types_mapping)
            } else {
                None
            },
            if edge_types_column.is_some() {
                Some(edge_types_reverse_mapping)
            } else {
                None
            },
            if weights_column.is_some() {
                Some(weights)
            } else {
                None
            },
        ))
    }

    fn read_nodes_csv(
        path: &str,
        sep: &str,
        nodes_column: &str,
        nodes_mapping: &HashMap<String, NodeT>,
        node_types_column: &str,
        default_node_type: &Option<&str>,
        ignore_duplicated_nodes: bool
    ) -> Result<(Vec<NodeTypeT>, HashMap<String, NodeTypeT>, Vec<String>), String> {
        let mut nodes: Vec<NodeT> = Vec::new();

        let mut node_types: Vec<NodeTypeT> = Vec::new();
        let mut node_types_mapping: HashMap<String, NodeTypeT> = HashMap::new();
        let mut node_types_reverse_mapping: Vec<String> = Vec::new();

        let mut unique_nodes_set: HashSet<NodeT> = HashSet::new();

        let headers = get_headers(path, sep);

        // open the file
        let file = File::open(path).expect("Cannot open file.");
        let mut buf_reader = BufReader::new(file);
        // Skip header
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        // convert the csv to a dict of lists
        for (j, line) in buf_reader.lines().enumerate() {
            let _line = line.unwrap();

            let parsed: HashMap<String, &str> = headers
                .iter()
                .cloned()
                .zip(_line.split(sep))
                .collect();

            let node = parsed.get(nodes_column).unwrap();
            let maybe_node_id = nodes_mapping.get(*node);
            // if the node is not present in the mapping, then it's a
            // singleton. Therefore it can be ignored and is type doesn't
            // matter
            if maybe_node_id.is_none() {
                continue;
            }
            let node_id = maybe_node_id.unwrap();

            // since the node is not a singleton, add it to the list.
            if unique_nodes_set.contains(node_id) {
                if ignore_duplicated_nodes{
                    continue;
                }
                return Err(format!(
                    concat!(
                        "\nFound duplicated line in nodes file!\n",
                        "Specifically, the duplicated line is the number {j}.\n",
                        "The node is {node}.\n",
                        "The node type of the row is {node_type}.\n",
                        "The path of the document was {path}.\n",
                        "The complete line in question is:\n{line}\n"
                    ),
                    j = j,
                    node = node_id,
                    node_type = node_types[*node_id],
                    path = path,
                    line = _line
                ));
            }

            nodes.push(*node_id);
            unique_nodes_set.insert(*node_id);

            // get and set default for the node type
            let mut node_type = parsed.get(node_types_column).unwrap();
            if node_type.is_empty() {
                if let Some(dnt) = default_node_type {
                    node_type = dnt;
                } else {
                    return Err(format!(
                        concat!(
                            "Found empty node type but no default node ",
                            "type to use was provided.",
                            "Specifically, the line is the number {j}.\n",
                            "The path of the document was {path}.\n",
                            "The complete line in question is:\n{line}\n"
                        ),
                        j=j,
                        path=path,
                        line=_line
                    ))
                }
            }

            // update node_types_mapping with the new node type
            if !node_types_mapping.contains_key(*node_type) {
                node_types_mapping.insert(
                    String::from(*node_type),
                    node_types_reverse_mapping.len() as NodeTypeT,
                );
                node_types_reverse_mapping.push(String::from(*node_type));
            }

            node_types.push(*node_types_mapping.get(*node_type).unwrap());
        }

        if nodes.len() != nodes_mapping.len() {
            return Err(
                format!(
                    concat!(
                        "The size of the given nodes_mapping {} does not match the number of nodes found {}.\n",
                        "This means that there are more nodes used in the edges file than are described in the nodes file.\n",
                        "This might be due to a mismatch between the edge and node files.\n",
                        "The path of the file is {}"
                    ),
                    nodes_mapping.len(), nodes.len(), path
                )
            );
        }

        // Sort the node types using the indices order specified by the nodes
        let sorted_node_types: Vec<NodeTypeT> = nodes.par_iter().map(|x| node_types[*x]).collect();

        // return the results
        Ok((
            sorted_node_types,
            node_types_mapping,
            node_types_reverse_mapping,
        ))
    }

    pub fn from_csv(
        edge_path: &str,
        sources_column: &str,
        destinations_column: &str,
        directed: bool,
        edge_types_column: Option<&str>,
        default_edge_type: Option<&str>,
        weights_column: Option<&str>,
        default_weight: Option<WeightT>,
        node_path: Option<&str>,
        nodes_column: Option<&str>,
        node_types_column: Option<&str>,
        default_node_type: Option<&str>,
        edge_sep: Option<&str>,
        node_sep: Option<&str>,
        validate_input_data: Option<bool>,
        ignore_duplicated_edges: Option<bool>,
        ignore_duplicated_nodes: Option<bool>,
        force_conversion_to_undirected: Option<bool>
    ) -> Result<Graph, String> {
        // If the separators were not provided we use by default tabs.
        let _edge_sep = edge_sep.unwrap_or_else(|| "\t");
        let _node_sep = node_sep.unwrap_or_else(|| "\t");
        let _validate_input_data = validate_input_data.unwrap_or_else(|| true);
        let _ignore_duplicated_edges = ignore_duplicated_edges.unwrap_or_else(|| false);
        let _ignore_duplicated_nodes = ignore_duplicated_nodes.unwrap_or_else(|| false);

        if _validate_input_data {
            // We validate the provided files, starting from the edges file.
            // Specifically, we start by checking if every line has the same amount
            // of the given separator character.
            check_consistent_lines(&*edge_path, &*_edge_sep)?;
            // Then we check if the given columns actually exist in the given file
            // header.
            has_columns(
                &*edge_path,
                &*_edge_sep,
                &[&sources_column, &destinations_column],
                &[&edge_types_column, &weights_column],
            )?;

            // If the nodes path was provided, we also validate it.
            if let Some(path) = &node_path {
                // As for the previous file, first we check that the file has the
                // same amount of separators in each line.
                check_consistent_lines(&*path, &*_node_sep)?;
                if nodes_column.is_none() || node_types_column.is_none(){
                    return Err(String::from(concat!(
                        "If the node_path is passed, ",
                        "the nodes_column and node_types_column",
                        " parameters are also required."
                    )))
                }
                // Then we check if the given columns actually exists in the file.
                has_columns(
                    &*path,
                    &*_node_sep,
                    &[&nodes_column.clone().unwrap(), &node_types_column.unwrap()],
                    &[],
                )?;
            }
        }

        let (
            sources,
            destinations,
            nodes_mapping,
            nodes_reverse_mapping,
            edge_types,
            edge_types_mapping,
            edge_types_reverse_mapping,
            weights,
        ) = Graph::read_edges_csv(
            &edge_path,
            &_edge_sep,
            &sources_column,
            &destinations_column,
            &edge_types_column,
            &default_edge_type,
            &weights_column,
            &default_weight,
            _ignore_duplicated_edges
        )?;

        let (node_types, node_types_mapping, node_types_reverse_mapping) =
            if let Some(path) = &node_path {
                let (node_types, node_types_mapping, node_types_reverse_mapping) =
                    Graph::read_nodes_csv(
                        &path,
                        &_node_sep,
                        &nodes_column.unwrap(),
                        &nodes_mapping,
                        &node_types_column.unwrap(),
                        &default_node_type,
                        _ignore_duplicated_nodes
                    )?;
                (
                    Some(node_types),
                    Some(node_types_mapping),
                    Some(node_types_reverse_mapping),
                )
            } else {
                (None, None, None)
            };

        if directed {
            Graph::new_directed(
                sources,
                destinations,
                Some(nodes_mapping),
                Some(nodes_reverse_mapping),
                node_types,
                node_types_mapping,
                node_types_reverse_mapping,
                edge_types,
                edge_types_mapping,
                edge_types_reverse_mapping,
                weights,
                Some(_validate_input_data),
            )
        } else {
            Graph::new_undirected(
                sources,
                destinations,
                Some(nodes_mapping),
                Some(nodes_reverse_mapping),
                node_types,
                node_types_mapping,
                node_types_reverse_mapping,
                edge_types,
                edge_types_mapping,
                edge_types_reverse_mapping,
                weights,
                Some(_validate_input_data),
                force_conversion_to_undirected
            )
        }
    }
}
