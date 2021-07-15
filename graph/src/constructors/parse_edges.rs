use super::*;
use elias_fano_rust::{ConcurrentEliasFanoBuilder, EliasFano};
use num_traits::Zero;
use rayon::prelude::*;
use std::cmp::Ordering;

macro_rules! parse_unsorted_edge_list {
    (
        $unsorted_edge_list:expr,
        $nodes_number:expr,
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $duplicates:expr,
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
            let $results = ThreadDataRaceAware {
                value: std::cell::UnsafeCell::new(vec![$default; edges_number]),
            };
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
            .for_each(|(i, (src, dst, $($input_tuple),*))|  {
                elias_fano_builder.set(i as u64, encode_edge(src, dst, node_bits));
                $(
                    unsafe{(*$results.value.get())[i] = $input_tuple};
                )*
            });
        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;
        // Return the computed values
        (
            edges,
            $(
                $results.value.into_inner()
            ),*
        )
    }}
}

macro_rules! parse_unsorted_string_edge_list {
    (
        $ei:expr,
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
        $duplicates:expr,
    ) => {{
        // Create the edge type parser
        let mut edge_type_parser = EdgeTypeParser::new($edge_types_vocabulary);
        // Create the node parser
        let mut node_parser = EdgeNodeNamesParser::new($nodes);
        // If the provided edge list is either
        // of a directed graph, hence there is no need in the first place to
        // create the edges in the opposite direction, or alternatively
        // the user has specified that the edge list is already complete
        // hence there is no need to create the inverse edges.
        let mut unsorted_edge_list = if $directed || $complete {
            $ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).map(|line| match line {
                Ok((_, (src, dst, $($workaround,)*))) => { Ok((src, dst, $($input_tuple,)*)) },
                Err(e) => Err(e)
            }).collect::<Result<Vec<_>>>()
        } else {
            $ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).flat_map(|line| match line {
                Ok((_, (src, dst, $($workaround,)*))) => {
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
            .collect::<Result<Vec<_>>>()
        }?;
        // Assigning to variable as patch
        let duplicates = $duplicates;
        // Recovering the nodes
        let nodes = node_parser.into_inner();
        // Build the actual numeric edge lists
        let (
            edges,
            $(
                $results
            ),*
        ) = parse_unsorted_edge_list!(
            unsorted_edge_list,
            nodes.len(),
            ($($input_tuple),*),
            ($($results),*),
            ($($default),*),
            duplicates,
        );

        let mut edge_types_vocabulary = edge_type_parser.into_inner();
        if edge_types_vocabulary.is_empty() {
            edge_types_vocabulary.build()?;
        }

        (
            edges,
            nodes,
            edge_types_vocabulary,
            $(
                $results
            ),*
        )
    }}
}

macro_rules! parse_sorted_string_edge_list {
    (
        $ei:expr,
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
        // Creating directly the edge list, as the value are SWORN
        // to be sorted and correct and complete.
        // Get the number of nodes and edges.
        let nodes_number = $nodes.len();
        // Create the node parser
        let mut node_parser = EdgeNodeNamesParser::new($nodes);
        // First we create the weights and edge types vectors
        $(
            let $results = ThreadDataRaceAware {
                value: std::cell::UnsafeCell::new(vec![$default; $edges_number as usize]),
            };
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits(nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge(nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            $edges_number as u64,
            maximum_edges_number
        )?;
        $ei.method_caller($edge_types_method, &mut edge_type_parser).method_caller($node_method, &mut node_parser).for_each(|line| {
            // There cannot be results when iterating on a sorted vector.
            let (i, (src, dst, $($workaround),*)) = line.unwrap();
            elias_fano_builder.set(i as u64, encode_edge(src, dst, node_bits));
            $(
                unsafe{(*$results.value.get())[i] = $input_tuple};
            )*
        });

        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;

        let mut edge_types_vocabulary = edge_type_parser.into_inner();
        if edge_types_vocabulary.is_empty() {
            edge_types_vocabulary.build()?;
        }
        // Return the computed values
        (
            edges,
            node_parser.into_inner(),
            edge_types_vocabulary,
            $(
                $results.value.into_inner()
            ),*
        )
    }}
}

macro_rules! parse_unsorted_integer_edge_list {
    (
        $ei:expr,
        $nodes_number:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $directed:expr,
        $complete:expr,
        $duplicates:expr,
    ) => {{
        // If the provided edge list is either
        // of a directed graph, hence there is no need in the first place to
        // create the edges in the opposite direction, or alternatively
        // the user has specified that the edge list is already complete
        // hence there is no need to create the inverse edges.
        let mut unsorted_edge_list = if $directed || $complete {
            $ei.map(|(_, (src, dst, $($workaround,)*))| (src, dst, $($input_tuple,)*)).collect::<Vec<_>>()
        } else {
            $ei.flat_map(|(_, (src, dst, $($workaround,)*))| {
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
        };
        // Assigning to variable as patch
        let duplicates = $duplicates;
        // Build the actual numeric edge lists
        parse_unsorted_edge_list!(
            unsorted_edge_list,
            $nodes_number,
            ($($input_tuple),*),
            ($($results),*),
            ($($default),*),
            duplicates,
        )
    }}
}

macro_rules! parse_sorted_integer_edge_list {
    (
        $ei:expr,
        $nodes_number:expr,
        $edges_number:expr,
        ($($workaround:ident),*),
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
    ) => {{
        // First we create the weights and edge types vectors
        $(
            let $results = ThreadDataRaceAware {
                value: std::cell::UnsafeCell::new(vec![$default; $edges_number as usize]),
            };
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits($nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge($nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            $edges_number as u64,
            maximum_edges_number
        )?;
        $ei.for_each(|(i, (src, dst, $($workaround),*))| {
            elias_fano_builder.set(i as u64, encode_edge(src, dst, node_bits));
            $(
                unsafe{(*$results.value.get())[i] = $input_tuple};
            )*
        });

        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build()?;
        // Return the computed values
        (
            edges,
            $(
                $results.value.into_inner()
            ),*
        )
    }}
}

fn check_general_edge_constructor_parameters_consistency<I>(
    sorted: bool,
    has_edge_types: bool,
    complete: bool,
    correct: bool,
    edges_number: Option<EdgeT>,
    nodes_number: NodeT,
    edges_iterator: &Option<I>,
) -> Result<()> {
    if sorted && edges_number.is_none() {
        return Err(concat!(
            "It is not possible to build a sorted edge list ",
            "without knowing the ",
            "number of edges in the edge list.",
        )
        .to_string());
    }

    if nodes_number.is_zero()
        && edges_number
            .as_ref()
            .map_or(false, |&edges_number| edges_number > 0)
    {
        return Err(format!(
            concat!(
                "This graph was parametrized in an impossible way: ",
                "a non zero number of edges {:?} and a zero nodes {:?}."
            ),
            edges_number, nodes_number
        ));
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

    if edges_iterator.is_none() && has_edge_types {
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
    edges_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,
    >,
    nodes: Vocabulary<NodeT>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
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
        nodes.len() as NodeT,
        &edges_iterator,
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

    let edge_types_vocabulary = edge_types_vocabulary.unwrap_or(Vocabulary::new());

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    let (edges, nodes, edge_types_vocabulary, edge_type_ids, weights) = match (
        edges_iterator,
        sorted,
        has_edge_types,
        has_edge_weights,
    ) {
        (None, _, _, _) => (
            EliasFano::new(0, 0)?,
            nodes,
            edge_types_vocabulary,
            None,
            None,
        ),
        // When the edge lists are provided and are:
        // - Sorted
        // - Completely defined in both directions
        // - Sworn on the tomb of Von Neumann to be a correct edge list
        (Some(ei), true, true, true) => {
            let (edges, nodes, edge_types_vocabulary, edge_type_ids, weights) = parse_sorted_string_edge_list!(
                ei,
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
            (
                edges,
                nodes,
                edge_types_vocabulary,
                optionify!(edge_type_ids),
                optionify!(weights),
            )
        }
        (Some(ei), true, false, true) => {
            let (edges, nodes, edge_types_vocabulary, weights) = parse_sorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                edges_number.unwrap(),
                (_edge_type, weight),
                (weight),
                (weights),
                (WeightT::NAN),
            );
            // Return the computed values
            (
                edges,
                nodes,
                edge_types_vocabulary,
                None,
                optionify!(weights),
            )
        }
        (Some(ei), true, true, false) => {
            let (edges, nodes, edge_types_vocabulary, edge_type_ids) = parse_sorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                edges_number.unwrap(),
                (edge_type, _weight),
                (edge_type),
                (edge_types),
                (None),
            );
            // Return the computed values
            (
                edges,
                nodes,
                edge_types_vocabulary,
                optionify!(edge_type_ids),
                None,
            )
        }
        (Some(ei), true, false, false) => {
            let (edges, nodes, edge_types_vocabulary) = parse_sorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                edges_number.unwrap(),
                (_edge_type, _weight),
                (),
                (),
                (),
            );
            // Return the computed values
            (edges, nodes, edge_types_vocabulary, None, None)
        }
        (Some(ei), false, true, true) => {
            // Building the edge list
            let (edges, nodes, edge_types_vocabulary, edge_type_ids, weights) = parse_unsorted_string_edge_list!(
                ei,
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
                duplicates,
            );
            // Return the computed values
            (
                edges,
                nodes,
                edge_types_vocabulary,
                optionify!(edge_type_ids),
                optionify!(weights),
            )
        }
        (Some(ei), false, true, false) => {
            // Building the edge list
            let (edges, nodes, edge_types_vocabulary, edge_type_ids) = parse_unsorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (edge_type, _weight),
                (edge_type),
                (edge_types),
                (None),
                directed,
                complete,
                duplicates,
            );
            // Return the computed values
            (
                edges,
                nodes,
                edge_types_vocabulary,
                optionify!(edge_type_ids),
                None,
            )
        }
        (Some(ei), false, false, true) => {
            // Building the edge list
            let (edges, nodes, edge_types_vocabulary, weights) = parse_unsorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (_edge_type, weight),
                (weight),
                (weights),
                (WeightT::NAN),
                directed,
                complete,
                duplicates,
            );
            // Return the computed values
            (
                edges,
                nodes,
                edge_types_vocabulary,
                None,
                optionify!(weights),
            )
        }
        (Some(ei), false, false, false) => {
            // Building the edge list
            let (edges, nodes, edge_types_vocabulary) = parse_unsorted_string_edge_list!(
                ei,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (_edge_type, _weight),
                (),
                (),
                (),
                directed,
                complete,
                duplicates,
            );
            // Return the computed values
            (edges, nodes, edge_types_vocabulary, None, None)
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
    edges_iterator: Option<
        impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
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
        nodes_number,
        &edges_iterator,
    )?;

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    let (edges, edge_type_ids, weights) =
        match (edges_iterator, sorted, has_edge_types, has_edge_weights) {
            (None, _, _, _) => (EliasFano::new(0, 0)?, None, None),
            // When the edge lists are provided and are:
            // - Sorted
            // - Completely defined in both directions
            // - Sworn on the tomb of Von Neumann to be a correct edge list
            (Some(ei), true, true, true) => {
                let (edges, edge_type_ids, weights) = parse_sorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                );
                // Return the computed values
                (edges, optionify!(edge_type_ids), optionify!(weights))
            }
            (Some(ei), true, false, true) => {
                let (edges, weights) = parse_sorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    edges_number.unwrap(),
                    (_edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                );
                // Return the computed values
                (edges, None, optionify!(weights))
            }
            (Some(ei), true, true, false) => {
                let (edges, edge_type_ids) = parse_sorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    edges_number.unwrap(),
                    (edge_type, _weight),
                    (edge_type),
                    (edge_types),
                    (None),
                );
                // Return the computed values
                (edges, optionify!(edge_type_ids), None)
            }
            (Some(ei), true, false, false) => {
                let (edges,) = parse_sorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    edges_number.unwrap(),
                    (_edge_type, _weight),
                    (),
                    (),
                    (),
                );
                // Return the computed values
                (edges, None, None)
            }
            (Some(ei), false, true, true) => {
                // Building the edge list
                let (edges, edge_type_ids, weights) = parse_unsorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    (edge_type, weight),
                    (edge_type, weight),
                    (edge_types, weights),
                    (None, WeightT::NAN),
                    directed,
                    complete,
                    duplicates,
                );
                // Return the computed values
                (edges, optionify!(edge_type_ids), optionify!(weights))
            }
            (Some(ei), false, true, false) => {
                // Building the edge list
                let (edges, edge_type_ids) = parse_unsorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    (edge_type, _weight),
                    (edge_type),
                    (edge_types),
                    (None),
                    directed,
                    complete,
                    duplicates,
                );
                // Return the computed values
                (edges, optionify!(edge_type_ids), None)
            }
            (Some(ei), false, false, true) => {
                // Building the edge list
                let (edges, weights) = parse_unsorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    (_edge_type, weight),
                    (weight),
                    (weights),
                    (WeightT::NAN),
                    directed,
                    complete,
                    duplicates,
                );
                // Return the computed values
                (edges, None, optionify!(weights))
            }
            (Some(ei), false, false, false) => {
                // Building the edge list
                let (edges,) = parse_unsorted_integer_edge_list!(
                    ei,
                    nodes_number,
                    (_edge_type, _weight),
                    (),
                    (),
                    (),
                    directed,
                    complete,
                    duplicates,
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
