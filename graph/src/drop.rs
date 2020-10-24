use super::*;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;

/// # Drop.
impl Graph {
    /// Returns a **NEW** Graph that have no edge types.
    /// If the Graph have weights, the new unique edge will have a weight that
    /// equal to the sum of all the weights of the edges with same src and dst.
    pub fn drop_edge_types(&self) -> Result<Graph, String> {
        if self.edge_types.is_none() {
            return Err("Cannot drop edge types from a graph without edge types".to_string());
        }

        Graph::build_graph(
            self.get_unique_edges_iter().map(|(src, dst)| {
                Ok((
                    src,
                    dst,
                    None,
                    match self.get_unchecked_link_weights(src, dst) {
                        Some(ws) => Some(ws.iter().sum::<f64>() / ws.len() as f64),
                        None => None,
                    },
                ))
            }),
            self.unique_edges_number,
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
            self.name.clone(),
            false,
        )
    }

    /// Returns a **NEW** Graph that have no weights.
    pub fn drop_weights(&self) -> Result<Graph, String> {
        if self.weights.is_none() {
            return Err("Cannot drop weights from a graph without weights".to_string());
        }

        let mut new = self.clone();
        new.weights = None;
        Ok(new)
    }

    /// Returns a **NEW** Graph that have no nodes types.
    pub fn drop_node_types(&self) -> Result<Graph, String> {
        if self.node_types.is_none() {
            return Err("Cannot drop node types from a graph without node types".to_string());
        }
        let mut new = self.clone();
        new.node_types = None;
        Ok(new)
    }

    /// Returns a **NEW** Graph that have no singletons.
    ///
    /// If the given graph does not have singletons, a cloned one is returned.
    ///
    /// # Arguments
    /// ---------------------
    /// `verbose`: bool - Wether to display a loading bar.
    ///
    pub fn drop_singletons(&self, verbose: bool) -> Result<Graph, String> {
        if !self.has_singletons() {
            return Ok(self.clone());
        }
        let pb_edges = get_loading_bar(
            verbose,
            "Building edges of graph without singletons",
            self.get_edges_number() as usize,
        );
        let pb_nodes = get_loading_bar(
            verbose,
            "Building nodes of graph without singletons",
            self.get_nodes_number() as usize,
        );
        Graph::from_string_sorted(
            self.get_edges_string_quadruples()
                .progress_with(pb_edges)
                .filter_map(|(_, src_name, dst_name, edge_type, weight)| {
                    let src = self.get_node_id(&src_name).unwrap();
                    let dst = self.get_node_id(&dst_name).unwrap();
                    match self.directed || src <= dst {
                        true => Some(Ok((src_name, dst_name, edge_type, weight))),
                        false => None,
                    }
                }),
            Some(
                self.get_nodes_names_iter()
                    .progress_with(pb_nodes)
                    .filter_map(|(node_name, node_type)| {
                        match self.is_singleton_string(&node_name).unwrap() {
                            true => None,
                            false => Some(Ok((node_name, node_type))),
                        }
                    }),
            ),
            self.directed,
            false,
            false,
            self.get_edges_number(),
            self.get_nodes_number() - self.get_singleton_nodes_number(),
            false,
            false,
            false,
            self.name.clone(),
        )
    }
    
    /// Drop all the components that are not connected to interesting
    /// nodes and edges.
    /// 
    /// # Arguments
    /// * `node_names` : Option<Vec<String>> - The name of the nodes of which components to keep
    /// * `node_types` : Option<Vec<String>> - The types of the nodes of which components to keep
    /// * `edge_types` : Option<Vec<String>> - The types of the edges of which components to keep
    pub fn drop_components(
        &self, 
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        verbose: bool,
    ) -> Result<Graph, String> {

        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_components_vector(verbose);

        if let Some(node_ids) = self.get_filter_bitmap(node_names, node_types)?{
            keep_components.extend(node_ids.iter().map(|node_id|{
                components_vector[node_id as usize]
             }));
        }
        
        if let Some(ets) = edge_types {
            let mut edge_types_ids = RoaringBitmap::new();
            edge_types_ids.extend(self.translate_edge_types(ets)?.iter().map(|x| *x as u32));
    
            let pb = get_loading_bar(
                verbose,
                &format!(
                    "Computing which components are to keep for the graph {}",
                    &self.name
                ),
                self.get_edges_number() as usize,
            );

            self.get_edges_triples()
                .progress_with(pb)
                .for_each(|(_, src, dst, edge_type)| {
                    if let Some(et) =  edge_type {
                        if edge_types_ids.contains(et as u32) {
                            keep_components.insert(components_vector[src as usize]);
                            keep_components.insert(components_vector[dst as usize]);
                        }
                    }
                });
        }
        let pb = get_loading_bar(
            verbose,
            &format!(
                "Dropping components for the graph {}",
                &self.name
            ),
            self.get_edges_number() as usize,
        );
        
        Graph::build_graph(
            self.get_edges_quadruples()
            .progress_with(pb)
            .filter_map(|(_, src, dst, edge_type, weight)|{
                match keep_components.contains(components_vector[src as usize]) 
                && 
                keep_components.contains(components_vector[dst as usize]) {
                    true => Some(Ok((src, dst, edge_type, weight))),
                    false => None,
                }
            }),
            self.get_edges_number(),
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            self.name.clone(),
            true
            
        )
    }
}
