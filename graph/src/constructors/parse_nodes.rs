use rayon::iter::ParallelIterator;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

use super::*;

pub(crate) fn parse_nodes(
    nodes_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
    >,
    nodes_number: Option<NodeT>,
    node_types_vocabulary: Option<Vocabulary<NodeTypeT>>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_ids: Option<NodeT>,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>)> {
    if !numeric_node_ids && minimum_node_ids.is_some() {
        return Err(
            "Giving the minimum id is not meaningfull when numeric_ids is false.".to_string(),
        );
    }

    if nodes_iterator.is_none() && node_types_vocabulary.is_some() {
        return Err(
            "Node types vocabulary was provided but no nodes iterator was given.".to_string(),
        );
    }

    let has_node_types = node_types_vocabulary.is_some();

    if !has_node_types && numeric_node_list_node_type_ids {
        return Err(concat!(
            "The numeric node list node type IDs parameter does not make sense ",
            "in the context where the node types have not been provided.\n",
            "If the node types within the nodes list are numeric, simply use ",
            "the numeric node types ids parameter."
        )
        .to_string());
    }

    let node_types_method = match (
        has_node_types,
        node_types_vocabulary
            .as_ref()
            .map_or(true, |x| x.is_empty()),
        node_list_is_correct,
        numeric_node_list_node_type_ids,
    ) {
        (false, _, _, false) => NodeTypeParser::ignore,
        (true, true, true, false) => NodeTypeParser::parse_strings_unchecked,
        (true, true, false, false) => NodeTypeParser::parse_strings,
        (true, false, true, false) => NodeTypeParser::get_unchecked,
        (true, false, false, false) => NodeTypeParser::get,
        (_, _, true, true) => NodeTypeParser::to_numeric_unchecked,
        (_, _, false, true) => NodeTypeParser::to_numeric,
    };
    let node_types_vocabulary = node_types_vocabulary.unwrap_or(Vocabulary::new());

    let mut node_type_parser = NodeTypeParser::new(node_types_vocabulary);
    let nodes_iterator =
        nodes_iterator.map(|ni| ni.method_caller(node_types_method, &mut node_type_parser));

    let (nodes_vocabulary, node_types_ids, node_types_vocabulary) = match (
        nodes_iterator,
        nodes_number,
        numeric_node_ids,
        minimum_node_ids,
    ) {
        // When the nodes iterator was provided, and the node IDs are expected
        // NOT to be numeric and a minimum node ID is therefore meaningless.
        (Some(ni), _, false, None) => {
            let (nodes_names, node_types_ids): (Vec<String>, Option<Vec<Option<Vec<NodeTypeT>>>>) =
                if has_node_types {
                    // If there are node types we need to collect them.
                    // We need to use the unzip utility because in this context we do not
                    // know the number of the nodes and we need to use a ParallellIterator,
                    // note that it is NOT an IndexedParallellIterator.
                    let (nodes_names, node_types_ids) = ni
                        .map(|line| line.map(|(_, node_and_node_type)| node_and_node_type))
                        .collect::<Result<(Vec<String>, Vec<Option<Vec<NodeTypeT>>>)>>()?;
                    (nodes_names, Some(node_types_ids))
                } else {
                    (
                        ni.map(|x| x.map(|(_, (name, _))| name))
                            .collect::<Result<Vec<String>>>()?,
                        None,
                    )
                };
            let mut node_type_vocabulary = node_type_parser.into_inner();
            if node_type_vocabulary.is_empty() {
                node_type_vocabulary.build()?;
            }

            println!("{:?}", nodes_names);
            Ok::<_, String>((
                Vocabulary::from_reverse_map(nodes_names)?,
                node_types_ids,
                Some(node_type_vocabulary),
            ))
        }
        // When the node iterator was provided, and the nodes number is not known
        // and the node IDs are expected to be numeric.
        (Some(ni), None, true, _) => {
            // In case the node types are expected to exist.
            let (min, max, node_types_ids) = if has_node_types {
                let min = AtomicU32::new(NodeT::MAX);
                let max = AtomicU32::new(0);
                let node_type_ids = ni
                    .map(|line| match line {
                        Ok((line_number, (node_name, node_type_ids))) => {
                            let node_id = match node_name.parse::<NodeT>() {
                                Ok(node_id) => Ok(node_id),
                                Err(_) => Err(format!(
                                    concat!(
                                        "While parsing the provided node list, ",
                                        "the node ID {:?} was found and it is not ",
                                        "possible to convert it to an integer as was requested.\n",
                                        "Specifically the line with the error is {}."
                                    ),
                                    node_name, line_number
                                )),
                            }?;
                            min.fetch_min(node_id, Relaxed);
                            max.fetch_max(node_id, Relaxed);
                            Ok(node_type_ids)
                        }
                        Err(e) => Err(e),
                    })
                    .collect::<Result<Vec<Option<Vec<NodeTypeT>>>>>()?;
                (
                    min.into_inner(),
                    max.into_inner(),
                    optionify!(node_type_ids),
                )
            } else {
                // Alternatively we can focus exclusively on the
                // node IDs, which being numeric boil down to collecting
                // the minimum and the maximum value.
                let (min, max): (NodeT, NodeT) = ni
                    .map(|line| match line {
                        Ok((line_number, (node_name, _))) => match node_name.parse::<NodeT>() {
                            Ok(node_id) => Ok(node_id),
                            Err(_) => Err(format!(
                                concat!(
                                    "While parsing the provided node list, ",
                                    "the node ID {:?} was found and it is not ",
                                    "possible to convert it to an integer as was requested.\n",
                                    "Specifically the line with the error is {}."
                                ),
                                node_name, line_number
                            )),
                        },
                        Err(e) => Err(e),
                    })
                    .map(|maybe_node_id: Result<NodeT>| {
                        maybe_node_id.map(|node_id| (node_id, node_id))
                    })
                    .try_reduce(
                        || (NodeT::MAX, 0 as NodeT),
                        |(min1, max1): (NodeT, NodeT), (min2, max2): (NodeT, NodeT)| {
                            Ok((min1.min(min2), max1.max(max2)))
                        },
                    )?;
                (min, max, None)
            };
            let minimum_node_ids = minimum_node_ids.unwrap_or(min);

            if min < minimum_node_ids {
                return Err(format!(
                    concat!(
                        "The given minimum id {:?} is higher ",
                        "than the minimum id found in the iterator {:?}."
                    ),
                    minimum_node_ids, min
                ));
            }

            let mut node_type_vocabulary = node_type_parser.into_inner();
            if node_type_vocabulary.is_empty() {
                node_type_vocabulary.build()?;
            }

            Ok((
                Vocabulary::from_range(min.min(minimum_node_ids)..max),
                node_types_ids,
                Some(node_type_vocabulary),
            ))
        }
        (None, Some(ntn), true, None) => Ok((Vocabulary::from_range(0..ntn), None, None)),
        (None, Some(ntn), true, Some(min_val)) => {
            Ok((Vocabulary::from_range(min_val..min_val + ntn), None, None))
        }
        (None, None, true, _) => {
            let min = minimum_node_ids.unwrap_or(0);
            Ok((Vocabulary::from_range(min..min), None, None))
        }
        (None, Some(ntn), false, None) => Ok((Vocabulary::with_capacity(ntn as usize), None, None)),
        (None, None, false, None) => Ok((Vocabulary::new(), None, None)),
        // TODO! imporve error
        _ => unreachable!("All other cases must be explictily handled."),
    }?;

    Ok((
        nodes_vocabulary,
        NodeTypeVocabulary::from_option_structs(node_types_ids, node_types_vocabulary),
    ))
}
