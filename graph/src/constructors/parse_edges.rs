use super::*;

fn parse_unsorted_edges(
    edges_iterator: Option<
        impl ParallelIterator<Item = Result<(String, String, Option<String>, Option<String>)>>,
    >,
    nodes: Vocabulary<NodeT>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_types: bool,
    has_edge_weights: bool,
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

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    match (edges_iterator, has_edge_types, has_edge_weights) {
        (None, _, _) => {
            // Here likely needs to simply return None
        },
        (Some(ei), true, true) => {
            let mut unsorted_edge_list = ei.map(
                |(src, dst, et, w)| unsafe {
                    (src, dst, et.unwrap_unchecked(), w.unwrap_unchecked())
                }).collect::<Vec<(NodeT, NodeT, EdgeTypeT, WeightT)>>();
            unsorted_edge_list.parse_sort_unstable_by((|(src1, dst1, edt1, _), (src2, dst2, edt2, _)| {
                (*src1, *dst1, *edt1).cmp(&(*src2, *dst2, *edt2))
            });
            // Likely here we need to call a custom core builder.
        },
        (Some(ei), true, false) => {
            let mut unsorted_edge_list =ei.map(|(src, dst, et, _)| unsafe{(src, dst, et.unwrap_unchecked())})
                .collect::<Vec<(NodeT, NodeT, EdgeTypeT)>>();
                unsorted_edge_list.parse_sort_unstable();
        },
        (Some(ei), false, true) => {
            let mut unsorted_edge_list = ei.map(
                |(src, dst, _, w)| unsafe {
                    (src, dst, w.unwrap_unchecked())
                }).collect::<Vec<(NodeT, NodeT, WeightT)>>();
            unsorted_edge_list.parse_sort_unstable_by((|(src1, dst1, _), (src2, dst2, _)| {
                (*src1, *dst1).cmp(&(*src2, *dst2))
            });
            // Likely here we need to call a custom core builder.
        },
        (Some(ei), false, false) => {
            let mut unsorted_edge_list = ei.map(
                |(src, dst, _, _)| unsafe {
                    (src, dst)
                }).collect::<Vec<(NodeT, NodeT)>>();
            unsorted_edge_list.parse_sort_unstable();
            // Likely here we need to call a custom core builder.
        }
    };
}
