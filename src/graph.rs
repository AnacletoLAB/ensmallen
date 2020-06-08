use std::collections::{HashMap, HashSet};
use std::iter::Enumerate;
use std::iter::FromIterator;

type node_t = usize;
type edge_t = usize;
type weight_t = f64;
type node_type_t = u16;
type edge_type_t = u16;

struct Graph {
    sources: Vec<node_t>,
    destinations: Vec<node_t>,
    nodes_mapping: HashMap<String, node_t>,
    reverse_nodes_mapping: Vec<String>,
    unique_edges: HashSet<(node_t, node_t)>,
    weights: Vec<weight_t>,
    outbound_edges: Vec<edge_t>,
    node_types: Vec<node_type_t>,
    edge_types: Vec<edge_type_t>,
}

impl Graph {
    pub fn new_directed(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Vec<node_type_t>,
        edge_types: Vec<edge_type_t>,
        weights: Vec<edge_type_t>,
    ) -> Graph {
        debug!("Computing nodes to node IDs mapping.");
        let nodes_mapping: HashMap<String, node_t> =
            nodes.iter().zip((0..nodes.len()).iter()).collect();

        debug!("Computing sources node IDs.");
        let sources: Vec<node_t> = sources_names
            .iter()
            .map(|dst| nodes_mapping.get(&dst))
            .collect();

        debug!("Computing destinations node IDs.");
        let destinations: Vec<node_t> = destinations_names
            .iter()
            .map(|dst| nodes_mapping.get(&dst))
            .collect();

        debug!("Computing unique edges.");
        let unique_edges: HashSet<(node_t, node_t)> =
            HashSet::from_iter(sources.iter().zip(destinations.iter()));

        debug!("Computing outbound edges ranges from each node.");
        let outbound_edges: Vec<edge_t> = Graph::compute_outbound_edges(nodes.len(), sources);

        Graph {
            sources: sources,
            destinations: destinations,
            nodes_mapping: nodes_mapping,
            reverse_nodes_mapping: nodes,
            unique_edges: unique_edges,
            outbound_edges: outbound_edges,
            weights: weights,
            node_types: node_types,
            edge_types: edge_types,
        }
    }

    pub fn new_undirected(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Option<Vec<node_type_t>>,
        edge_types: Option<Vec<edge_type_t>>,
        weights: Option<Vec<edge_type_t>>,
    ) -> Graph {
        debug!("Identifying self-loops present in given graph.");
        let loops_mask: Vec<bool> = sources_names
            .iter()
            .zip(destinations_names.iter())
            .map(|a, b| a == b)
            .collect();

        let total_loops: u64 = loops_mask.iter().sum();
        let total_edges: u64 = (sources_names.len() - total_loops) * 2 + total_loops;

        debug!("Building undirected graph sources.");
        let full_sources: Vec<String> = sources_names.clone();
        full_sources.extend(
            sources_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value)
                .collect(),
        );

        debug!("Building undirected graph destinations.");
        let full_destinations: Vec<String> = destinations_names.clone();
        full_destinations.extend(
            destinations_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value)
                .collect(),
        );

        if let Some(e) = &mut edge_types {
            debug!("Building undirected graph edge types.");
            e.extend(
                e.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| value)
                    .collect(),
            );
        }

        if let Some(w) = &mut weights {
            debug!("Building undirected graph weights.");
            w.extend(
                w.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| value)
                    .collect(),
            );
        }

        Graph::new_directed(
            nodes,
            full_sources,
            full_destinations,
            node_types,
            edge_types,
            weights,
        )
    }

    fn compute_outbound_edges(nodes_number: node_t, sources: Vec<node_t>) -> Vec<edge_t> {
        let last_src: node_t = 0;
        let outbound_edges: Vec<edge_t> = Vec::with_capacity(nodes_number);

        for (i, src) in self._sources.iter().enumerate() {
            if last_src != src {
                // Assigning to range instead of single value, so that traps
                // have as delta between previous and next node zero.
                for j in last_src..src {
                    outbound_edges[j] = i
                }
                last_src = src
            }
        }

        // Fix the last nodes foward edges by propagating the last_count because
        // if we haven't already filled the array,
        // all the remaining nodes are traps
        for j in src.. {
            outbound_edges[j] = i + 1
        }
        return outbound_edges;
    }
}
