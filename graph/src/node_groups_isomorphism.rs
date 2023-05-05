use super::*;
use crate::hashes::*;
use crate::isomorphism_iter::EqualBucketsParIter;
use rayon::prelude::*;

#[no_binding]
#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct WrapperIsomorphism<const N: usize> {
    nodes: [NodeT; N],
}

impl From<usize> for WrapperIsomorphism<1> {
    fn from(value: usize) -> Self {
        Self {
            nodes: [value as NodeT],
        }
    }
}

impl From<NodeT> for WrapperIsomorphism<1> {
    fn from(value: NodeT) -> Self {
        Self { nodes: [value] }
    }
}

impl<const N: usize> From<[NodeT; N]> for WrapperIsomorphism<N> {
    fn from(nodes: [NodeT; N]) -> Self {
        Self { nodes }
    }
}

impl<const N: usize> Into<[NodeT; N]> for WrapperIsomorphism<N> {
    fn into(self) -> [NodeT; N] {
        self.nodes
    }
}

impl Into<NodeT> for WrapperIsomorphism<1> {
    fn into(self) -> NodeT {
        self.nodes[0]
    }
}

trait WrapperToNodeNames<const N: usize> {
    fn to_node_names(&self, graph: &Graph) -> [String; N];
}

pub trait SelfloopExcludedGroupNodeDegree {
    fn get_selfloop_excluded_group_node_degree(&self, graph: &Graph) -> usize;
    fn is_selfloop(&self, node_id: &NodeT) -> bool;
    fn intersects(&self, other: &Self) -> bool;
}

pub trait IsomorphicCandidateGenerator<W>
where
    W: Send + Sync,
{
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, W)> + 'a;
}

#[no_binding]
#[derive(Default)]
pub struct NodeIsomorphismsGenerator;

#[no_binding]
#[derive(Default)]
pub struct EdgeIsomorphismsGenerator;

#[no_binding]
#[derive(Default)]
pub struct TupleIsomorphismsGenerator;

#[no_binding]
#[derive(Default)]
pub struct TripleIsomorphismsGenerator;

#[no_binding]
#[derive(Default)]
pub struct TriadIsomorphismsGenerator;

impl IsomorphicCandidateGenerator<WrapperIsomorphism<1>> for NodeIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, WrapperIsomorphism<1>)> + 'a {
        graph
            .par_iter_node_degrees()
            .enumerate()
            .filter(move |(_, node_degree)| *node_degree >= minimum_node_degree)
            .map(move |(node_id, _)| {
                let mut hasher = Hasher::commutative_simple();
                hasher.update(&unsafe {
                    graph.get_unchecked_node_type_ids_from_node_id(node_id as NodeT)
                });
                (hasher.digest(), node_id.into())
            })
    }
}

impl IsomorphicCandidateGenerator<WrapperIsomorphism<2>> for EdgeIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, WrapperIsomorphism<2>)> + 'a {
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
                        (second_hasher.digest(), [src, dst].into())
                    })
            })
    }
}

impl IsomorphicCandidateGenerator<WrapperIsomorphism<2>> for TupleIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, WrapperIsomorphism<2>)> + 'a {
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
                        (second_hasher.digest(), [src, dst].into())
                    })
            })
    }
}

impl IsomorphicCandidateGenerator<WrapperIsomorphism<3>> for TripleIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, WrapperIsomorphism<3>)> + 'a {
        graph
            .par_iter_node_ids()
            .zip(graph.par_iter_node_degrees())
            .filter(move |(_, node_degree)| *node_degree > minimum_node_degree)
            .flat_map(move |(first, _first_node_degree)| {
                let mut first_hasher = Hasher::commutative_simple();
                first_hasher.update(&unsafe {
                    graph.get_unchecked_node_type_ids_from_node_id(first as NodeT)
                });
                graph
                    .par_iter_node_ids()
                    .zip(graph.par_iter_node_degrees())
                    .filter(move |(second, node_degree)| {
                        first != *second
                            && (graph.is_directed() || first < *second)
                            && *node_degree > minimum_node_degree
                    })
                    .flat_map(move |(second, _)| {
                        let mut second_hasher = first_hasher.clone();
                        second_hasher.update(&unsafe {
                            graph.get_unchecked_node_type_ids_from_node_id(second as NodeT)
                        });
                        graph
                            .par_iter_node_ids()
                            .zip(graph.par_iter_node_degrees())
                            .filter(move |(third, node_degree)| {
                                *third != second
                                    && first != *third
                                    && (graph.is_directed() || second < *third)
                                    && *node_degree > minimum_node_degree
                            })
                            .map(move |(third, _)| {
                                let mut third_hasher = second_hasher.clone();
                                third_hasher.update(&unsafe {
                                    graph.get_unchecked_node_type_ids_from_node_id(third as NodeT)
                                });
                                (third_hasher.digest(), [first, second, third].into())
                            })
                    })
            })
    }
}

impl IsomorphicCandidateGenerator<WrapperIsomorphism<3>> for TriadIsomorphismsGenerator {
    fn par_iter_isomorphic_candidates<'a>(
        &'a self,
        graph: &'a Graph,
        minimum_node_degree: NodeT,
    ) -> impl ParallelIterator<Item = (u64, WrapperIsomorphism<3>)> + 'a {
        graph
            .par_iter_node_ids()
            .zip(graph.par_iter_node_degrees())
            .filter(move |(_, node_degree)| *node_degree > minimum_node_degree)
            .flat_map(move |(first, _first_node_degree)| {
                let (min_edge_id, max_edge_id) =
                    unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(first) };
                let min_edge_id = min_edge_id as usize;
                let max_edge_id = max_edge_id as usize;
                let first_edge_type_ids = graph
                    .edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| &ets.ids[min_edge_id..max_edge_id]);
                let mut first_hasher = Hasher::commutative_simple();
                first_hasher
                    .update(&unsafe { graph.get_unchecked_node_type_ids_from_node_id(first) });
                unsafe { graph.par_iter_unchecked_neighbour_node_ids_from_source_node_id(first) }
                    .enumerate()
                    .filter(move |(_i, second)| {
                        first != *second
                            && (graph.is_directed() || first < *second)
                            && unsafe { graph.get_unchecked_node_degree_from_node_id(*second) }
                                > minimum_node_degree
                    })
                    .flat_map(move |(i, second)| {
                        let mut second_hasher = first_hasher.clone();
                        second_hasher.update(&unsafe {
                            graph.get_unchecked_node_type_ids_from_node_id(second)
                        });
                        second_hasher.update(&first_edge_type_ids.as_ref().and_then(|ids| ids[i]));
                        let (min_edge_id, max_edge_id) = unsafe {
                            graph.get_unchecked_minmax_edge_ids_from_source_node_id(second)
                        };
                        let min_edge_id = min_edge_id as usize;
                        let max_edge_id = max_edge_id as usize;
                        let second_edge_type_ids = graph
                            .edge_types
                            .as_ref()
                            .as_ref()
                            .map(|ets| &ets.ids[min_edge_id..max_edge_id]);
                        unsafe {
                            graph.par_iter_unchecked_neighbour_node_ids_from_source_node_id(second)
                        }
                        .enumerate()
                        .filter(move |(_j, third)| {
                            first != *third
                                && second != *third
                                && unsafe { graph.get_unchecked_node_degree_from_node_id(*third) }
                                    > minimum_node_degree
                        })
                        .map(move |(j, third)| {
                            let mut third_hasher = second_hasher.clone();
                            third_hasher.update(&unsafe {
                                graph.get_unchecked_node_type_ids_from_node_id(third)
                            });
                            third_hasher
                                .update(&second_edge_type_ids.as_ref().and_then(|ids| ids[j]));
                            (third_hasher.digest(), [first, second, third].into())
                        })
                    })
            })
    }
}

pub trait IterNeighbours {
    fn iter(&self) -> impl Iterator<Item = NodeT> + '_;
    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a;
    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a;
    fn iter_selfloop_excluded_neighbours<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = NodeT> + 'a {
        iter_set::difference(self.iter_neighbours(&graph), self.iter())
    }
}

impl<const N: usize> WrapperIsomorphism<N> {
    pub fn len(&self) -> usize {
        N
    }
}

impl<const N: usize> SelfloopExcludedGroupNodeDegree for WrapperIsomorphism<N> {
    fn get_selfloop_excluded_group_node_degree(&self, graph: &Graph) -> usize {
        self.nodes
            .iter()
            .map(|node_id| unsafe {
                graph.get_unchecked_selfloop_excluded_node_degree_from_node_id(*node_id) as usize
            })
            .sum::<usize>()
            - self.len() * (self.len() - 1)
    }

    fn is_selfloop(&self, node_id: &NodeT) -> bool {
        self.nodes.contains(node_id)
    }

    fn intersects(&self, other: &Self) -> bool {
        iter_set::intersection(self.nodes.iter(), other.nodes.iter()).count() == 0
    }
}

impl WrapperToNodeNames<1> for WrapperIsomorphism<1> {
    fn to_node_names(&self, graph: &Graph) -> [String; 1] {
        [unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[0]) }]
    }
}

impl WrapperToNodeNames<2> for WrapperIsomorphism<2> {
    fn to_node_names(&self, graph: &Graph) -> [String; 2] {
        [
            unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[0]) },
            unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[1]) },
        ]
    }
}

impl WrapperToNodeNames<3> for WrapperIsomorphism<3> {
    fn to_node_names(&self, graph: &Graph) -> [String; 3] {
        [
            unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[0]) },
            unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[1]) },
            unsafe { graph.get_unchecked_node_name_from_node_id(self.nodes[2]) },
        ]
    }
}

impl IterNeighbours for WrapperIsomorphism<2> {
    fn iter(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.nodes.iter().copied()
    }

    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a {
        iter_set::union(
            unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0]) },
            unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[1]) },
        )
    }

    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a {
        let (first_min_edge, first_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[0]) };
        let (second_min_edge, second_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[1]) };
        iter_set::union_by(
            unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0])
                    .zip((first_min_edge..first_max_edge).into_iter())
            },
            unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[1])
                    .zip((second_min_edge..second_max_edge).into_iter())
            },
            |(first, _), (second, _)| first.cmp(&second),
        )
    }
}

impl IterNeighbours for WrapperIsomorphism<3> {
    fn iter(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.nodes.iter().copied()
    }

    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a {
        iter_set::union(
            iter_set::union(
                unsafe {
                    graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0])
                },
                unsafe {
                    graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[1])
                },
            ),
            unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[2]) },
        )
    }

    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a {
        let (first_min_edge, first_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[0]) };
        let (second_min_edge, second_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[1]) };
        let (third_min_edge, third_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[2]) };
        iter_set::union_by(
            iter_set::union_by(
                unsafe {
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0])
                        .zip((first_min_edge..first_max_edge).into_iter())
                },
                unsafe {
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[1])
                        .zip((second_min_edge..second_max_edge).into_iter())
                },
                |(first, _), (second, _)| first.cmp(&second),
            ),
            unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[2])
                    .zip((third_min_edge..third_max_edge).into_iter())
            },
            |(first, _), (second, _)| first.cmp(&second),
        )
    }
}

impl IterNeighbours for WrapperIsomorphism<1> {
    fn iter(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.nodes.iter().copied()
    }

    fn iter_neighbours<'a>(&'a self, graph: &'a Graph) -> impl Iterator<Item = NodeT> + 'a {
        unsafe { graph.iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0]) }
    }

    fn iter_neighbours_and_edge_ids<'a>(
        &'a self,
        graph: &'a Graph,
    ) -> impl Iterator<Item = (NodeT, EdgeT)> + 'a {
        let (first_min_edge, first_max_edge) =
            unsafe { graph.get_unchecked_minmax_edge_ids_from_source_node_id(self.nodes[0]) };
        unsafe {
            graph
                .iter_unchecked_neighbour_node_ids_from_source_node_id(self.nodes[0])
                .zip((first_min_edge..first_max_edge).into_iter())
        }
    }
}

impl Graph {
    unsafe fn get_hash_from_node_ids<W>(
        &self,
        node_ids: &W,
        minimum_node_degree: NodeT,
        number_of_neighbours_for_hash: usize,
        seed: u64,
    ) -> u64
    where
        W: IterNeighbours + SelfloopExcludedGroupNodeDegree,
    {
        // The following assumes we are dealing with a simple graph!
        // The following assumes we are dealing with a clique!
        node_ids
            .iter_selfloop_excluded_neighbours(&self)
            .take(number_of_neighbours_for_hash)
            .filter(|node_id| {
                self.get_unchecked_node_degree_from_node_id(*node_id) < minimum_node_degree
            })
            .fold(
                seed | node_ids.get_selfloop_excluded_group_node_degree(&self) as u64,
                |hash, node| hash | (1 << (node % 64)),
            )
    }

    unsafe fn are_unchecked_isomorphic_from_node_id_sets<W>(
        &self,
        first_node_id_set: &W,
        second_node_id_set: &W,
    ) -> bool
    where
        W: IterNeighbours + SelfloopExcludedGroupNodeDegree,
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
    pub fn par_iter_isomorphic_node_group_ids<G, W>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: G,
    ) -> Result<impl ParallelIterator<Item = Vec<W>> + '_>
    where
        G: IsomorphicCandidateGenerator<W>,
        W: SelfloopExcludedGroupNodeDegree
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
        let mut degree_bounded_hash_and_edge_ids: Vec<(u64, W)> = candidates_generator
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
            .collect::<Vec<(u64, W)>>();

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

        // Then we sort the nodes, according to the score.
        // TODO! This sorting operation is implemented using quicksort
        // and is general porpose, including support for swapping
        // large complex structs. This is overkill for our use
        // case, since we only need to sort u32s, and it is likely
        // we could re-implement this in an ad-hoc manner that
        // is sensibly faster.
        degree_bounded_hash_and_edge_ids.par_sort_unstable();

        Ok(
            unsafe { EqualBucketsParIter::new(degree_bounded_hash_and_edge_ids) }.flat_map(
                move |candidate_isomorphic_group_slice| {
                    let mut found_non_intersection = false;

                    // First, we proceed assuming for the best case scenario which
                    // would also be the fastest: if the `candidate_isomorphic_group_slice` is
                    // indeed an isomorphic group of edges.
                    let first = candidate_isomorphic_group_slice[0].1;
                    // We proceed to count how many of these edges are effectively isomorphic
                    // to the first one.
                    let number_of_initial_isomorphic_edges = 1 + candidate_isomorphic_group_slice
                        [1..]
                        .iter()
                        .take_while(|&(_, second)| unsafe {
                            let is_isomorphic =
                                self.are_unchecked_isomorphic_from_node_id_sets(&first, second);
                            if is_isomorphic && !found_non_intersection {
                                found_non_intersection = first.intersects(second);
                            }
                            is_isomorphic
                        })
                        .count();

                    // If all of the edges are isomorphic to the first node,
                    // then we have finished.
                    if number_of_initial_isomorphic_edges == candidate_isomorphic_group_slice.len()
                    {
                        if found_non_intersection {
                            return vec![candidate_isomorphic_group_slice
                                .iter()
                                .map(|&(_, node_id)| node_id)
                                .collect::<Vec<_>>()];
                        } else {
                            return Vec::new();
                        }
                    }

                    // We can do the same thing also for the case where we are only off by
                    // one node, since that is surely an hash singleton.
                    // Of course, we need to check that we would not be left with only
                    // a single node in the case of an slice of two candidate isomorphic edges.
                    if number_of_initial_isomorphic_edges > 1
                        && number_of_initial_isomorphic_edges
                            == candidate_isomorphic_group_slice.len() - 1
                    {
                        if found_non_intersection {
                            return vec![candidate_isomorphic_group_slice
                                [..number_of_initial_isomorphic_edges]
                                .iter()
                                .map(|&(_, node_id)| node_id)
                                .collect::<Vec<_>>()];
                        } else {
                            return Vec::new();
                        }
                    }

                    // Otherwise, we are in a situation where either we have multiple
                    // isomorphic groups that were smashed togheter by an hash collision,
                    // or we have hash singletons, that is edges that do not actually share
                    // the neighbours with these edges but have the same hash.

                    // The two initial isomorphic groups are composed by
                    let mut candidate_isomorphic_groups: Vec<Vec<_>> = vec![
                        // The edges that we have checked as being isomorphic
                        candidate_isomorphic_group_slice[..number_of_initial_isomorphic_edges]
                            .iter()
                            .map(|&(_, group)| group)
                            .collect::<Vec<_>>(),
                        // The first node that appeared to be not isomorphic to the previous ones
                        vec![
                            candidate_isomorphic_group_slice[number_of_initial_isomorphic_edges].1,
                        ],
                    ];

                    // We set a flag that determines whether we will need to filter out isomorphic groups with
                    // only a single element in them.
                    let mut number_of_isomorphic_groups_with_size_one =
                        if number_of_initial_isomorphic_edges == 1 {
                            // If the number of isomorphic edges we have managed to validate
                            // is nada, i.e. only the first one, we currently have two potentially hash singletons
                            // in the array `candidate_isomorphic_groups`.
                            2
                        } else {
                            // Otherwise, we have only one potential hash singleton in the array.
                            1
                        };
                    // We start to iterate to the edges that immediately follow the last node that
                    // we have already checked previously, and we keep all of the subsequent edges that have indeed the same local hash.
                    for (_, other) in candidate_isomorphic_group_slice
                        [(number_of_initial_isomorphic_edges + 1)..]
                        .iter()
                    {
                        // Then, since within the same hash there might be multiple isomorphic node groups in collision
                        // we need to identify which one of these groups is actually isomorphic with the current node.
                        if let Some(isomorphic_group) =
                            //
                            candidate_isomorphic_groups
                                .iter_mut()
                                .find(|candidate_isomorphic_group| unsafe {
                                    let is_isomorphic = self
                                        .are_unchecked_isomorphic_from_node_id_sets(
                                            &candidate_isomorphic_group[0],
                                            other,
                                        );
                                    if is_isomorphic && !found_non_intersection {
                                        found_non_intersection =
                                            candidate_isomorphic_group[0].intersects(other);
                                    }
                                    is_isomorphic
                                })
                        {
                            if isomorphic_group.len() == 1 {
                                number_of_isomorphic_groups_with_size_one -= 1;
                            }
                            isomorphic_group.push(other.clone());
                        } else {
                            // We may have found another isomorphic group, or, possibly, a single node
                            // with a colliding hash. As such, we will need to verify whether this group
                            // will effectively grow or not.
                            number_of_isomorphic_groups_with_size_one += 1;
                            candidate_isomorphic_groups.push(vec![other.clone()]);
                        }
                    }
                    // We check whether there may be groups with a single node,
                    // which of course do not count as isomorphic groups
                    if number_of_isomorphic_groups_with_size_one > 0 {
                        candidate_isomorphic_groups.retain(|candidate_isomorphic_group| {
                            candidate_isomorphic_group.len() > 1
                        });
                    }
                    if found_non_intersection {
                        candidate_isomorphic_groups
                    } else {
                        Vec::new()
                    }
                },
            ),
        )
    }

    /// Returns parallel iterator over isomorphic groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    fn par_iter_isomorphic_node_group_names<const N: usize, G>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: G,
    ) -> Result<impl ParallelIterator<Item = Vec<[String; N]>> + '_>
    where
        G: IsomorphicCandidateGenerator<WrapperIsomorphism<N>>,
        WrapperIsomorphism<N>: SelfloopExcludedGroupNodeDegree
            + WrapperToNodeNames<N>
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(self
            .par_iter_isomorphic_node_group_ids(
                minimum_node_degree,
                number_of_neighbours_for_hash,
                candidates_generator,
            )?
            .map(move |ws| {
                ws.into_iter()
                    .map(|w| w.to_node_names(&self))
                    .collect::<Vec<[String; N]>>()
            }))
    }

    /// Returns vector with isomorphic groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    fn get_isomorphic_group_ids<G, W, R>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: G,
    ) -> Result<Vec<Vec<R>>>
    where
        R: Send + Sync,
        G: IsomorphicCandidateGenerator<W>,
        W: SelfloopExcludedGroupNodeDegree
            + IterNeighbours
            + Into<R>
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(self
            .par_iter_isomorphic_node_group_ids(
                minimum_node_degree,
                number_of_neighbours_for_hash,
                candidates_generator,
            )?
            .map(|ws| ws.into_iter().map(|w| w.into()).collect())
            .collect())
    }

    /// Returns vector with isomorphic groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    fn get_isomorphic_group_names<const N: usize, G>(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        candidates_generator: G,
    ) -> Result<Vec<Vec<[String; N]>>>
    where
        G: IsomorphicCandidateGenerator<WrapperIsomorphism<N>>,
        WrapperIsomorphism<N>: SelfloopExcludedGroupNodeDegree
            + WrapperToNodeNames<N>
            + IterNeighbours
            + Send
            + Sync
            + Ord
            + Copy
            + Clone
            + 'static,
    {
        Ok(self
            .par_iter_isomorphic_node_group_names(
                minimum_node_degree,
                number_of_neighbours_for_hash,
                candidates_generator,
            )?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[NodeT; 1]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            NodeIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[String; 1]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            NodeIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_edge_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[NodeT; 2]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            EdgeIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_edge_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[String; 2]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            EdgeIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic tuple groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    ///
    /// # Implementative details
    /// A node tuple is NOT necessarily connected. These are simply two
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_tuple_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[NodeT; 2]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TupleIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic tuple groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    ///
    /// # Implementative details
    /// A node tuple is NOT necessarily connected. These are simply two
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_tuple_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[String; 2]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TupleIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic triple groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    ///
    /// # Implementative details
    /// A node triple is NOT necessarily connected. These are simply three
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_triple_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[NodeT; 3]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TripleIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic triple groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    ///
    /// # Implementative details
    /// A node triple is NOT necessarily connected. These are simply three
    /// nodes in the graph with degree higher than the requested amount.
    pub fn get_isomorphic_triple_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[String; 3]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TripleIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic triads groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_triads_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[NodeT; 3]>>> {
        Ok(self.get_isomorphic_group_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TriadIsomorphismsGenerator::default(),
        )?)
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic triads groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_triads_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<[String; 3]>>> {
        Ok(self.get_isomorphic_group_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            TriadIsomorphismsGenerator::default(),
        )?)
    }
}
