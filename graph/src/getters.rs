use super::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::once;

impl Graph {
    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing if graph has weights.
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing if graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Return iterator of nodes that have selfloops.
    pub fn get_selfloops_iter(&self) -> impl Iterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number()).filter(move |node| self.has_edge(*node, *node))
    }

    /// Return boolean representing if graph has selfloops.
    pub fn has_selfloops(&self) -> bool {
        self.get_selfloops_iter().any(|_| true)
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> usize {
        self.nodes.len()
    }

    /// Returns number of edges in the graph.
    pub fn get_edges_number(&self) -> usize {
        self.edges.len()
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> usize {
        if let Some(etm) = &self.edge_types {
            etm.len()
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> usize {
        if let Some(etm) = &self.node_types {
            etm.len()
        } else {
            0
        }
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.get_node_degree(node) as NodeT)
            .collect::<Vec<NodeT>>()
    }

    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.get_edges_iter()
            .flat_map(|(src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node)| (node, i))
            .collect()
    }

    /// Return iterator on the edges of the graph.
    pub fn get_edges_iter(&self) -> impl Iterator<Item = (NodeT, NodeT)> + '_ {
        self.edges.iter().map(move |edge| self.decode_edge(edge))
    }

    /// Return iterator on the edges of the graph.
    pub fn get_edges_enumerate(&self) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges.enumerate().map(move |(edge_id, edge)| {
            let (src, dst) = self.decode_edge(edge);
            (edge_id as EdgeT, src, dst)
        })
    }

    /// Return iterator on the edges of the graph.
    pub fn get_edges_par_enumerate(
        &self,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges.par_enumerate().map(move |(edge_id, edge)| {
            let (src, dst) = self.decode_edge(edge);
            (edge_id as EdgeT, src, dst)
        })
    }

    /// Return parallel iterator on the edges (as triples) of the graph.
    pub fn get_edge_triples_par_enumerate(
        &self,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.get_edges_par_enumerate()
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    match &self.edge_types {
                        Some(et) => Some(et.ids[edge_id]),
                        None => None,
                    },
                )
            })
    }

    pub fn get_edge_quadruples_par_enumerate(
        &self,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_
    {
        self.get_edge_triples_par_enumerate()
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    match &self.weights {
                        Some(ws) => Some(ws[edge_id]),
                        None => None,
                    },
                )
            })
    }

    /// Return iterator on the edges (as triples) of the graph.
    pub fn get_edge_triples_enumerate(
        &self,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.get_edges_enumerate().map(move |(edge_id, src, dst)| {
            (
                edge_id,
                src,
                dst,
                match &self.edge_types {
                    Some(et) => Some(et.ids[edge_id]),
                    None => None,
                },
            )
        })
    }

    pub fn get_edge_quadruples_enumerate(
        &self,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.get_edge_triples_enumerate()
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    match &self.weights {
                        Some(ws) => Some(ws[edge_id]),
                        None => None,
                    },
                )
            })
    }

    pub fn get_edge_quadruples(
        &self,
    ) -> impl Iterator<Item = (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.get_edge_quadruples_enumerate()
            .map(move |(_, src, dst, edge_type, weight)| (src, dst, edge_type, weight))
    }

    /// Return iterator on the edges of the graph.
    pub fn get_unique_edges_iter(&self) -> impl Iterator<Item = (NodeT, NodeT)> + '_ {
        self.get_edges_iter().unique()
    }

    /// Return iterator on the edges of the graph.
    pub fn get_unique_edges(&self) -> HashSet<(NodeT, NodeT)> {
        self.get_unique_edges_iter().collect()
    }

    /// Return parallel iterator on the edges of the graph.
    pub fn get_edges_par_iter(&self) -> impl ParallelIterator<Item = (NodeT, NodeT)> + '_ {
        self.edges
            .par_iter()
            .map(move |edge| decode_edge(edge, self.node_bits, self.node_bit_mask))
    }

    /// Return iterable of the sources.
    pub fn get_sources_par_iter(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.get_edges_par_iter().map(|(src, _)| src)
    }

    /// Return iterable of the sources.
    pub fn get_sources_iter(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.get_edges_iter().map(|(src, _)| src)
    }

    pub fn get_destination(&self, edge_id: EdgeT) -> NodeT {
        self.get_edge_from_edge_id(edge_id).1
    }

    pub fn get_destinations_range(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (min_edge_id..max_edge_id).map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_source_destinations_range(&self, src: NodeT) -> impl Iterator<Item = NodeT> + '_ {
        self.get_unchecked_destinations_range(src)
            .map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_unique_sources_number(&self) -> NodeT {
        self.unique_sources.len()
    }

    pub fn get_trap_nodes(&self) -> HashSet<NodeT> {
        (0..self.get_nodes_number())
            .filter(|candidate_src| !self.unique_sources.contains(*candidate_src as u64))
            .collect()
    }
}
