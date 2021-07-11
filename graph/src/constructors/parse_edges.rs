use super::*;
use elias_fano_rust::{ConcurrentEliasFanoBuilder, EliasFano};
use rayon::prelude::*;
use std::cmp::Ordering;

macro_rules! parse_unsorted_edge_list {
    (
        $unsorted_edge_list:expr,
        $nodes_number:expr,
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $duplicates:expr
    ) => {{
        // Sorting the vector using a par sort, which is:
        // - unstable because we do not care for changing order of equal values
        // - requires a by because we have weights in the mix.
        $unsorted_edge_list.par_sort_unstable_by(
            |v1, v2| {
                v1.partial_cmp(&v2).unwrap_or(Ordering::Greater)
            },
        );
        // Removes duplicated edges.
        if $duplicates {
            $unsorted_edge_list.dedup_by(|v1, v2| {
                v1.partial_cmp(&v2).unwrap_or(Ordering::Greater) == Ordering::Equal
            });
        }
        // Get the number of nodes and edges.
        let edges_number = $unsorted_edge_list.len();
        // We create the empty vectors for edge types and weights
        $(
            let mut $results = vec![$default; edges_number];
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits($nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge($nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            edges_number as u64,
            maximum_edges_number
        )?;
        // Parsing and building edge list objects
        $unsorted_edge_list
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (src, dst, $($input_tuple),*))| {
                elias_fano_builder.set(i as u64, encode_edge(src, dst, node_bits));
                $(
                    $results[i as usize] = $input_tuple;
                )*
            });
        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;
        // Return the computed values
        (edges, $($results),*)
    }}
}

macro_rules! parse_unsorted_string_edge_list {
    (
        $eis:expr,
        $nodes:expr,
        $node_method:expr,
        $edge_types_vocabulary:expr,
        $edge_types_method:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $directed:expr,
        $complete:expr,
        $duplicates:expr
    ) => {{
        // Create the edge type parser
        let mut edge_type_parser = EdgeTypeParser::new($edge_types_vocabulary);
        // Create the node parser
        let mut node_parser = EdgeNodeNamesParser::new($nodes);
        // Collecting the edges into a mutable vector of tuples
        // collecting exclusively what needs to be collected.
        let mut unsorted_edge_list = $eis.into_iter().flat_map(|ei| {
            // If the provided edge list is either
            // of a directed graph, hence there is no need in the first place to
            // create the edges in the opposite direction, or alternatively
            // the user has specified that the edge list is already complete
            // hence there is no need to create the inverse edges.
            if $directed || $complete {
                ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).map(|line| match line {
                    Ok((_, (src, dst, $($workaround,)*))) => unsafe { Ok((src, dst, $($input_tuple,)*)) },
                    Err(e) => Err(e)
                }).collect::<Vec<Result<_>>>()
            } else {
                ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).flat_map(|line| match line {
                    Ok((_, (src, dst, $($workaround,)*))) => unsafe {
                        if src == dst {
                            vec![Ok((src, dst, $($input_tuple,)*))]
                        } else {
                            vec![
                                Ok((src, dst, $($input_tuple,)*)),
                                Ok((dst, src, $($input_tuple,)*)),
                            ]
                        }
                    },
                    Err(e) => vec![Err(e)]
                })
                .collect::<Vec<Result<_>>>()
            }
        }).collect::<Result<Vec<_>>>()?;
        // Build the actual numeric edge lists
        parse_unsorted_edge_list!(
            unsorted_edge_list,
            $nodes.len(),
            ($($input_tuple),*),
            ($($results),*),
            ($($default),*),
            $duplicates
        )
    }}
}

macro_rules! parse_sorted_string_edge_list {
    (
        $eis:expr,
        $nodes:expr,
        $node_method:expr,
        $edge_types_vocabulary:expr,
        $edge_types_method:expr,
        $edges_number:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
    ) => {{
        // Create the edge type parser
        let mut edge_type_parser = EdgeTypeParser::new($edge_types_vocabulary);
        // Create the node parser
        let mut node_parser = EdgeNodeNamesParser::new($nodes);
        // Creating directly the edge list, as the value are SWORN
        // to be sorted and correct and complete.
        // Get the number of nodes and edges.
        let nodes_number = $nodes.len();
        // Current offset
        let mut offset = 0 as usize;
        // First we create the weights and edge types vectors
        $(
            let mut $results = vec![$default; $edges_number as usize];
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits(nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge(nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            $edges_number as u64,
            maximum_edges_number
        )?;
        $eis.into_iter().for_each(|ei| {
            ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).for_each(|line| {
                // There cannot be results when iterating on a sorted vector.
                let (i, (src, dst, $($workaround),*)) = line.unwrap();
                elias_fano_builder.set((offset + i) as u64, encode_edge(src, dst, node_bits));
                $(
                    $results[offset + i] = $input_tuple;
                )*
            });
            offset = elias_fano_builder.len() as usize;
        });

        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;
        // Return the computed values
        (edges, $($results),*)
    }}
}

macro_rules! parse_unsorted_integer_edge_list {
    (
        $eis:expr,
        $nodes_number:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $directed:expr,
        $complete:expr,
        $duplicates:expr
    ) => {{
        // Collecting the edges into a mutable vector of tuples
        // collecting exclusively what needs to be collected.
        let mut unsorted_edge_list = $eis.into_iter().flat_map(|ei| {
            // If the provided edge list is either
            // of a directed graph, hence there is no need in the first place to
            // create the edges in the opposite direction, or alternatively
            // the user has specified that the edge list is already complete
            // hence there is no need to create the inverse edges.
            if $directed || $complete {
                ei.map(|(_, (src, dst, $($workaround,)*))| (src, dst, $($input_tuple,)*)).collect::<Vec<_>>()
            } else {
                ei.flat_map(|(_, (src, dst, $($workaround,)*))| {
                    if src == dst {
                        vec![(src, dst, $($input_tuple,)*)]
                    } else {
                        vec![
                            (src, dst, $($input_tuple,)*),
                            (dst, src, $($input_tuple,)*),
                        ]
                    }
                })
                .collect::<Vec<_>>()
            }
        }).collect::<Vec<_>>();
        // Build the actual numeric edge lists
        parse_unsorted_edge_list!(
            unsorted_edge_list,
            $nodes_number,
            ($($input_tuple),*),
            ($($results),*),
            ($($default),*),
            $duplicates
        )
    }}
}

macro_rules! parse_sorted_integer_edge_list {
    (
        $eis:expr,
        $nodes_number:expr,
        $edges_number:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
    ) => {{
        // Current offset
        let mut offset = 0 as usize;
        // First we create the weights and edge types vectors
        $(
            let mut $results = vec![$default; $edges_number as usize];
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits($nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge($nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            $edges_number as u64,
            maximum_edges_number
        )?;
        $eis.into_iter().for_each(|ei| {
            ei.for_each(|(i, (src, dst, $($workaround),*))| {
                elias_fano_builder.set((offset + i) as u64, encode_edge(src, dst, node_bits));
                $(
                    $results[offset + i] = $input_tuple;
                )*
            });
            offset = elias_fano_builder.len() as usize;
        });

        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;
        // Return the computed values
        (edges, $($results),*)
    }}
}

fn check_general_edge_constructor_parameters_consistency<I>(
    sorted: bool,
    has_edge_types: bool,
    complete: bool,
    correct: bool,
    edges_number: Option<EdgeT>,
    edges_iterators: Option<Vec<I>>,
) -> Result<()> {
    if sorted && edges_number.is_none() {
        return Err(concat!(
            "It is not possible to build a sorted edge list ",
            "without knowing at least a rough estimate of the ",
            "number of edges in the edge list.\n",
            "This estimate must be at least within a ",
            "binary exponentiation range, that is between ",
            "2^{n} and 2^{n+1}."
        )
        .to_string());
    }

    if sorted && !complete {
        return Err(concat!(
            "It is not possible to build a sorted edge list ",
            "if it is not provided as complete."
        )
        .to_string());
    }

    if sorted && !correct {
        return Err(concat!(
            "It is not possible to build a sorted edge list ",
            "if it is not provided as correct, that is ",
            "without any sort of error."
        )
        .to_string());
    }

    if edges_iterators.as_ref().map_or(true, |ei| ei.is_empty()) && has_edge_types {
        return Err(concat!(
            "Edge types vocabulary was provided ",
            "but no edge list was given."
        )
        .to_string());
    }
    Ok(())
}

// TODO! trovare un nome
pub(crate) fn parse_string_edges(
    edges_iterators: Option<
        Vec<
            impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,
        >,
    >,
    nodes: Vocabulary<NodeT>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_types: bool,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    edges_number: Option<EdgeT>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
) -> Result<(
    Vocabulary<NodeT>,
    EliasFano,
    Option<EdgeTypeVocabulary>,
    Option<Vec<WeightT>>,
)> {
    let correct = correct.unwrap_or(false);
    let complete = complete.unwrap_or(directed);
    let duplicates = duplicates.unwrap_or(true);
    let sorted = sorted.unwrap_or(false);
    let numeric_edge_list_node_ids = numeric_edge_list_node_ids.unwrap_or(false);
    let numeric_edge_list_edge_type_ids = numeric_edge_list_edge_type_ids.unwrap_or(false);
    let has_edge_types = edge_types_vocabulary.is_some();

    check_general_edge_constructor_parameters_consistency(
        sorted,
        has_edge_types,
        complete,
        correct,
        edges_number,
        edges_iterators,
    )?;

    if !has_edge_types && numeric_edge_list_edge_type_ids {
        return Err(concat!(
            "The numeric node list node type IDs parameter does not make sense ",
            "in the context where the node types have not been provided.\n",
            "If the node types within the nodes list are numeric, simply use ",
            "the numeric node types ids parameter."
        )
        .to_string());
    }

    let edge_types_method = match (
        has_edge_types,
        edge_types_vocabulary
            .as_ref()
            .map_or(true, |x| x.is_empty()),
        correct,
        numeric_edge_list_edge_type_ids,
    ) {
        (false, _, _, false) => EdgeTypeParser::ignore,
        (true, true, true, false) => EdgeTypeParser::parse_strings_unchecked,
        (true, true, false, false) => EdgeTypeParser::parse_strings,
        (true, false, true, false) => EdgeTypeParser::get_unchecked,
        (true, false, false, false) => EdgeTypeParser::get,
        (_, _, true, true) => EdgeTypeParser::to_numeric_unchecked,
        (_, _, false, true) => EdgeTypeParser::to_numeric,
    };
    let node_method = match (nodes.is_empty(), correct, numeric_edge_list_node_ids) {
        (true, true, false) => EdgeNodeNamesParser::parse_strings_unchecked,
        (true, false, false) => EdgeNodeNamesParser::parse_strings,
        (false, true, false) => EdgeNodeNamesParser::get_unchecked,
        (false, false, false) => EdgeNodeNamesParser::get,
        (_, true, true) => EdgeNodeNamesParser::to_numeric_unchecked,
        (_, false, true) => EdgeNodeNamesParser::to_numeric,
    };

    let mut edge_types_vocabulary = edge_types_vocabulary.unwrap_or(Vocabulary::new());

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    let (edges, edge_type_ids, weights) =
        match (edges_iterators, sorted, has_edge_types, has_edge_weights) {
            (None, _, _, _) => (EliasFano::new(0, 0)?, None, None),
            // When the edge lists are provided and are:
            // - Sorted
            // - Completely defined in both directions
            // - Sworn on the tomb of Von Neumann to be a correct edge list
            (Some(eis), true, true, true) => {
                let (edges, edge_type_ids, weights) = parse_sorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                );
                // Return the computed values
                (edges, Some(edge_type_ids), Some(weights))
            }
            (Some(eis), true, false, true) => {
                let (edges, weights) = parse_sorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                );
                // Return the computed values
                (edges, None, Some(weights))
            }
            (Some(eis), true, true, false) => {
                let (edges, edge_type_ids) = parse_sorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (edge_type),
                    (edge_types),
                    (None),
                );
                // Return the computed values
                (edges, Some(edge_type_ids), None)
            }
            (Some(eis), true, false, false) => {
                let (edges,) = parse_sorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (),
                    (),
                    (),
                );
                // Return the computed values
                (edges, None, None)
            }
            (Some(eis), false, true, true) => {
                // Building the edge list
                let (edges, edge_type_ids, weights) = parse_unsorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, Some(edge_type_ids), Some(weights))
            }
            (Some(eis), false, true, false) => {
                // Building the edge list
                let (edges, edge_type_ids) = parse_unsorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    (edge_type, weight),
                    (edge_type),
                    (edge_types),
                    (None),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, Some(edge_type_ids), None)
            }
            (Some(eis), false, false, true) => {
                // Building the edge list
                let (edges, weights) = parse_unsorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    (edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, None, Some(weights))
            }
            (Some(eis), false, false, false) => {
                // Building the edge list
                let (edges,) = parse_unsorted_string_edge_list!(
                    eis,
                    nodes,
                    node_method,
                    edge_types_vocabulary,
                    edge_types_method,
                    (edge_type, weight),
                    (),
                    (),
                    (),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, None, None)
            }
        };

    Ok((
        nodes,
        edges,
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, Some(edge_types_vocabulary)),
        weights,
    ))
}

// TODO! trovare un nome
pub(crate) fn parse_integer_edges(
    edges_iterators: Option<
        Vec<impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>>,
    >,
    nodes_number: NodeT,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_weights: bool,
    directed: bool,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    edges_number: Option<EdgeT>,
) -> Result<(EliasFano, Option<EdgeTypeVocabulary>, Option<Vec<WeightT>>)> {
    let complete = complete.unwrap_or(directed);
    let duplicates = duplicates.unwrap_or(true);
    let sorted = sorted.unwrap_or(false);
    let has_edge_types = edge_types_vocabulary.is_some();

    check_general_edge_constructor_parameters_consistency(
        sorted,
        has_edge_types,
        complete,
        true,
        edges_number,
        edges_iterators,
    )?;

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    let (edges, edge_type_ids, weights) =
        match (edges_iterators, sorted, has_edge_types, has_edge_weights) {
            (None, _, _, _) => (EliasFano::new(0, 0)?, None, None),
            // When the edge lists are provided and are:
            // - Sorted
            // - Completely defined in both directions
            // - Sworn on the tomb of Von Neumann to be a correct edge list
            (Some(eis), true, true, true) => {
                let (edges, edge_type_ids, weights) = parse_sorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                );
                // Return the computed values
                (edges, Some(edge_type_ids), Some(weights))
            }
            (Some(eis), true, false, true) => {
                let (edges, weights) = parse_sorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                );
                // Return the computed values
                (edges, None, Some(weights))
            }
            (Some(eis), true, true, false) => {
                let (edges, edge_type_ids) = parse_sorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (edge_type),
                    (edge_types),
                    (None),
                );
                // Return the computed values
                (edges, Some(edge_type_ids), None)
            }
            (Some(eis), true, false, false) => {
                let (edges,) = parse_sorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (),
                    (),
                    (),
                );
                // Return the computed values
                (edges, None, None)
            }
            (Some(eis), false, true, true) => {
                // Building the edge list
                let (edges, edge_type_ids, weights) = parse_unsorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, Some(edge_type_ids), Some(weights))
            }
            (Some(eis), false, true, false) => {
                // Building the edge list
                let (edges, edge_type_ids) = parse_unsorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    (edge_type, weight),
                    (edge_type),
                    (edge_types),
                    (None),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, Some(edge_type_ids), None)
            }
            (Some(eis), false, false, true) => {
                // Building the edge list
                let (edges, weights) = parse_unsorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    (edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, None, Some(weights))
            }
            (Some(eis), false, false, false) => {
                // Building the edge list
                let (edges,) = parse_unsorted_integer_edge_list!(
                    eis,
                    nodes_number,
                    (edge_type, weight),
                    (),
                    (),
                    (),
                    directed,
                    complete,
                    duplicates
                );
                // Return the computed values
                (edges, None, None)
            }
        };

    Ok((
        edges,
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, edge_types_vocabulary),
        weights,
    ))
}
