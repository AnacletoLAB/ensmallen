use std::sync::atomic::AtomicU32;

use super::*;

fn parse_nodes(
    nodes_iterator: Option<impl ParallelIterator<Item = Result<(String, Option<Vec<String>>)>>>,
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
        ));
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
        (true, false, true, false) => NodeTypeParser::translate_unchecked,
        (true, false, false, false) => NodeTypeParser::translate,
        (_, _, true, true) => NodeTypeParser::to_numeric_unchecked,
        (_, _, false, true) => NodeTypeParser::to_numeric,
    };
    let mut node_types_vocabulary = node_types_vocabulary.unwrap_or(Vocabulary::new());

    let nodes_iterator =
        nodes_iterator.map(|ni| ni.method_caller(node_types_method, &mut node_types_vocabulary));

    let (nodes_vocabulary, node_types_ids) = match (
        nodes_iterator,
        nodes_number,
        numeric_node_ids,
        minimum_node_ids,
    ) {
        (Some(ni), _, false, None) => {
            let (nodes_names, node_types_ids): (Vec<String>, Option<Vec<Option<Vec<NodeTypeT>>>>) =
                if has_node_types {
                    let (nodes_names, node_types_ids) = ni.unzip()?;
                    (nodes_names, Some(node_types_ids))
                } else {
                    (
                        ni.map(|x| x.map_ok(|(name, _)| name)).collect(),
                        node_types_vocabulary,
                    )
                };
            (Vocabulary::from_reverse_map(nodes_names), node_types_ids)
        }
        (Some(ni), None, true, _) => {
            let (min, max, node_types_ids) = if has_node_types {
                let min = AtomicU32::new(NodeT::MAX);
                let max = AtomicU32::new(0);
                let node_type_ids = ni
                    .map(|(node_name, node_type_ids)| {
                        let node_id = node_name.parse::<NodeT>().map_err(|_| {
                            format!(
                                "The node name '{}' cannot be parsed as an integer",
                                node_name
                            )
                        })?;
                        min.fetch_min(node_id);
                        max.fetch_max(node_id);
                        Ok(node_type_ids)
                    })
                    .collect::<Result<Vec<Option<Vec<NodeTypeT>>>>>()?;
                (min.into_inner(), max.into_inner(), Some(node_type_ids))
            } else {
                let (min, max) = ni
                    .map(|x| {
                        x.parse::<NodeT>().map_err(|_| {
                            format!("The string '{}' cannot be parsed as an integer", x)
                        })
                    })
                    .map(|x| (x, x))
                    .reduce(|v1, v2| match (v1, v2) {
                        (Ok(min1, max1), Ok(min2, max2)) => Ok((min1.min(min2), max1.max(max2))),
                        (Err(e), _) | (_, Err(e)) => Err(e),
                    })?;
                (min, max, None)
            };
            let minimum_node_ids = minimum_node_ids.unwrap_or(min);

            if min < minimum_node_ids {
                // TODO! improve error
                return Err(
                    "The given minimum id is bigger than the minimum id found in the iterator",
                );
            }

            Ok(
                Vocabulary::from_range(min.min(minimum_node_ids)..max),
                node_types_ids,
            )
        }
        (None, Some(ntn), true, None) => Ok(Vocabulary::from_range(0..ntn)),
        (None, Some(ntn), true, Some(min_val)) => {
            Ok(Vocabulary::from_range(min_val..min_val + ntn))
        }
        (None, Some(ntn), true, _) => {
            let min = minimum_node_ids.unwrap_or(0);
            Ok(Vocabulary::from_range(min..min))
        }
        (None, Some(ntn), false, None) => Ok(Vocabulary::with_capacity(ntn)),
        (None, None, false, None) => Ok(Vocabulary::new()),
        // TODO! imporve error
        _ => unreachable!("All other cases must be explictily handled."),
    };

    (
        nodes_vocabulary,
        node_types_ids.map(|ntis| NodeTypeVocabulary::from_structs(ntis, node_types_vocabulary)),
    )
}
