use super::*;
use crate::hashes::*;
use crate::isomorphism_iter::EqualBucketsParIter;
use core::ops::BitOr;
use itertools::Itertools;
use num_traits::{AsPrimitive, WrappingShl};
use rayon::prelude::*;

/// The `WordInteger` trait represents a word-sized integer type that satisfies various constraints.
/// Types implementing this trait must be `Send`, `Sync`, `Eq`, `Copy`, `Ord`, and support the `BitOr` and `WrappingShl` operations.
/// Additionally, the trait has an associated constant `ONE` representing the value of one for the implementing type.
/// The `WordInteger` trait is meant to be used for word-sized integer types in a generic context.
trait WordInteger:
    Send + Sync + Eq + Copy + Ord + BitOr<Self, Output = Self> + WrappingShl + 'static
{
    /// The value representing one for the implementing type.
    const ONE: Self;
}

/// Implementation of the `WordInteger` trait for the `u8` type.
/// The `u8` type is a word-sized integer with a size of 8 bits.
impl WordInteger for u8 {
    /// The value representing one for the `u8` type is 1.
    const ONE: Self = 1_u8;
}

/// Implementation of the `WordInteger` trait for the `u16` type.
/// The `u16` type is a word-sized integer with a size of 16 bits.
impl WordInteger for u16 {
    /// The value representing one for the `u16` type is 1.
    const ONE: Self = 1_u16;
}

/// Implementation of the `WordInteger` trait for the `u32` type.
/// The `u32` type is a word-sized integer with a size of 32 bits.
impl WordInteger for u32 {
    /// The value representing one for the `u32` type is 1.
    const ONE: Self = 1_u32;
}

/// Implementation of the `WordInteger` trait for the `u64` type.
/// The `u64` type is a word-sized integer with a size of 64 bits.
impl WordInteger for u64 {
    /// The value representing one for the `u64` type is 1.
    const ONE: Self = 1_64;
}

/// The `ToNodeNames` trait represents a wrapper type that can be converted to isomorphism node names.
/// Types implementing this trait provide a method `to_node_names` that takes a reference to a `Graph`
/// and returns the isomorphism node names.
/// The `ToNodeNames` trait is meant to be used when a wrapper type needs to be converted into isomorphism node names.
trait ToNodeNames<IsomorphismNames> {
    /// Converts the wrapper type into isomorphism node names using the given `Graph`.
    ///
    /// # Arguments
    /// * `graph`: A reference to the `Graph` used for converting the wrapper type into isomorphism node names.
    ///
    /// # Returns
    /// The isomorphism node names obtained from the wrapper type.
    ///
    fn to_node_names(&self, graph: &Graph) -> IsomorphismNames;
}

/// The `SelfloopExcludedGroupNodeDegree` trait provised methods to,
/// the self-loop excluded node degree, check if a group contains a self-loops, and determine if
/// it intersects with another group.
pub trait SelfloopExcludedGroupNodeDegree {
    /// Retrieves the node degree of the group with self-loops excluded using the given `Graph`.
    ///
    /// # Arguments
    /// * `graph`: A reference to the `Graph` used for calculating the node degree.
    ///
    /// # Returns
    /// The node degree of the group with self-loops excluded.
    fn get_selfloop_excluded_group_node_degree(&self, graph: &Graph) -> NodeT;

    /// Checks if the given node ID represents a self-loop within the group.
    ///
    /// # Arguments
    /// * `node_id`: A reference to the node ID to check.
    ///
    /// # Returns
    /// `true` if the node ID represents a self-loop within the group, otherwise `false`.
    fn is_selfloop(&self, node_id: &NodeT) -> bool;

    /// Determines if the current group intersects with another group.
    ///
    /// # Arguments
    /// * `other`: A reference to another group to check for intersection.
    ///
    /// # Returns
    /// `true` if the current group intersects with the other group, otherwise `false`.
    fn intersects(&self, other: &Self) -> bool;
}

/// The `IsomorphicCandidateGenerator` trait represents a generator of isomorphic candidates,
/// providing a method to parallel iterate over isomorphic candidates.
/// The generic parameter `W` represents the type of isomorphic candidate.
trait IsomorphicCandidateGenerator<Candidate>
where
    Candidate: Send + Sync,
{
    /// Parallel iterator over the isomorphic candidates.
    ///
    /// # Arguments
    /// * `graph`: &Graph - A reference to the `Graph` used for generating isomorphic candidates.
    /// * `minimum_node_degree`: NodeT - The minimum degree that a node must have to be included in the isomorphic candidates.
    ///
    /// # Returns
    /// A parallel iterator that yields tuples representing isomorphic candidates, where the first element of the tuple
    /// is of type `Word` (a generic type representing an integer), and the second element is of type `Candidate` (the isomorphic candidate type).
    ///
    fn par_iter_isomorphic_candidates<'a, Word: WordInteger>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (Word, Candidate)> + 'a
    where
        u64: AsPrimitive<Word>;
}
/// The `NodeIsomorphismsGenerator` struct represents a generator for node isomorphisms.
/// It is used to generate isomorphisms related to nodes in a graph.
#[no_binding]
#[derive(Default)]
struct NodeIsomorphismsGenerator;

/// The `EdgeIsomorphismsGenerator` struct represents a generator for edge isomorphisms.
/// It is used to generate isomorphisms related to edges in a graph.
#[no_binding]
#[derive(Default)]
struct EdgeIsomorphismsGenerator;

/// The `TupleIsomorphismsGenerator` struct represents a generator for tuple isomorphisms.
/// It is used to generate isomorphisms related to tuples.
#[no_binding]
#[derive(Default)]
struct TupleIsomorphismsGenerator;

impl IsomorphicCandidateGenerator<NodeT> for NodeIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a, Word: WordInteger>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (Word, NodeT)> + 'a
    where
        u64: AsPrimitive<Word>,
    {
        graph
            .par_iter_node_degrees()
            .enumerate()
            .filter(move |(_, node_degree)| *node_degree >= minimum_node_degree)
            .map(move |(node_id, _)| {
                let mut hasher = Hasher::commutative_simple();
                hasher.update(&unsafe {
                    graph.get_unchecked_node_type_ids_from_node_id(node_id as NodeT)
                });
                (hasher.digest().as_(), node_id as NodeT)
            })
    }
}

impl IsomorphicCandidateGenerator<[NodeT; 2]> for EdgeIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a, Word: WordInteger>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (Word, [NodeT; 2])> + 'a
    where
        u64: AsPrimitive<Word>,
    {
        graph
            .par_iter_node_ids()
            .zip(graph.par_iter_node_degrees())
            .filter(move |(_, node_degree)| *node_degree > minimum_node_degree)
            .flat_map(move |(src, _src_node_degree)| {
                let (min_edge_id, max_edge_id) =
                    unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(src) };
                let min_edge_id = min_edge_id as usize;
                let max_edge_id = max_edge_id as usize;
                let src_edge_type_ids = graph
                    .edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| &ets.ids[min_edge_id..max_edge_id]);
                let mut first_hasher = Hasher::commutative_simple();
                first_hasher.update(&unsafe {
                    graph.get_unchecked_node_type_ids_from_node_id(src as NodeT)
                });
                unsafe { graph.par_iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .enumerate()
                    .filter(move |(_i, dst)| {
                        src != *dst
                            && (graph.is_directed() || src < *dst)
                            && unsafe { graph.get_unchecked_node_degree_from_node_id(*dst) }
                                > minimum_node_degree
                    })
                    .map(move |(i, dst)| {
                        let mut second_hasher = first_hasher.clone();
                        second_hasher.update(&unsafe {
                            graph.get_unchecked_node_type_ids_from_node_id(dst as NodeT)
                        });
                        second_hasher.update(&src_edge_type_ids.as_ref().and_then(|ids| ids[i]));
                        (second_hasher.digest().as_(), [src, dst].into())
                    })
            })
    }
}

impl IsomorphicCandidateGenerator<[NodeT; 2]> for TupleIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a, Word: WordInteger>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (Word, [NodeT; 2])> + 'a
    where
        u64: AsPrimitive<Word>,
    {
        graph
            .par_iter_node_ids()
            .zip(graph.par_iter_node_degrees())
            .filter(move |(_, node_degree)| *node_degree > minimum_node_degree)
            .flat_map(move |(src, _src_node_degree)| {
                let mut first_hasher = Hasher::commutative_simple();
                first_hasher.update(&unsafe {
                    graph.get_unchecked_node_type_ids_from_node_id(src as NodeT)
                });
                graph
                    .par_iter_node_ids()
                    .zip(graph.par_iter_node_degrees())
                    .filter(move |(dst, node_degree)| {
                        src != *dst
                            && (graph.is_directed() || src < *dst)
                            && *node_degree > minimum_node_degree
                    })
                    .map(move |(dst, _)| {
                        let mut second_hasher = first_hasher.clone();
                        second_hasher.update(&unsafe {
                            graph.get_unchecked_node_type_ids_from_node_id(dst as NodeT)
                        });
                        (
                            second_hasher.digest().as_(),
                            if src < dst { [src, dst] } else { [dst, src] },
                        )
                    })
            })
    }
}

/// The `IterNeighbours` trait represents a set of methods for iterating over nodes and neighbors in a graph.
pub trait IterNeighbours {
    /// Returns an iterator over the nodes.
    ///
    /// # Returns
    /// An iterator that yields `NodeT` values representing the nodes in the graph.
    fn iter_nodes(&self) -> impl Iterator<Item = NodeT> + '_;

    /// Returns an iterator over the neighbors of the current element in the graph.
    ///
    /// # Arguments
    /// * `graph`: A reference to the `Graph` from which the neighbors are iterated.
    ///
    /// # Returns
    /// An iterator that yields `NodeT` values representing the neighbors of the current element.
    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a;

    /// Returns an iterator over the neighbors and their corresponding edge IDs of the current element in the graph.
    ///
    /// # Arguments
    /// * `graph`: A reference to the `Graph` from which the neighbors and edge IDs are iterated.
    ///
    /// # Returns
    /// An iterator that yields tuples consisting of `NodeT` values representing the neighbors and `EdgeT` values representing their corresponding edge IDs.
    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a;

    /// Returns an iterator over the neighbors of the current element in the graph, excluding self-loops.
    ///
    /// # Arguments
    /// * `graph`: A reference to the `Graph` from which the neighbors are iterated.
    ///
    /// # Returns
    /// An iterator that yields `NodeT` values representing the neighbors of the current element, excluding self-loops.
    fn iter_selfloop_excluded_neighbours<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = NodeT> + 'a {
        iter_set::difference(self.iter_neighbours(&graph), self.iter_nodes())
    }
}

impl IterNeighbours for u32 {
    fn iter_nodes(&self) -> impl Iterator<Item = NodeT> + '_ {
        core::iter::once(*self as NodeT)
    }

    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a {
        unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(*self as NodeT) }
    }

    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a {
        let (min_edge_id, max_edge_id) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(*self as NodeT) };
        unsafe {
            graph
                .iter_unchecked_neighbour_node_ids_from_source_node_id(*self as NodeT)
                .zip((min_edge_id..max_edge_id).into_iter())
        }
    }
}

impl SelfloopExcludedGroupNodeDegree for u32 {
    fn get_selfloop_excluded_group_node_degree(&self, graph: &Graph) -> u32 {
        unsafe { graph.get_unchecked_selfloop_excluded_node_degree_from_node_id(*self as NodeT) }
    }

    fn is_selfloop(&self, node_id: &NodeT) -> bool {
        self == node_id
    }

    fn intersects(&self, other: &Self) -> bool {
        self == other
    }
}

impl<const N: usize> SelfloopExcludedGroupNodeDegree for [NodeT; N] {
    fn get_selfloop_excluded_group_node_degree(&self, graph: &Graph) -> u32 {
        self.iter()
            .map(|node_id| unsafe {
                graph.get_unchecked_selfloop_excluded_node_degree_from_node_id(*node_id)
            })
            .sum::<u32>()
            - (self.len() * (self.len() - 1)) as u32
    }

    fn is_selfloop(&self, node_id: &NodeT) -> bool {
        self.contains(node_id)
    }

    fn intersects(&self, other: &Self) -> bool {
        iter_set::intersection(self.iter(), other.iter()).count() == 0
    }
}

impl ToNodeNames<String> for u32 {
    fn to_node_names(&self, graph: &Graph) -> String {
        unsafe { graph.get_unchecked_node_name_from_node_id(*self) }
    }
}

impl ToNodeNames<[String; 2]> for [NodeT; 2] {
    fn to_node_names(&self, graph: &Graph) -> [String; 2] {
        [
            unsafe { graph.get_unchecked_node_name_from_node_id(self[0]) },
            unsafe { graph.get_unchecked_node_name_from_node_id(self[1]) },
        ]
    }
}

impl IterNeighbours for [NodeT; 2] {
    fn iter_nodes(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter().copied()
    }

    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a {
        iter_set::union(
            unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self[0]) },
            unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self[1]) },
        )
    }

    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a {
        let (first_min_edge, first_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self[0]) };
        let (second_min_edge, second_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self[1]) };
        iter_set::union_by(
            unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(self[0])
                    .zip((first_min_edge..first_max_edge).into_iter())
            },
            unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(self[1])
                    .zip((second_min_edge..second_max_edge).into_iter())
            },
            |(first, _), (second, _)| first.cmp(&second),
        )
    }
}

impl Graph {
    /// Computes a hash value based on a set of node IDs, excluding self-loops, and other parameters.
    ///
    /// # Safety
    /// This function is marked as `unsafe` because it assumes that the provided `node_ids` are valid and that the associated graph is properly constructed.
    ///
    /// # Arguments
    /// * `node_ids`: A reference to an object implementing the `Isomorphism` trait that provides the node IDs for computing the hash value.
    /// * `minimum_node_degree`: The minimum degree that a node must have for it to be considered in the hash computation.
    /// * `number_of_neighbours_for_hash`: The number of neighbor nodes to consider for the hash computation.
    /// * `seed`: The initial seed value for the hash computation.
    ///
    /// # Returns
    /// The computed hash value based on the node IDs, excluding self-loops, and other parameters.
    ///
    /// # Type Parameters
    /// * `Word`: A generic type implementing the `WordInteger` trait, representing the type of the hash value.
    /// * `Isomorphism`: A generic type implementing both the `IterNeighbours` and `SelfloopExcludedGroupNodeDegree` traits, providing the necessary functionality for iterating over neighbors and excluding self-loops.
    unsafe fn get_hash_from_node_ids<Word: WordInteger, Isomorphism>(
        &self,
        node_ids: &Isomorphism,
        minimum_node_degree: NodeT,
        number_of_neighbours_for_hash: usize,
        seed: Word,
    ) -> Word
    where
        u64: AsPrimitive<Word>,
        Isomorphism: IterNeighbours + SelfloopExcludedGroupNodeDegree,
    {
        // Get an iterator over the neighbors of the provided `node_ids`, excluding self-loops.
        let neighbors = node_ids.iter_selfloop_excluded_neighbours(&self);

        // Take only the specified number of neighbor nodes.
        let selected_neighbors = neighbors.take(number_of_neighbours_for_hash);

        // Filter out neighbor nodes whose degree is less than the minimum node degree.
        let filtered_neighbors = selected_neighbors.filter(|node_id| {
            self.get_unchecked_node_degree_from_node_id(*node_id) < minimum_node_degree
        });

        // Compute the hash value by folding over the filtered neighbor nodes.
        let hash = filtered_neighbors.fold(
            seed | (node_ids.get_selfloop_excluded_group_node_degree(&self) as u64).as_(),
            |hash, node| hash | Word::ONE.wrapping_shl(node),
        );

        hash
    }

    /// Checks if two sets of node IDs are unchecked isomorphic in the context of a graph.
    ///
    /// # Safety
    /// This function is marked as `unsafe` because it assumes that the provided node ID sets are valid and that the associated graph is properly constructed.
    ///
    /// # Arguments
    ///
    /// * `first_node_id_set`: A reference to an object implementing the `IterNeighbours` and `SelfloopExcludedGroupNodeDegree` traits that represents the first set of node IDs.
    /// * `second_node_id_set`: A reference to an object implementing the `IterNeighbours` and `SelfloopExcludedGroupNodeDegree` traits that represents the second set of node IDs.
    ///
    /// # Returns
    /// A boolean value indicating whether the two sets of node IDs are unchecked isomorphic.
    ///
    /// # Type Parameters
    /// * `Candidate`: A generic type implementing both the `IterNeighbours` and `SelfloopExcludedGroupNodeDegree` traits.
    unsafe fn are_unchecked_isomorphic_from_node_id_sets<Candidate>(
        &self,
        first_node_id_set: &Candidate,
        second_node_id_set: &Candidate,
    ) -> bool
    where
        Candidate: IterNeighbours + SelfloopExcludedGroupNodeDegree,
    {
        let mut first = first_node_id_set
            .iter_neighbours_and_edge_ids(&self)
            .peekable();
        let mut second = second_node_id_set
            .iter_neighbours_and_edge_ids(&self)
            .peekable();

        // Counters for edges going FROM the group
        // to the SAME GROUP itself.
        let mut first_selfloops = 0;
        let mut second_selfloops = 0;

        // Counters for edges going FROM a group
        // to the OTHER group. These edges will be
        // certainly equal in undirected graphs,
        // but might now be in the context of directed
        // graphs. It remains relevant to check whether
        // the two groups are connected in order to verify
        // the topological isomorphism were there to be
        // self-loops in either groups.
        let mut first_to_second_connections = 0;
        let mut second_to_first_connections = 0;

        'outer: while let (Some((first_node, first_edge_id)), Some((second_node, second_edge_id))) =
            (first.peek(), second.peek())
        {
            // We start by evaluating whether we are dealing in either
            // the first or second isomorphic candidates with self-loops,
            // that is edges that go from any node in the isomorphic candidate
            // to any node in the SAME isomorphic candidate.
            // If so, we need to increase the relative counter and proceed onward.
            if first_node_id_set.is_selfloop(first_node) {
                first_selfloops += 1;
                first.advance_by(1).unwrap();
                continue 'outer;
            }

            if second_node_id_set.is_selfloop(second_node) {
                second_selfloops += 1;
                second.advance_by(1).unwrap();
                continue 'outer;
            }

            // Secondarily, we evaluate whether the first group
            // is connected to the second and viceversa.
            if second_node_id_set.is_selfloop(first_node) {
                first_to_second_connections += 1;
                first.advance_by(1).unwrap();
                continue 'outer;
            }

            if first_node_id_set.is_selfloop(second_node) {
                second_to_first_connections += 1;
                second.advance_by(1).unwrap();
                continue 'outer;
            }

            // Thirdly, and this is the most intuitive check
            // of all others, we need to evaluate whether
            // the two nodes are equal. If the two nodes
            // are not equal, we found a difference between the
            // two neighbourhoods and therefore the two candidate
            // isomorphisms are NOT isomorphic.
            if first_node != second_node {
                return false;
            }

            // We check whether the two edges connecting the neighbouring
            // node to the two candidate isomorphic groups are identical
            if self.get_unchecked_edge_type_id_from_edge_id(*first_edge_id)
                != self.get_unchecked_edge_type_id_from_edge_id(*second_edge_id)
            {
                return false;
            }

            // And finally, we check whether the two edges connecting the neighbouring
            // node to the two candidate isomorphic groups are identical
            if let (Some(first_weight), Some(second_weight)) = (
                self.get_unchecked_edge_weight_from_edge_id(*first_edge_id),
                self.get_unchecked_edge_weight_from_edge_id(*second_edge_id),
            ) {
                if (first_weight - second_weight).abs() > WeightT::EPSILON {
                    return false;
                }
            }

            first.advance_by(1).unwrap();
            second.advance_by(1).unwrap();
        }

        // We need to fully complete consuming both iterators.
        // It may happen that the previous loop finishes with
        // one iterator completed and the other one still with
        // some nodes.
        for (first_node, _first_edge_id) in first {
            // If this is a selfloop.
            if first_node_id_set.is_selfloop(&first_node) {
                first_selfloops += 1;
                continue;
            }

            // If this is an edge towards the other loop.
            if second_node_id_set.is_selfloop(&first_node) {
                first_to_second_connections += 1;
                continue;
            }

            // Otherwise this is a new node that no longer
            // matches the other iterator, so we can stop.
            return false;
        }

        for (second_node, _second_edge_id) in second {
            // If this is a selfloop.
            if second_node_id_set.is_selfloop(&second_node) {
                second_selfloops += 1;
                continue;
            }

            // If this is an edge towards the other loop.
            if first_node_id_set.is_selfloop(&second_node) {
                second_to_first_connections += 1;
                continue;
            }

            // Otherwise this is a new node that no longer
            // matches the other iterator, so we can stop.
            return false;
        }

        // We check that is one of the isomorphic groups
        // has self-loops, than the other one either has
        // self-loops or is connected to the first isomorphic group.
        if first_selfloops > 0 && !(second_selfloops > 0 || first_to_second_connections > 0)
            || second_selfloops > 0 && !(first_selfloops > 0 || second_to_first_connections > 0)
        {
            return false;
        }

        true
    }

    /// Returns parallel iterator of vectors of isomorphic edges groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for each of the two nodes involved in the edge isomorphism. By default, 10.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    fn par_iter_isomorphic_node_group_ids<CandidatesGenerator, Isomorphism, Word: WordInteger>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: CandidatesGenerator,
    ) -> Result<impl ParallelIterator<Item = Vec<Isomorphism>> + '_>
    where
        u64: AsPrimitive<Word>,
        CandidatesGenerator: IsomorphicCandidateGenerator<Isomorphism>,
        Isomorphism: SelfloopExcludedGroupNodeDegree
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        // If the graph does not have edges, it is pointless.
        self.must_have_edges()?;

        // If no minimum node degree is provided, we use arbitrarily 10.
        let minimum_node_degree =
            minimum_node_degree.unwrap_or(10.min(self.get_maximum_node_degree().unwrap_or(0)));
        let number_of_neighbours_for_hash = number_of_neighbours_for_hash.unwrap_or(10);

        // We collect the node IDs that have degree higher than the provided one.
        let mut degree_bounded_hash_and_edge_ids: Vec<(Word, Isomorphism)> = candidates_generator
            .par_iter_isomorphic_candidates(&self, minimum_node_degree)
            .map(move |(seed, group)| {
                (
                    unsafe {
                        self.get_hash_from_node_ids(
                            &group,
                            minimum_node_degree,
                            number_of_neighbours_for_hash,
                            seed,
                        )
                    },
                    group,
                )
            })
            .collect::<Vec<(Word, Isomorphism)>>();

        if degree_bounded_hash_and_edge_ids.len() <= 1 {
            return Err(format!(
                concat!(
                    "The provided parametrization in the current graph, ",
                    "including specifically minimum_node_degree=`{minimum_node_degree}`, ",
                    "has caused the list of degree-bounded nodes to be empty. ",
                    "Consider relaxing the constraints."
                ),
                minimum_node_degree = minimum_node_degree
            ));
        }

        degree_bounded_hash_and_edge_ids
            .par_sort_unstable_by(|(left, _), (right, _)| left.cmp(right));

        Ok(
            unsafe { EqualBucketsParIter::new(degree_bounded_hash_and_edge_ids) }.flat_map(
                move |slice| {
                    let mut empty_intersections: Vec<bool> = vec![];
                    let mut candidate_isomorphic_groups: Vec<Vec<_>> = vec![];

                    for (_hash, other) in slice.iter().skip(1) {
                        // Then, since within the same hash there might be multiple isomorphic node groups in collision
                        // we need to identify which one of these groups is actually isomorphic with the current node.
                        if let Some((isomorphic_group, empty_intersection)) =
                            //
                            candidate_isomorphic_groups
                                    .iter_mut()
                                    .zip_eq(empty_intersections.iter_mut())
                                    .find(|(candidate_isomorphic_group, _)| unsafe {
                                        self.are_unchecked_isomorphic_from_node_id_sets(
                                            &candidate_isomorphic_group[0],
                                            other,
                                        )
                                    })
                        {
                            if !*empty_intersection {
                                *empty_intersection |= isomorphic_group[0].intersects(other);
                            }
                            isomorphic_group.push(other.clone());
                        } else {
                            // We may have found another isomorphic group, or, possibly, a single node
                            // with a colliding hash. As such, we will need to verify whether this group
                            // will effectively grow or not.
                            empty_intersections.push(false);
                            candidate_isomorphic_groups.push(vec![other.clone()]);
                        }
                    }

                    let mut i = 0;
                    candidate_isomorphic_groups.retain(|_| {
                        i += 1;
                        empty_intersections[i - 1]
                    });

                    candidate_isomorphic_groups
                },
            ),
        )
    }

    /// Returns parallel iterator over isomorphic groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    fn par_iter_isomorphic_node_group_names<
        CandidatesGenerator,
        Isomorphism,
        IsomorphismNames,
        Word: WordInteger,
    >(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: CandidatesGenerator,
    ) -> Result<impl ParallelIterator<Item = Vec<IsomorphismNames>> + '_>
    where
        u64: AsPrimitive<Word>,
        CandidatesGenerator: IsomorphicCandidateGenerator<Isomorphism>,
        IsomorphismNames: Send + Sync,
        Isomorphism: SelfloopExcludedGroupNodeDegree
            + IterNeighbours
            + ToNodeNames<IsomorphismNames>
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(self
            .par_iter_isomorphic_node_group_ids::<CandidatesGenerator, Isomorphism, Word>(
                minimum_node_degree,
                number_of_neighbours_for_hash,
                candidates_generator,
            )?
            .map(move |ws| {
                ws.into_iter()
                    .map(|w| w.to_node_names(&self))
                    .collect::<Vec<IsomorphismNames>>()
            }))
    }

    /// Returns vector with isomorphic groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    fn get_isomorphic_node_group_hashes<CandidatesGenerator, Isomorphism>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: CandidatesGenerator,
        dtype: Option<&str>,
    ) -> Result<Vec<u64>>
    where
        CandidatesGenerator: IsomorphicCandidateGenerator<Isomorphism>,
        Isomorphism: SelfloopExcludedGroupNodeDegree
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        // If the graph does not have edges, it is pointless.
        self.must_have_edges()?;

        // If no minimum node degree is provided, we use arbitrarily 10.
        let minimum_node_degree =
            minimum_node_degree.unwrap_or(10.min(self.get_maximum_node_degree().unwrap_or(0)));
        let number_of_neighbours_for_hash = number_of_neighbours_for_hash.unwrap_or(10);

        Ok(match dtype.unwrap_or("u32") {
            "u8" => candidates_generator
                .par_iter_isomorphic_candidates::<u8>(&self, minimum_node_degree)
                .map(move |(seed, group)| unsafe {
                    self.get_hash_from_node_ids(
                        &group,
                        minimum_node_degree,
                        number_of_neighbours_for_hash,
                        seed,
                    ) as u64
                })
                .collect::<Vec<u64>>(),
            "u16" => candidates_generator
                .par_iter_isomorphic_candidates::<u16>(&self, minimum_node_degree)
                .map(move |(seed, group)| unsafe {
                    self.get_hash_from_node_ids(
                        &group,
                        minimum_node_degree,
                        number_of_neighbours_for_hash,
                        seed,
                    ) as u64
                })
                .collect::<Vec<u64>>(),
            "u32" => candidates_generator
                .par_iter_isomorphic_candidates::<u32>(&self, minimum_node_degree)
                .map(move |(seed, group)| unsafe {
                    self.get_hash_from_node_ids(
                        &group,
                        minimum_node_degree,
                        number_of_neighbours_for_hash,
                        seed,
                    ) as u64
                })
                .collect::<Vec<u64>>(),
            "u64" => candidates_generator
                .par_iter_isomorphic_candidates::<u64>(&self, minimum_node_degree)
                .map(move |(seed, group)| unsafe {
                    self.get_hash_from_node_ids(
                        &group,
                        minimum_node_degree,
                        number_of_neighbours_for_hash,
                        seed,
                    )
                })
                .collect::<Vec<u64>>(),
            _ => Err(format!(
                concat!(
                    "The provided data type `{dtype}` is not supported. ",
                    "Please choose one of the following: `u8`, `u16`, `u32`, `u64`."
                ),
                dtype = dtype.unwrap_or("u32")
            ))?,
        })
    }

    /// Returns vector with isomorphic groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    fn get_isomorphic_group_ids<CandidatesGenerator, Isomorphism>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: CandidatesGenerator,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<Isomorphism>>>
    where
        CandidatesGenerator: IsomorphicCandidateGenerator<Isomorphism>,
        Isomorphism: SelfloopExcludedGroupNodeDegree
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(match dtype.unwrap_or("u32") {
            "u8" => self
                .par_iter_isomorphic_node_group_ids::<CandidatesGenerator, Isomorphism, u8>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .map(|ws| ws.into_iter().map(|w| w.into()).collect())
                .collect(),
            "u16" => self
                .par_iter_isomorphic_node_group_ids::<CandidatesGenerator, Isomorphism, u16>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .map(|ws| ws.into_iter().map(|w| w.into()).collect())
                .collect(),
            "u32" => self
                .par_iter_isomorphic_node_group_ids::<CandidatesGenerator, Isomorphism, u32>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .map(|ws| ws.into_iter().map(|w| w.into()).collect())
                .collect(),
            "u64" => self
                .par_iter_isomorphic_node_group_ids::<CandidatesGenerator, Isomorphism, u64>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .map(|ws| ws.into_iter().map(|w| w.into()).collect())
                .collect(),
            _ => Err(format!(
                concat!(
                    "The provided data type `{dtype}` is not supported. ",
                    "Please choose one of the following: `u8`, `u16`, `u32`, `u64`."
                ),
                dtype = dtype.unwrap_or("u32")
            ))?,
        })
    }

    /// Returns vector with isomorphic groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    fn get_isomorphic_group_names<CandidatesGenerator, Isomorphism, IsomorphismNames>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: CandidatesGenerator,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<IsomorphismNames>>>
    where
        CandidatesGenerator: IsomorphicCandidateGenerator<Isomorphism>,
        IsomorphismNames: Send + Sync,
        Isomorphism: SelfloopExcludedGroupNodeDegree
            + ToNodeNames<IsomorphismNames>
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(match dtype.unwrap_or("u32") {
            "u8" => self
                .par_iter_isomorphic_node_group_names::<CandidatesGenerator, Isomorphism, IsomorphismNames, u8>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .collect(),
            "u16" => self
                .par_iter_isomorphic_node_group_names::<CandidatesGenerator, Isomorphism, IsomorphismNames, u16>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .collect(),
            "u32" => self
                .par_iter_isomorphic_node_group_names::<CandidatesGenerator, Isomorphism, IsomorphismNames, u32>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .collect(),
            "u64" => self
                .par_iter_isomorphic_node_group_names::<CandidatesGenerator, Isomorphism, IsomorphismNames, u64>(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    candidates_generator,
                )?
                .collect(),
            _ => Err(format!(
                concat!(
                    "The provided data type `{dtype}` is not supported. ",
                    "Please choose one of the following: `u8`, `u16`, `u32`, `u64`."
                ),
                dtype = dtype.unwrap_or("u32")
            ))?,
        })
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<NodeT>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            NodeIsomorphismsGenerator::default(),
            dtype,
        )?)
    }

    /// Retrieves a vector of flattened and repeated isomorphic node IDs, that is by removing one per group.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - An optional parameter specifying the minimum degree a node must have to be included
    ///   in the isomorphisms. Nodes with a degree lower than this value will be excluded. Set to `None` to include all nodes.
    /// * `number_of_neighbours_for_hash`: Option<NodeT> - An optional parameter determining the number of neighboring nodes used for hashing
    ///   in the isomorphisms generation. Increasing this value can increase the uniqueness of the generated isomorphisms.
    ///   Set to `None` to use the default number of neighbors for hashing.
    ///
    /// # Returns
    /// A `Result` containing a vector of node IDs. If the operation is successful, the vector contains the flattened and
    /// repeated isomorphic node IDs. If an error occurs during the execution, the error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use your_library::*;
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let minimum_degree = Some(3);
    /// let num_neighbors_for_hash = Some(5);
    /// let result = graph.get_flat_repeated_isomorphic_node_ids(minimum_degree, num_neighbors_for_hash);
    ///
    /// match result {
    ///     Ok(node_ids) => {
    ///         println!("Flattened isomorphic node IDs: {:?}", node_ids);
    ///         // Further processing of the node IDs...
    ///     }
    ///     Err(err) => {
    ///         eprintln!("An error occurred: {}", err);
    ///         // Handle the error...
    ///     }
    /// }
    /// ```
    pub fn get_flat_repeated_isomorphic_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<NodeT>> {
        Ok(self
            .par_iter_isomorphic_node_group_ids::<NodeIsomorphismsGenerator, NodeT, u32>(
                minimum_node_degree,
                number_of_neighbours_for_hash,
                NodeIsomorphismsGenerator::default(),
            )?
            .flat_map(|mut group| {
                group.pop();
                group
            })
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<String>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            NodeIsomorphismsGenerator::default(),
            dtype,
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_edge_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<[NodeT; 2]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            EdgeIsomorphismsGenerator::default(),
            dtype,
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_edge_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<[String; 2]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            EdgeIsomorphismsGenerator::default(),
            dtype,
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic tuple groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    /// # Implementative details
    /// A node tuple is NOT necessarily connected. These are simply two
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_tuple_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<[NodeT; 2]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TupleIsomorphismsGenerator::default(),
            dtype,
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic tuple groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `dtype`: Option<&str> - The data type of the hash. By default, `&str`.
    ///
    /// # Implementative details
    /// A node tuple is NOT necessarily connected. These are simply two
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_tuple_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> Result<Vec<Vec<[String; 2]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TupleIsomorphismsGenerator::default(),
            dtype,
        )?)
    }
}
