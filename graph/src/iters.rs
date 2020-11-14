use super::*;
use rayon::prelude::*;

impl Graph {
    /// Return iterator on the node of the graph.
    pub fn get_nodes_iter(&self) -> impl Iterator<Item = (NodeT, Option<NodeTypeT>)> + '_ {
        (0..self.get_nodes_number())
            .map(move |node_id| (node_id, self.get_unchecked_node_type(node_id)))
    }

    /// Return iterator on the node of the graph as Strings.
    pub fn get_nodes_names_iter(
        &self,
    ) -> impl Iterator<Item = (NodeT, String, Option<String>)> + '_ {
        (0..self.get_nodes_number()).map(move |node_id| {
            (
                node_id,
                self.nodes.translate(node_id).to_owned(),
                self.get_node_type_string(node_id),
            )
        })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_iter(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .iter()
            .enumerate()
            .filter_map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                if !directed && src > dst {
                    return None;
                }
                Some((edge_id as EdgeT, src, dst))
            })
    }

    /// Return iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_sources_iter(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.get_edges_iter(directed).map(move |(_, src, _)| src)
    }

    /// Return parallel iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_sources_par_iter(&self, directed: bool) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.get_edges_par_iter(directed)
            .map(move |(_, src, _)| src)
    }

    /// Return iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_destinations_iter(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.get_edges_iter(directed).map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_destinations_par_iter(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.get_edges_par_iter(directed)
            .map(move |(_, _, dst)| dst)
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_string_iter(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, String, String)> + '_ {
        self.get_edges_iter(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    self.nodes.translate(src).to_owned(),
                    self.nodes.translate(dst).to_owned(),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_iter(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .par_enumerate()
            .filter_map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                if !directed && src > dst {
                    return None;
                }
                Some((edge_id as EdgeT, src, dst))
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_string_iter(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, String, String)> + '_ {
        self.get_edges_par_iter(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    self.nodes.translate(src).to_owned(),
                    self.nodes.translate(dst).to_owned(),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_triples(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.get_edges_iter(directed)
            .map(move |(edge_id, src, dst)| {
                (edge_id, src, dst, self.get_unchecked_edge_type(edge_id))
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_string_triples(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, String, String, Option<String>)> + '_ {
        self.get_edges_string_iter(directed)
            .map(move |(edge_id, src, dst)| (edge_id, src, dst, self.get_edge_type_string(edge_id)))
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_string_triples(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, String, String, Option<String>)> + '_ {
        self.get_edges_par_string_iter(directed)
            .map(move |(edge_id, src, dst)| (edge_id, src, dst, self.get_edge_type_string(edge_id)))
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_string_quadruples(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, String, String, Option<String>, Option<WeightT>)> + '_
    {
        self.get_edges_par_string_triples(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (edge_id, src, dst, edge_type, self.get_edge_weight(edge_id))
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_string_quadruples(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, String, String, Option<String>, Option<WeightT>)> + '_ {
        self.get_edges_string_triples(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (edge_id, src, dst, edge_type, self.get_edge_weight(edge_id))
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_triples(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.get_edges_par_iter(directed)
            .map(move |(edge_id, src, dst)| {
                (edge_id, src, dst, self.get_unchecked_edge_type(edge_id))
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_quadruples(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.get_edges_triples(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (edge_id, src, dst, edge_type, self.get_edge_weight(edge_id))
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_edges_par_quadruples(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_
    {
        self.get_edges_par_triples(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (edge_id, src, dst, edge_type, self.get_edge_weight(edge_id))
            })
    }

    /// Return the src, dst, edge type and weight of a given edge id
    pub fn get_edge_quadruple(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        let (src, dst, edge_type) = self.get_edge_triple(edge_id);
        (src, dst, edge_type, self.get_edge_weight(edge_id))
    }

    /// Return the src, dst, edge type of a given edge id
    pub fn get_edge_triple(&self, edge_id: EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        let (src, dst) = self.get_edge_from_edge_id(edge_id);
        (src, dst, self.get_unchecked_edge_type(edge_id))
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_unique_edges_iter(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (NodeT, NodeT)> + '_ {
        // TODO: implement custom unique that uses bitvec instead of the default HashSet
        self.edges.iter_uniques().filter_map(move |edge| {
            let (src, dst) = self.decode_edge(edge);
            if !directed && src > dst {
                return None;
            }
            Some((src, dst))
        })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, wethever to filter out the undirected edges.
    pub fn get_unique_edges_par_iter(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT)> + '_ {
        // TODO: implement custom unique that uses bitvec instead of the default HashSet
        self.edges.par_iter_uniques().filter_map(move |edge| {
            let (src, dst) = self.decode_edge(edge);
            if !directed && src > dst {
                return None;
            }
            Some((src, dst))
        })
    }
}
