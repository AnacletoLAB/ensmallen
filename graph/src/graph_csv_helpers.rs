use super::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use log::info;

#[inline(always)]
/// Check that the current parsed line is consistent with the rest of the file
/// Meaning that it must have the same number of fields and that it's not empty.
pub(crate) fn check_line_consistency(
    parsed: &HashMap<String, String>, 
    number_of_separators: usize, 
    line: &str, 
    line_index: usize
) -> Result<(), String>{
    if parsed.len() != number_of_separators {
        return Err(format!(
            concat!(
                "Provided edges file has malformed lines. ",
                "The provided lines have different numbers ",
                "of the given separator.\n",
                "The expected number of separators was {expected_length}, ",
                "but a line with {separators_number} separators was found. \n",
                "The line is the number {counter}.\n",
                "The line in question is: '{line}'\n",
            ),
            expected_length = number_of_separators,
            separators_number =  parsed.len(),
            counter = line_index,
            line = line
        ));
    }
    if parsed.len() == 0 {
        return Err(format!(
            concat!(
                "Provided edges file has malformed lines. ",
                "The provided lines have no instances ",
                "of the given separator.\n",
                "The line is the number {counter}.\n",
                "The line in question is the {line_index} and it's content is '{line}'\n",
            ),
            counter = parsed.len(),
            line_index = line_index,
            line = line
        ));
    }
    Ok(())
}

#[inline(always)]
/// Function that allows us to either load a node files and then map the edges on it
/// or to just load the edges file and create the node mapping.
/// The only thing the user needs to know is that it returns the ids of the given
/// src_name and dst_name.
pub(crate) fn get_nodes_ids_and_map(
    node_types: &Option<Vec<NodeTypeT>>,
    nodes_mapping: &mut HashMap<String, NodeT>,
    nodes_reverse_mapping: &mut Vec<String>, 
    src_name: &str, 
    dst_name: &str,
    line: &str, 
    line_index: usize
) -> Result<(usize,usize), String> {
    Ok(// if the node file was provided
        if node_types.is_some() {
            (
                // TODO find how to reduce duplication, what is better a function or a macro in this case?
                *nodes_mapping.get(src_name.clone()).ok_or(format!(concat!(
                    "The node {node_name} is not present in the provided nodes file.\n",
                    "The complete line is in question is the {line_index} and its content is:\n{line}\n"
                ),
                    node_name = src_name,
                    line=line,
                    line_index=line_index
                ))?,
                *nodes_mapping.get(dst_name.clone()).ok_or(format!(concat!(
                    "The node {node_name} is not present in the provided nodes file.\n",
                    "The complete line is in question is the {line_index} and its content is:\n{line}\n"
                ),
                    node_name = dst_name,
                    line=line,
                    line_index=line_index
                ))?
            )
        } else { // if no node file was provided we must create the mappings
            // update the mappings
            (
                // TODO find how to reduce duplication, what is better a function or a macro in this case?
                match nodes_mapping.get(src_name.clone()) {
                    Some(g) => *g,
                    None => {
                        let new_id = nodes_reverse_mapping.len();
                        nodes_mapping.insert(src_name.to_string(), new_id);
                        nodes_reverse_mapping.push(src_name.to_string());
                        new_id
                    }
                },
                match nodes_mapping.get(dst_name.clone()) {
                    Some(g) => *g,
                    None => {
                        let new_id = nodes_reverse_mapping.len();
                        nodes_mapping.insert(dst_name.to_string(), new_id);
                        nodes_reverse_mapping.push(dst_name.to_string());
                        new_id
                    }
                }
            )
        }
    )
}


pub(crate) fn parse_weight(
    parsed: &HashMap<String, String>, 
    weights_column: &Option<String>,
    default_weight: &Option<WeightT>,
    line: &str,
    line_index: usize
) -> Result<Option<WeightT>, String> {
    match weights_column {
        // if no colums is passed, ignore the weights
        None => Ok(None),
        Some(w_column) => {
            let weight_str = &parsed[w_column];
            // if the column is present but the field is empty return default if present.
            if weight_str.is_empty() {
                return match default_weight {
                    Some(dw) => Ok(Some(*dw)),
                    None => {
                        Err(format!(
                            concat!(
                                "Found empty weight but no default weight to use was provided.",
                                "Specifically, the line is the number {line_index}.\n",
                                "The complete line in question is:\n{line}\n"
                            ),
                            line=line,
                            line_index=line_index
                        ))
                    }
                }
            }
            // else, parse the string
            return match weight_str.parse::<WeightT>() {
                Ok(g) => Ok(Some(g)),
                Err(_) => {
                    Err(format!(
                        concat!(
                            "Cannot parse {weight} as float.\n",
                            "Specifically, the line is the number {line_index}.\n",
                            "The complete line in question is:\n{line}\n"
                        ),
                        weight=weight_str,
                        line_index=line_index,
                        line=line
                    ))
                }
            }
        }

    }
}

pub(crate) fn parse_edge_type_name(
    parsed: &HashMap<String, String>, 
    edge_types_column: &Option<String>,
    default_edge_type: &Option<String>,
    line: &str,
    line_index: usize
) -> Result<Option<String>, String> {
    match edge_types_column {
        None => Ok(None),
        Some(et_column) => {
            let edge_type = &parsed[et_column];
            // if the column is present but the field is empty return default if present.
            if edge_type.is_empty() {
                return match default_edge_type {
                    Some(et) => Ok(Some(et.to_string())),
                    None => {
                        Err(format!(
                            concat!(
                                "Found empty edge type but no default edge type to use was provided.",
                                "Specifically, the line is the number {line_index}.\n",
                                "The complete line in question is:\n{line}\n"
                            ),
                            line=line,
                            line_index=line_index
                        ))
                    }
                };
            }
            Ok(Some(edge_type.clone()))
        }
    }
}


pub fn validate(
    sources: &[NodeT],
    destinations: &[NodeT],
    nodes_mapping: &HashMap<String, NodeT>,
    nodes_reverse_mapping: &[String],
    node_types: &Option<Vec<NodeTypeT>>,
    edge_types: &Option<Vec<EdgeTypeT>>,
    weights: &Option<Vec<WeightT>>,
) -> Result<(), String> {
    info!("Checking that the graph is not empty.");
    if sources.is_empty() {
        return Err(String::from("The provided graph has no edges."));
    }

    info!("Checking that the nodes mappings are of the same length.");
    if nodes_mapping.len() != nodes_reverse_mapping.len() {
        return Err(format!("The size of the node_mapping ({}) does not match the size of the nodes_reverse_mapping ({}).",
            nodes_mapping.len(), nodes_reverse_mapping.len()
        ));
    }

    if let Some(nt) = &node_types {
        info!("Checking that nodes and node types are of the same length.");
        if nt.len() != nodes_reverse_mapping.len() {
            return Err(format!(
                "The number of given nodes ({}) does not match the number of node_types ({}).",
                nt.len(),
                nodes_reverse_mapping.len()
            ));
        }
    }

    if let Some(nt) = &node_types {
        info!("Checking if every node used by the edges exists.");
        for node in sources.iter().chain(destinations.iter()) {
            if *node >= nt.len() {
                return Err(format!(
                    "A node provided with the edges ('{}') does not exists within given nodes.",
                    node
                ));
            }
        }
    }

    if let Some(w) = weights {
        info!("Checking for length between weights and given edges.");
        if w.len() != sources.len() {
            return Err(format!(
                "Length of given weights ({}) does not match length of given edges ({}).",
                w.len(),
                sources.len()
            ));
        }
        info!("Checking for non-zero weights.");
        for weight in w.iter() {
            if *weight == 0.0 {
                return Err(format!(
                    "One of the provided weights '{}' is either 0 or within float error to zero.",
                    weight
                ));
            }
            if *weight < 0.0 {
                return Err(format!(
                    "One of the provided weights '{}' is negative.",
                    weight
                ));
            }
            if weight.is_nan() {
                return Err(String::from("One of the provided weights is NaN."));
            }
            if weight.is_infinite() {
                return Err(String::from("One of the provided weights is infinite."));
            }
        }
    }

    if let Some(et) = edge_types {
        info!("Checking for length between edge types and given edges.");
        if et.len() != sources.len() {
            return Err(format!(
                "The len of edge types ({}) is different than the len of given edges ({}).  ",
                et.len(),
                sources.len()
            ));
        }
    }

    info!("Checking for unique edges (including edge types).");
    let mut unique_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();
    for i in 0..sources.len() {
        let src = sources[i];
        let dst = destinations[i];
        let edge_type = if let Some(et) = edge_types {
            Some(et[i])
        } else {
            None
        };
        if unique_edges.contains(&(src, dst, edge_type)) {
            return Err(format!(
                concat!(
                    "Duplicated edge was found within given edges.\n",
                    "The source node is {src}.\n",
                    "The destination node is {dst}.\n",
                    "{edge_type_message}\n",
                    "This issue is relative to the graph building and not ",
                    "the CSV reader, hence it can not be addressed by passing ",
                    "the parameter ignore_duplicated_edges."
                ),
                src = src,
                dst = dst,
                edge_type_message = if let Some(et) = edge_type {
                    format!("The edge type is {}", et)
                } else {
                    String::from("No edge type was detected.")
                }
            ));
        }
        unique_edges.insert((src, dst, edge_type));
    }

    Ok(())
}