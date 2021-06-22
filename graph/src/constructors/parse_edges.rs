use super::*;

fn parse_unsorted_edges(
    edges_iterator: Option<
        impl ParallelIterator<Item = Result<(String, String, Option<String>, Option<String>)>>,
    >,
    nodes: Vocabulary<NodeT>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    edge_list_is_correct: Option<bool>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>)> {
    let edge_list_is_correct = edge_list_is_correct.unwrap_or(false);
    let numeric_edge_list_node_ids = numeric_edge_list_node_ids.unwrap_or(false);
    let numeric_edge_list_edge_type_ids = numeric_edge_list_edge_type_ids.unwrap_or(false);

    if edges_iterator.is_none() && edge_types_vocabulary.is_some() {
        return Err(concat!(
            "Edge types vocabulary was provided ",
            "but no edge list was given."
        )
        .to_string());
    }

    let has_edge_types = edge_types_vocabulary.is_some();

    if !has_edge_types && numeric_edge_list_edge_type_ids {
        return Err(concat!(
            "The numeric node list node type IDs parameter does not make sense ",
            "in the context where the node types have not been provided.\n",
            "If the node types within the nodes list are numeric, simply use ",
            "the numeric node types ids parameter."
        ));
    }

    let edge_types_method = match (
        has_edge_types,
        edge_types_vocabulary
            .as_ref()
            .map_or(true, |x| x.is_empty()),
        edge_list_is_correct,
        numeric_edge_list_edge_type_ids,
    ) {
        (false, _, _, false) => EdgeTypeParser::ignore,
        (true, true, true, false) => EdgeTypeParser::parse_strings_unchecked,
        (true, true, false, false) => EdgeTypeParser::parse_strings,
        (true, false, true, false) => EdgeTypeParser::translate_unchecked,
        (true, false, false, false) => EdgeTypeParser::translate,
        (_, _, true, true) => EdgeTypeParser::to_numeric_unchecked,
        (_, _, false, true) => EdgeTypeParser::to_numeric,
    };
    let mut edge_types_vocabulary = edge_types_vocabulary.unwrap_or(Vocabulary::new());

    let edges_iterator =
        edges_iterator.map(|ei| ei.method_caller(edge_types_method, &mut edge_types_vocabulary));

    let node_method = match (
        has_edge_types,
        nodes.is_empty(),
        edge_list_is_correct,
        numeric_edge_list_edge_type_ids,
    ) {
        (false, _, _, false) => EdgeNodeNamesParser::ignore,
        (true, true, true, false) => EdgeNodeNamesParser::parse_strings_unchecked,
        (true, true, false, false) => EdgeNodeNamesParser::parse_strings,
        (true, false, true, false) => EdgeNodeNamesParser::translate_unchecked,
        (true, false, false, false) => EdgeNodeNamesParser::translate,
        (_, _, true, true) => EdgeNodeNamesParser::to_numeric_unchecked,
        (_, _, false, true) => EdgeNodeNamesParser::to_numeric,
    };

    let edges_iterator = edges_iterator.map(|ei| ei.method_caller(node_method, &mut nodes));
}
