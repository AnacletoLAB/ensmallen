use super::*;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use log::info;
use rayon::prelude::*;
use vec_rand::{sample, sample_uniform};

impl Graph {
    /// Return the node transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions, if this is bigger that the number of nodes it will panic.
    /// * change_node_type_weight: ParamsT, weight for changing node type.
    ///
    fn get_node_transition(
        &self,
        node: NodeT,
        change_node_type_weight: ParamsT,
    ) -> (Vec<WeightT>, &[NodeT], EdgeT, EdgeT) {
        // Retrieve edge boundaries.
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        // If weights are given
        let mut transition: Vec<WeightT> = if let Some(w) = &self.weights {
            w[min_edge..max_edge].to_vec()
        } else {
            vec![1.0; max_edge - min_edge]
        };

        let destinations: &[NodeT] = &self.destinations[min_edge..max_edge];

        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if (change_node_type_weight - 1.0).abs() > f64::EPSILON {
            // If the node types were given:
            if let Some(nt) = &self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.
                let this_type: NodeTypeT = nt.ids[node];

                transition
                    .iter_mut()
                    .zip(destinations.iter().map(|dst| nt.ids[*dst]))
                    .filter(|(_, neigh_type)| this_type == *neigh_type)
                    .for_each(|(transition_value, _)| *transition_value /= change_node_type_weight);
                // credo non serva collect perche' modifichiamo i valori direttamente
            }
        }
        (transition, destinations, min_edge, max_edge)
    }

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT - the previous edge from which to compute the transitions.
    /// * weights: WalkWeights - Weights to use for the weighted walk.
    fn get_edge_transition(
        &self,
        edge: EdgeT,
        walk_weights: &WalkWeights,
    ) -> (Vec<WeightT>, &[NodeT], EdgeT, EdgeT) {
        // Get the source and destination for current edge.
        let (src, dst) = (self.sources[edge], self.destinations[edge]);

        // Compute the transition weights relative to the node weights.
        let (mut transition, destinations, min_edge, max_edge) =
            self.get_node_transition(dst, walk_weights.change_node_type_weight);

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if (walk_weights.change_edge_type_weight - 1.0).abs() > f64::EPSILON {
            if let Some(et) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: EdgeTypeT = et.ids[edge];
                transition
                    .iter_mut()
                    .zip(et.ids[min_edge..max_edge].iter())
                    .filter(|(_, &neigh_type)| this_type == neigh_type)
                    .for_each(|(transition_value, _)| {
                        *transition_value /= walk_weights.change_edge_type_weight
                    });
            }
        }

        //############################################################
        //# Handling of the P parameter: the return coefficient      #
        //############################################################

        //# If the neigbour matches with the source, hence this is
        //# a backward loop like the following:
        //# SRC -> DST
        //#  ▲     /
        //#   \___/
        //#
        //# We weight the edge weight with the given return weight.

        // If the return weight, which is the inverse of p, is not 1, hence
        // it has some impact, we procced and increase by the given weight
        // the probability of transitions that go back a previously visited
        // node.
        if (walk_weights.return_weight - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| src == *ndst || dst == *ndst)
                .for_each(|(transition_value, _)| *transition_value *= walk_weights.return_weight);
        }
        //############################################################
        //# Handling of the Q parameter: the exploration coefficient #
        //############################################################

        if (walk_weights.explore_weight - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| {
                    (src != *ndst || dst == *ndst) && !self.unique_edges.contains_key(&(*ndst, src))
                })
                .for_each(|(transition_value, _)| *transition_value *= walk_weights.explore_weight);
        }

        (transition, destinations, min_edge, max_edge)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    ///
    pub fn extract_uniform_node(&self, node: NodeT, seed: usize) -> NodeT {
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        self.destinations[min_edge + sample_uniform((max_edge - min_edge) as u64, seed as u64)]
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    /// * change_node_type_weight: ParamsT, weight for changing node type.
    pub fn extract_node(
        &self,
        node: NodeT,
        seed: usize,
        change_node_type_weight: ParamsT,
    ) -> (NodeT, EdgeT) {
        let (mut weights, dsts, min_edge, _) =
            self.get_node_transition(node, change_node_type_weight);
        let index = sample(&mut weights, seed as u64);
        (dsts[index], min_edge + index)
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT, the previous edge from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    pub fn extract_edge(
        &self,
        edge: EdgeT,
        seed: usize,
        walk_weights: &WalkWeights,
    ) -> (NodeT, EdgeT) {
        let (mut weights, dsts, min_edge, _) = self.get_edge_transition(edge, walk_weights);
        let index = sample(&mut weights, seed as u64);
        (dsts[index], min_edge + index)
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn walk(&self, parameters: &WalksParameters) -> Result<Vec<Vec<NodeT>>, String> {
        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        info!("Starting random walk.");
        let pb = if parameters.verbose {
            let pb = ProgressBar::new(parameters.total_iterations() as u64);
            pb.set_draw_delta(parameters.total_iterations() as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Computing random walks {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let iterator = (0..parameters.total_iterations())
            .into_par_iter()
            .progress_with(pb)
            .map(|index| {
                (
                    parameters.seed + index,
                    self.not_trap_nodes[parameters.mode_index(index)],
                )
            });

        let mut walks = if self.has_traps {
            if self.weights.is_none() && parameters.is_first_order_walk() {
                info!("Using trap-aware uniform first order random walk algorithm.");
                iterator
                    .map(|(seed, node)| {
                        self.uniform_walk(node, seed, &parameters.single_walk_parameters)
                    })
                    .filter(|walk| walk.len() >= parameters.min_length)
                    .collect::<Vec<Vec<NodeT>>>()
            } else {
                info!("Using trap-aware second order random walk algorithm.");
                iterator
                    .map(|(seed, node)| {
                        self.single_walk(node, seed, &parameters.single_walk_parameters)
                    })
                    .filter(|walk| walk.len() >= parameters.min_length)
                    .collect::<Vec<Vec<NodeT>>>()
            }
        } else if self.weights.is_none() && parameters.is_first_order_walk() {
            info!("Using uniform first order random walk algorithm.");
            iterator
                .map(|(seed, node)| {
                    self.uniform_walk_no_traps(node, seed, &parameters.single_walk_parameters)
                })
                .collect::<Vec<Vec<NodeT>>>()
        } else {
            info!("Using second order random walk algorithm.");
            iterator
                .map(|(seed, node)| {
                    self.single_walk_no_traps(node, seed, &parameters.single_walk_parameters)
                })
                .collect::<Vec<Vec<NodeT>>>()
        };

        if let Some(dense_nodes_mapping) = &parameters.dense_nodes_mapping {
            walks.par_iter_mut().for_each(|walk| {
                walk.iter_mut()
                    .for_each(|node| *node = *dense_nodes_mapping.get(node).unwrap())
            })
        }

        Ok(walks)
    }

    /// Returns single walk from given node
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let (dst, mut edge) =
            self.extract_node(node, seed, parameters.weights.change_node_type_weight);

        if self.is_node_trap(dst) {
            return vec![node, dst];
        }

        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            if self.is_edge_trap(edge) {
                break;
            }
            let (dst, inner_edge) = self.extract_edge(edge, iteration + seed, &parameters.weights);
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk_no_traps(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);

        let (dst, mut edge) =
            self.extract_node(node, seed, parameters.weights.change_node_type_weight);
        walk.push(dst);

        for iteration in 2..parameters.length {
            let (dst, inner_edge) = self.extract_edge(edge, seed + iteration, &parameters.weights);
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }

    /// Returns single walk from given node executed uniformely.
    ///
    /// This walk executes uniformely a walk of first order. This method
    /// works in context of uniform graphs (all weights are None) and the
    /// weights of the node2vec are all equal to 1.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn uniform_walk(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut dst = self.extract_uniform_node(node, seed);

        if self.is_node_trap(dst) {
            return vec![node, dst];
        }

        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            if self.is_node_trap(dst) {
                break;
            }
            dst = self.extract_uniform_node(dst, seed + iteration);
            walk.push(dst);
        }
        walk
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn uniform_walk_no_traps(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        let mut dst = self.extract_uniform_node(node, seed);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            dst = self.extract_uniform_node(dst, seed + iteration);
            walk.push(dst);
        }
        walk
    }
}
