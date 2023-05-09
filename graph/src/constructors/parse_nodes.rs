use rayon::iter::ParallelIterator;

use super::*;

pub(crate) fn parse_nodes(
    nodes_iterator: Option<
        ItersWrapper<
            Result<(usize, (String, Option<Vec<String>>))>,
            impl Iterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
            impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
        >,
    >,
    nodes_number: Option<NodeT>,
    node_types_vocabulary: Option<Vocabulary<NodeTypeT>>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_id: Option<NodeT>,
    skip_node_types_if_unavailable: Option<bool>,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>)> {
    let skip_node_types_if_unavailable = skip_node_types_if_unavailable.unwrap_or(false);
    if !numeric_node_ids && minimum_node_id.is_some() {
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

    if !has_node_types && !skip_node_types_if_unavailable && numeric_node_list_node_type_ids {
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
        (false, _, _, _) => NodeTypeParser::ignore,
        (true, true, true, false) => NodeTypeParser::parse_strings_unchecked,
        (true, true, false, false) => NodeTypeParser::parse_strings,
        (true, false, true, false) => NodeTypeParser::get_unchecked,
        (true, false, false, false) => NodeTypeParser::get,
        (true, _, true, true) => NodeTypeParser::to_numeric_unchecked,
        (true, _, false, true) => NodeTypeParser::to_numeric,
    };
    let node_types_vocabulary = node_types_vocabulary.unwrap_or(Vocabulary::new(true));

    let mut node_type_parser = NodeTypeParser::new(node_types_vocabulary);
    let nodes_iterator: Option<
        ItersWrapper<Result<(usize, (String, Option<Vec<NodeTypeT>>))>, _, _>,
    > = nodes_iterator
        .map(|ni| ni.method_caller(node_types_method, node_types_method, &mut node_type_parser));

    let (nodes_vocabulary, node_types_ids, node_types_vocabulary) = match (
        nodes_iterator,
        nodes_number,
        numeric_node_ids,
        minimum_node_id,
        node_list_is_correct,
    ) {
        // When the nodes iterator was provided, and the node IDs are expected
        // NOT to be numeric and a minimum node ID is therefore meaningless.
        // Note that this is the use case when the node list is ASSUMED TO BE CORRECT
        // and the total number of nodes is known and provided.
        (Some(ni), Some(nodes_number), false, None, true) => {
            let (node_names, node_types_ids): (Vec<String>, Option<Vec<Option<Vec<NodeTypeT>>>>) =
                if has_node_types {
                    // If there are node types we need to collect them.
                    // We cannot use the unzip utility because in this context
                    // since we need to use a ParallellIterator,
                    // note that it is NOT an IndexedParallellIterator.
                    // Since we know the number of nodes and the node list
                    // is provided as correct, it is possible to pre-allocate the vectors
                    // and populate them with a foreach.
                    let node_names =
                        ThreadDataRaceAware::new(vec!["".to_owned(); nodes_number as usize]);
                    let node_types_ids =
                        ThreadDataRaceAware::new(vec![None; nodes_number as usize]);
                    ni.for_each(|line| unsafe {
                        // We can unwrap because the user tells us that this is surely
                        // a correct node list.
                        let (line_number, (node_name, node_type_ids)) = line.unwrap();
                        (*node_names.value.get())[line_number] = node_name;
                        (*node_types_ids.value.get())[line_number] = node_type_ids;
                    });
                    let node_type_ids = node_types_ids.value.into_inner();
                    (node_names.value.into_inner(), optionify!(node_type_ids))
                } else {
                    let node_names =
                        ThreadDataRaceAware::new(vec!["".to_owned(); nodes_number as usize]);
                    ni.for_each(|line| unsafe {
                        // We can unwrap because the user tells us that this is surely
                        // a correct node list.
                        let (line_number, (node_name, _)) = line.unwrap();
                        (*node_names.value.get())[line_number] = node_name;
                    });
                    (node_names.value.into_inner(), None)
                };
            let mut node_type_vocabulary = node_type_parser.into_inner();
            if node_type_vocabulary.is_empty() {
                node_type_vocabulary.build()?;
            }

            Ok::<_, String>((
                Vocabulary::from_reverse_map(node_names)?,
                node_types_ids,
                Some(node_type_vocabulary),
            ))
        }
        // When the nodes iterator was provided, and the node IDs are expected
        // NOT to be numeric and a minimum node ID is therefore meaningless.
        // Note that this is the use case when it is not known if the node list is
        // correct and how many nodes are inside it.
        (Some(ni), _, false, None, _) => {
            let (node_names, node_types_ids): (Vec<String>, Option<Vec<Option<Vec<NodeTypeT>>>>) =
                if has_node_types {
                    // If there are node types we need to collect them.
                    // We need to use the unzip utility because in this context we do not
                    // know the number of the nodes and we need to use a ParallellIterator,
                    // note that it is NOT an IndexedParallellIterator.
                    let (node_names, node_types_ids) = match ni
                        .map(|line| line.map(|(_, node_and_node_type)| node_and_node_type))
                    {
                        ItersWrapper::Parallel(ni_par) => ni_par
                            .collect::<Result<(Vec<String>, Vec<Option<Vec<NodeTypeT>>>)>>()?,
                        ItersWrapper::Sequential(ni_seq) => {
                            let mut node_names = Vec::new();
                            let mut node_types_ids = Vec::new();
                            for line in ni_seq {
                                let (node_name, node_type_ids) = line?;
                                node_names.push(node_name);
                                node_types_ids.push(node_type_ids);
                            }
                            (node_names, node_types_ids)
                        }
                    };

                    (node_names, optionify!(node_types_ids))
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

            Ok::<_, String>((
                Vocabulary::from_reverse_map(node_names)?,
                node_types_ids,
                Some(node_type_vocabulary),
            ))
        }
        // When the node iterator was provided, and the nodes number is not known
        // and the node IDs are expected to be numeric.
        (Some(ni), maybe_nodes_number, true, _, _) => {
            // In case the node types are expected to exist.
            let (min, max) = if has_node_types {
                return Err(concat!(
                    "This case is not supported. You cannot have a nodes iterator of numeric node ids with node types.",
                    " This would require to sort the csv and thus it requires a higher memory peak.",
                    " If you want to load it just set numeric_node_ids to false and load them as strings.",
                ).to_string());
            } else {
                // Alternatively we can focus exclusively on the
                // node IDs, which being numeric boil down to collecting
                // the minimum and the maximum value.
                let (mut min, mut max, actual_nodes_number): (NodeT, NodeT, NodeT) = ni
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
                        maybe_node_id.map(|node_id| (node_id, node_id, 1))
                    })
                    .reduce(
                        || Ok((NodeT::MAX, 0 as NodeT, 0)),
                        |line1: Result<(NodeT, NodeT, NodeT)>,
                         line2: Result<(NodeT, NodeT, NodeT)>| match (
                            line1, line2,
                        ) {
                            (Ok((min1, max1, count1)), Ok((min2, max2, count2))) => {
                                Ok((min1.min(min2), max1.max(max2), count1 + count2))
                            }
                            (Err(e), _) => Err(e),
                            (_, Err(e)) => Err(e),
                        },
                    )?;

                if actual_nodes_number == 0 {
                    min = 0;
                    max = 0;
                }

                if let Some(nn) = maybe_nodes_number {
                    if nn != max - min {
                        return Err(format!(
                                "The given nodes number '{}' is different from the actual nodes number '{}'.",
                                nn, actual_nodes_number,
                            ));
                    }
                }

                (min, max)
            };
            let minimum_node_id = minimum_node_id.unwrap_or(min);

            if min < minimum_node_id {
                return Err(format!(
                    concat!(
                        "The given minimum id {:?} is higher ",
                        "than the minimum id found in the iterator {:?}."
                    ),
                    minimum_node_id, min
                ));
            }

            let mut node_type_vocabulary = node_type_parser.into_inner();
            if node_type_vocabulary.is_empty() {
                node_type_vocabulary.build()?;
            }

            Ok((
                Vocabulary::from_range(min.min(minimum_node_id)..(max + 1)),
                None,
                Some(node_type_vocabulary),
            ))
        }
        (None, Some(ntn), true, None, _) => Ok((Vocabulary::from_range(0..ntn), None, None)),
        (None, Some(ntn), true, Some(min_val), _) => {
            let max = match min_val.checked_add(ntn){
                Some(max) => Ok(max),
                None => Err(format!(
                    concat!(
                        "To compute the maximum node type, it is needed to sum ",
                        "the minimum node type ID `{}` to the provided number of node types `{}`, ",
                        "but this would lead to an overflow, that is a value higher than the maximum U32."
                    ),
                    min_val, ntn
                ))
            }?;
            Ok((Vocabulary::from_range(min_val..max), None, None))
        }
        (None, None, true, _, _) => {
            let min = minimum_node_id.unwrap_or(0);
            Ok((Vocabulary::from_range(min..min), None, None))
        }
        (None, Some(ntn), false, None, _) => {
            Ok((Vocabulary::with_capacity(ntn as usize, true), None, None))
        }
        (None, None, false, None, _) => Ok((Vocabulary::new(true), None, None)),
        // TODO! improve error
        _ => unreachable!("All other cases must be explicitly handled."),
    }?;

    // Executing self-consistency check for the node type IDs
    if node_types_ids.as_ref().map_or(false, |node_types_ids| {
        nodes_vocabulary.len() != node_types_ids.len()
    }) {
        panic!(
            concat!(
                "The length of the nodes vocabulary is {}, ",
                "while the length of the node type IDs vector is {}."
            ),
            nodes_vocabulary.len(),
            node_types_ids.unwrap().len()
        );
    }

    Ok((
        nodes_vocabulary,
        NodeTypeVocabulary::from_option_structs(node_types_ids, node_types_vocabulary),
    ))
}
