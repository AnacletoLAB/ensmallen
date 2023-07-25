use super::*;
use itertools::Itertools;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * `name`: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.name = Arc::new(name);
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This happens INPLACE, that is edits the current graph instance.
    ///
    /// # Arguments
    /// * `edge_type`: S - The edge type to assing to all the edges.
    ///
    /// # Raises
    /// * If the graph does not have edges.
    /// * If the graph is a multigraph.
    pub fn set_inplace_all_edge_types<S: Into<String>>(&mut self, edge_type: S) -> Result<&Graph> {
        // If the graph does not have edges, it does not make sense to
        // try and set the edge types.
        self.must_have_edges()?;
        // Similarly, setting the edge types of a multigraph would make it
        // collapse to a homogeneous graph, and this operation is not supported
        // with the function set all edge types.
        self.must_not_be_multigraph().map_err(|_| {
            concat!(
                "The method set_all_edge_types does not support multigraphs because ",
                "setting the edge types of all edges to a single one in this type",
                "of graphs will cause a multigraph to collapse to an homogeneous ",
                "graph, leading to multiple undefined behaviours, such as loosing ",
                "the parallel edges that would collapse to one: which one should we keep?\n",
                "You can drop the parallell edges by calling the not INPLACE version ",
                "of this method.\n",
                "Consider that when using the remove method, you will still collapse ",
                "the multigraph to an homogeneous graph, and it will keep the FIRST edge ",
                "of any group of multigraph edges between two given nodes."
            )
            .to_string()
        })?;
        let vocabulary =
            Vocabulary::from_reverse_map(vec![edge_type.into()], "Edge types".to_string())?;
        let edge_types = EdgeTypeVocabulary::from_structs(
            vec![Some(0); self.get_number_of_directed_edges() as usize],
            vocabulary,
        );
        self.edge_types = Arc::new(Some(edge_types));
        Ok(self)
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// # Arguments
    /// * `edge_type`: S - The edge type to assing to all the edges.
    pub fn set_all_edge_types<S: Into<String>>(&self, edge_type: S) -> Result<Graph> {
        let mut graph = self.remove_parallel_edges();
        graph.set_inplace_all_edge_types(edge_type)?;
        Ok(graph)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// * `node_type`: S - The node type to assing to all the nodes.
    pub fn set_inplace_all_node_types<S: Into<String>>(&mut self, node_type: S) -> Result<&Graph> {
        self.must_have_nodes()?;
        let vocabulary =
            Vocabulary::from_reverse_map(vec![node_type.into()], "Node types".to_string())?;
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); self.get_number_of_nodes() as usize],
            vocabulary,
        );
        self.node_types = Arc::new(Some(node_types));
        Ok(self)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// # Arguments
    /// * `node_type`: S - The node type to assing to all the nodes.
    pub fn set_all_node_types<S: Into<String>>(&self, node_type: S) -> Result<Graph> {
        let mut graph = self.clone();
        graph.set_inplace_all_node_types(node_type)?;
        Ok(graph)
    }

    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_ids_to_remove`: Vec<NodeTypeT> - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type ID does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_ids(
        &mut self,
        node_type_ids_to_remove: Vec<NodeTypeT>,
    ) -> Result<&Graph> {
        self.must_have_node_types()?;

        // if the user passed no values, we won't modify the graph so we can
        // return ealry
        if node_type_ids_to_remove.is_empty() {
            return Ok(self);
        }
        // check that the values are in the range of node type ids
        self.validate_node_type_id(node_type_ids_to_remove.iter().max().cloned())?;

        // if there are duplicated nodes it's probably an error
        if node_type_ids_to_remove.len() != node_type_ids_to_remove.iter().unique().count() {
            return Err(
                "In the given vector of node type ids to remove there are duplicated values."
                    .to_string(),
            );
        }

        if let Some(node_types) = Arc::make_mut(&mut self.node_types) {
            // compute the new node ids once the given ones are removed
            // we need this to keep a dense mapping.
            let new_node_type_ids =
                unsafe { node_types.unchecked_remove_values(node_type_ids_to_remove.clone()) };

            // Counter of how many new nodes have unknown type (aka how many nodes we removed)
            let new_unknown_nodes = AtomicU32::new(0);

            // Iter over each node and update its node
            node_types.ids.par_iter_mut().for_each(|node_type_ids| {
                // If the node type of the current node is not unknown
                if let Some(ntis) = node_type_ids.as_mut() {
                    // We remove the given node type if one was given.
                    // For each of the node
                    for node_type_id_to_remove in &node_type_ids_to_remove {
                        if let Some(pos) = ntis.iter().position(|x| *x == *node_type_id_to_remove) {
                            ntis.remove(pos);
                        }
                    }

                    // node type anymore, we replace its empty vector with a None.
                    if ntis.is_empty() {
                        *node_type_ids = None;
                        new_unknown_nodes.fetch_add(1, Ordering::SeqCst);
                        return;
                    }

                    // densify the mapping
                    ntis.iter_mut().for_each(|node_type_id| {
                        if let Some(idx) = new_node_type_ids[*node_type_id as usize] {
                            *node_type_id = idx as NodeTypeT;
                        } else {
                            unreachable!("This should not happen");
                        }
                    }); // If after we have removed the node type the node does not have any
                }
            });

            node_types.unknown_count += new_unknown_nodes.load(Ordering::SeqCst);
        }
        Ok(self)
    }

    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_singleton_node_types(&mut self) -> Result<&mut Graph> {
        self.remove_inplace_node_type_ids(self.get_singleton_node_type_ids()?)?;
        Ok(self)
    }

    /// Assigns inplace given node type id to the nodes with given prefixes.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - The node type ID to assign.
    /// * `node_name_prefixes`: Vec<String> - The node name prefixes to check for.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes_inplace(
        &mut self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> Result<&Graph> {
        if node_name_prefixes.is_empty() {
            return Err("The provided list of node name prefixes is empty!".to_string());
        }

        // check that the values are in the range of node type ids
        self.validate_node_type_id(Some(node_type_id))?;

        let self2 = unsafe { &*(self as *mut Self) };

        if let Some(node_types) = Arc::make_mut(&mut self.node_types) {
            // Counter of how many new nodes have known type (aka how many nodes we addded)
            let new_known_nodes = AtomicU32::new(0);

            // Iter over each node and update its node
            let total_added = node_types
                .ids
                .par_iter_mut()
                .zip(self2.par_iter_node_names())
                .map(|(node_type_ids, node_name)| {
                    if node_name_prefixes
                        .iter()
                        .any(|prefix| node_name.starts_with(prefix))
                    {
                        if node_type_ids.is_none() {
                            let _ = node_type_ids.insert(vec![node_type_id]);
                            new_known_nodes.fetch_add(1, Ordering::SeqCst);
                        } else {
                            node_type_ids.as_mut().map(|value| {
                                value.push(node_type_id);
                                value.sort_unstable();
                            });
                        }
                        1
                    } else {
                        0
                    }
                })
                .sum::<NodeT>();
            node_types.counts[node_type_id as usize] += total_added;
            node_types.unknown_count -= new_known_nodes.load(Ordering::SeqCst);
            node_types.update_min_max_count();
        }
        Ok(self)
    }

    /// Replaces inplace given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to replace with.
    /// * `source_node_type_ids`: &[Option<NodeTypeT>] - Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// * `destination_node_type_ids`: &[Option<NodeTypeT>] - Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids_inplace(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: &[Option<NodeTypeT>],
        destination_node_type_ids: &[Option<NodeTypeT>],
    ) -> Result<&Graph> {
        if source_node_type_ids.is_empty() {
            return Err("The provided list of source node type IDs is empty!".to_string());
        }
        if destination_node_type_ids.is_empty() {
            return Err("The provided list of destination node type IDs is empty!".to_string());
        }

        // check that the values are in the range of node type ids
        self.validate_edge_type_id(Some(edge_type_id))?;
        let source_node_type_ids = self.validate_node_type_ids(source_node_type_ids)?;
        let destination_node_type_ids = self.validate_node_type_ids(destination_node_type_ids)?;
        let count_changes = self
            .iter_unique_edge_type_ids()?
            .map(|_| AtomicI64::new(0))
            .collect::<Vec<_>>();

        let self2 = unsafe { &*(self as *mut Self) };

        if let Some(edge_types) = Arc::make_mut(&mut self.edge_types) {
            // Counter of how many new edges have known type (aka how many edges we addded)
            let new_known_edges = AtomicU64::new(0);

            // Iter over each edge and update its edge
            edge_types
                .ids
                .par_iter_mut()
                .zip(self2.par_iter_directed_edges())
                .for_each(|(old_edge_type_id, (_, src, _, dst, _))| unsafe {
                    let src_node_type_ids = self2.get_unchecked_node_type_ids_from_node_id(src);
                    let dst_node_type_ids = self2.get_unchecked_node_type_ids_from_node_id(dst);
                    let found_source = match src_node_type_ids {
                        Some(src_node_type_ids) => src_node_type_ids.iter().any(|node_type_id| {
                            source_node_type_ids.contains(&Some(*node_type_id))
                        }),
                        None => source_node_type_ids.contains(&None),
                    };
                    let found_destination = match dst_node_type_ids {
                        Some(dst_node_type_ids) => dst_node_type_ids.iter().any(|node_type_id| {
                            destination_node_type_ids.contains(&Some(*node_type_id))
                        }),
                        None => destination_node_type_ids.contains(&None),
                    };

                    let reversed = if self.is_directed() {
                        false
                    } else {
                        let found_source = match dst_node_type_ids {
                            Some(dst_node_type_ids) => {
                                dst_node_type_ids.iter().any(|node_type_id| {
                                    source_node_type_ids.contains(&Some(*node_type_id))
                                })
                            }
                            None => source_node_type_ids.contains(&None),
                        };
                        let found_destination = match src_node_type_ids {
                            Some(src_node_type_ids) => {
                                src_node_type_ids.iter().any(|node_type_id| {
                                    destination_node_type_ids.contains(&Some(*node_type_id))
                                })
                            }
                            None => destination_node_type_ids.contains(&None),
                        };
                        found_source && found_destination
                    };

                    if found_source && found_destination || reversed {
                        count_changes[edge_type_id as usize].fetch_add(1, Ordering::SeqCst);
                        match old_edge_type_id {
                            Some(old_edge_type_id) => {
                                count_changes[*old_edge_type_id as usize]
                                    .fetch_sub(1, Ordering::SeqCst);
                                *old_edge_type_id = edge_type_id;
                            }
                            None => {
                                let _ = old_edge_type_id.insert(edge_type_id);
                                new_known_edges.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                });
            edge_types
                .counts
                .iter_mut()
                .zip(count_changes.iter())
                .for_each(|(count, delta)| {
                    *count = (*count as i64 + delta.load(Ordering::SeqCst)) as EdgeT;
                });
            edge_types.unknown_count -= new_known_edges.load(Ordering::SeqCst);
        }

        // If the current graph is a multi-graph, we check whether we corrupted some
        // of the edges by collapsing them to a single edge. If that happened, we raise
        // an error.
        if self.is_multigraph() {
            self.par_iter_directed_edge_node_ids_and_edge_type_id().zip(self.par_iter_directed_edge_node_ids_and_edge_type_id().skip(1)).map(
                |((_, src1, dst1, edge_type1), (_, src2, dst2, edge_type2))|{
                    if src1 == src2 && dst1 == dst2 && edge_type1 == edge_type2 {
                        Err(format!(
                            concat!(
                                "When replacing the edge type of the MULTIPLE EDGES with source node {src_name} ",
                                "and destination node {dst_name} to edge type {edge_type_name}, ",
                                "we have collapsed the edges with source node {src_name} and ",
                                "destination node {dst_name} to a single edge of type {edge_type_name}. ",
                                "This lead to a corrupted graph data structure, as the graph is a multigraph ",
                                "and it should have multiple edges between the same pair of nodes characterized ",
                                "by different edge types. ",
                            ),
                            src_name=unsafe{self.get_unchecked_node_name_from_node_id(src1)},
                            dst_name=unsafe{self.get_unchecked_node_name_from_node_id(dst1)},
                            edge_type_name=unsafe{self.get_unchecked_edge_type_name_from_edge_type_id(edge_type1).unwrap_or_else(||"Unknown".to_string())},
                        ))
                    } else {
                        Ok(())
                    }
                }
            ).collect::<Result<()>>()?;
        }

        Ok(self)
    }

    /// Replaces given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to replace with.
    /// * `source_node_type_ids`: &[Option<NodeTypeT>] - Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// * `destination_node_type_ids`: &[Option<NodeTypeT>] - Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: &[Option<NodeTypeT>],
        destination_node_type_ids: &[Option<NodeTypeT>],
    ) -> Result<Graph> {
        let mut graph = self.clone();
        graph.replace_edge_type_id_from_edge_node_type_ids_inplace(
            edge_type_id,
            source_node_type_ids,
            destination_node_type_ids,
        )?;
        Ok(graph)
    }

    /// Assigns given node type id to the nodes with given prefixes.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - The node type ID to assign.
    /// * `node_name_prefixes`: Vec<String> - The node name prefixes to check for.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes(
        &self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> Result<Graph> {
        let mut graph = self.clone();
        graph.add_node_type_id_from_node_name_prefixes_inplace(node_type_id, node_name_prefixes)?;
        Ok(graph)
    }

    /// Add node type name to the graph in place.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type name to add.
    ///
    /// # Raises
    /// * If the given node type name already exists in the graph.
    ///
    pub fn add_node_type_name_inplace(&mut self, node_type_name: String) -> Result<NodeTypeT> {
        if let Some(node_types) = Arc::make_mut(&mut self.node_types) {
            node_types.add_node_type_name_inplace(node_type_name)
        } else {
            unreachable!("Something has gone horribly wrong.")
        }
    }

    /// Assigns inplace given node type name to the nodes with given prefixes.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type ID to assign.
    /// * `node_name_prefixes`: Vec<String> - The node name prefixes to check for.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes_inplace(
        &mut self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> Result<&Graph> {
        let node_type_id = match self.get_node_type_id_from_node_type_name(&node_type_name) {
            Ok(node_type_id) => node_type_id,
            Err(_) => self.add_node_type_name_inplace(node_type_name)?,
        };
        self.add_node_type_id_from_node_name_prefixes_inplace(node_type_id, node_name_prefixes)
    }

    /// Add edge type name to the graph in place.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type name to add.
    ///
    /// # Raises
    /// * If the given edge type name already exists in the graph.
    ///
    pub fn add_edge_type_name_inplace(&mut self, edge_type_name: String) -> Result<EdgeTypeT> {
        if let Some(edge_types) = Arc::make_mut(&mut self.edge_types) {
            edge_types.add_edge_type_name_inplace(edge_type_name)
        } else {
            unreachable!("Something has gone horribly wrong.")
        }
    }

    /// Replaces inplace given edge type name to the nodes with given source and destination node type names.
    ///
    /// # Arguments
    /// * `edge_type_name`: String - The edge type name to replace with.
    /// * `source_node_type_names`: &[Option<&str>] - Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// * `destination_node_type_names`: &[Option<&str>] - Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names_inplace(
        &mut self,
        edge_type_name: String,
        source_node_type_names: &[Option<&str>],
        destination_node_type_names: &[Option<&str>],
    ) -> Result<&Graph> {
        let edge_type_id = match self.get_edge_type_id_from_edge_type_name(Some(&edge_type_name)) {
            Ok(edge_type_id) => edge_type_id.unwrap(),
            Err(_) => self.add_edge_type_name_inplace(edge_type_name)?,
        };
        self.replace_edge_type_id_from_edge_node_type_ids_inplace(
            edge_type_id,
            &self.get_node_type_ids_from_node_type_names(source_node_type_names)?,
            &self.get_node_type_ids_from_node_type_names(destination_node_type_names)?,
        )
    }

    /// Replaces given edge type name to the nodes with given source and destination node type names.
    ///
    /// # Arguments
    /// * `edge_type_name`: String - The edge type name to replace with.
    /// * `source_node_type_names`: &[Option<&str>] - Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// * `destination_node_type_names`: &[Option<&str>] - Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names(
        &mut self,
        edge_type_name: String,
        source_node_type_names: &[Option<&str>],
        destination_node_type_names: &[Option<&str>],
    ) -> Result<Graph> {
        let mut graph = self.clone();
        graph.replace_edge_type_name_from_edge_node_type_names_inplace(
            edge_type_name,
            source_node_type_names,
            destination_node_type_names,
        )?;
        Ok(graph)
    }

    /// Assigns given node type name to the nodes with given prefixes.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type ID to assign.
    /// * `node_name_prefixes`: Vec<String> - The node name prefixes to check for.
    ///
    /// # Raises
    /// * If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes(
        &self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> Result<Graph> {
        let mut graph = self.clone();
        let node_type_id = graph.add_node_type_name_inplace(node_type_name)?;
        graph.add_node_type_id_from_node_name_prefixes_inplace(node_type_id, node_name_prefixes)?;
        Ok(graph)
    }

    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_homogeneous_node_types(&mut self) -> Result<&mut Graph> {
        self.remove_inplace_node_type_ids(self.get_homogeneous_node_type_ids()?)?;
        Ok(self)
    }

    /// Remove given edge type ID from all edges.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    /// * If the graph does not have edge types.
    /// * If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_ids(
        &mut self,
        edge_type_ids_to_remove: Vec<EdgeTypeT>,
    ) -> Result<&mut Graph> {
        self.must_have_edge_types()?;
        self.must_not_be_multigraph().map_err(|_| {
            concat!(
                "The method remove_inplace_edge_type_ids does not support multigraphs because ",
                "setting the edge types of all edges to a single one in this type",
                "of graphs will cause a multigraph to collapse to an homogeneous ",
                "graph, leading to multiple undefined behaviours, such as loosing ",
                "the parallel edges that would collapse to one: which one should we keep?\n",
                "You can drop the parallell edges by calling the not INPLACE version ",
                "of this method.\n",
                "Consider that when using the remove method, you will still collapse ",
                "the multigraph to an homogeneous graph, and it will keep the FIRST edge ",
                "of any group of multigraph edges between two given nodes."
            )
            .to_string()
        })?;

        // if the user passed no values, we won't modify the graph so we can
        // return ealry
        if edge_type_ids_to_remove.is_empty() {
            return Ok(self);
        }

        // check that the values are in the range of edge type ids
        self.validate_edge_type_id(edge_type_ids_to_remove.iter().max().cloned())?;

        // if there are duplicated edges it's probably an error
        if edge_type_ids_to_remove.len() != edge_type_ids_to_remove.iter().unique().count() {
            return Err(
                "In the given vector of edge type ids to remove there are duplicated values."
                    .to_string(),
            );
        }

        if let Some(edge_types) = Arc::make_mut(&mut self.edge_types) {
            // compute the new edge ids once the given ones are removed
            // we need this to keep a dense mapping.
            let new_edge_type_ids =
                unsafe { edge_types.unchecked_remove_values(edge_type_ids_to_remove) };

            let new_unknown_edges = AtomicU64::new(0);
            edge_types
                .ids
                .par_iter_mut()
                .for_each(|maybe_edge_type_id| {
                    *maybe_edge_type_id = maybe_edge_type_id.and_then(|x| {
                        new_edge_type_ids[x as usize].map_or_else(
                            || {
                                new_unknown_edges.fetch_add(1, Ordering::SeqCst);
                                None
                            },
                            |x| Some(x as EdgeTypeT),
                        )
                    });
                });
            edge_types.unknown_count += new_unknown_edges.load(Ordering::SeqCst);
        }
        Ok(self)
    }

    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_inplace_singleton_edge_types(&mut self) -> Result<&mut Graph> {
        self.remove_inplace_edge_type_ids(self.get_singleton_edge_type_ids()?)?;
        Ok(self)
    }

    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_names`: Vec<&str> - The node type names to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_names(&mut self, node_type_names: Vec<&str>) -> Result<&Graph> {
        let node_type_ids = node_type_names
            .into_iter()
            .map(|node_type_name| self.get_node_type_id_from_node_type_name(node_type_name))
            .collect::<Result<Vec<NodeTypeT>>>()?;
        self.remove_inplace_node_type_ids(node_type_ids)?;
        Ok(self)
    }

    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type names to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_name(&mut self, node_type_name: &str) -> Result<&Graph> {
        let node_type_id = self.get_node_type_id_from_node_type_name(node_type_name)?;
        self.remove_inplace_node_type_ids(vec![node_type_id])?;
        Ok(self)
    }

    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type ID does not exists in the graph.
    ///
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_type_ids(vec![node_type_id])?;
        Ok(graph)
    }

    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_singleton_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_singleton_node_types()?;
        Ok(graph)
    }

    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_homogeneous_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_homogeneous_node_types()?;
        Ok(graph)
    }

    /// Remove inplace isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_isomorphic_node_types(&mut self) -> Result<&Graph> {
        let node_type_ids_to_remove = self
            .par_iter_isomorphic_node_type_ids_groups()?
            .flat_map(|group| group.into_par_iter().skip(1))
            .collect::<Vec<NodeTypeT>>();
        self.remove_inplace_node_type_ids(node_type_ids_to_remove)?;
        Ok(self)
    }

    /// Remove isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_isomorphic_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_isomorphic_node_types()?;
        Ok(graph)
    }

    /// Remove inplace isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_inplace_isomorphic_edge_types(
        &mut self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<&Graph> {
        let edge_type_ids_to_remove = self
            .par_iter_isomorphic_edge_type_ids_groups(minimum_number_of_edges)?
            .flat_map(|group| group.into_par_iter().skip(1))
            .collect::<Vec<EdgeTypeT>>();
        self.remove_inplace_edge_type_ids(edge_type_ids_to_remove)?;
        Ok(self)
    }

    /// Remove isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_isomorphic_edge_types(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_isomorphic_edge_types(minimum_number_of_edges)?;
        Ok(graph)
    }

    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `node_type_names`: Vec<&str> - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_names(&self, node_type_names: Vec<&str>) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_type_names(node_type_names)?;
        Ok(graph)
    }

    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_name(&self, node_type_name: &str) -> Result<Graph> {
        self.remove_node_type_names(vec![node_type_name])
    }

    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type name does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_name(&mut self, edge_type_name: &str) -> Result<&mut Graph> {
        let edge_type_id = self
            .get_edge_type_id_from_edge_type_name(Some(edge_type_name))?
            .unwrap();
        self.remove_inplace_edge_type_ids(vec![edge_type_id])
    }

    /// Remove given edge type ID from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_type_ids(vec![edge_type_id])?;
        Ok(graph)
    }

    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_singleton_edge_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_singleton_edge_types()?;
        Ok(graph)
    }

    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type name does not exists in the graph.
    ///
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_type_name(edge_type_name)?;
        Ok(graph)
    }

    /// Remove node types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_node_types(&mut self) -> Result<&Graph> {
        self.must_have_node_types()?;
        self.node_types = Arc::new(None);
        Ok(self)
    }

    /// Remove node types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_types()?;
        Ok(graph)
    }

    /// Remove edge types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the graph is a multigraph.
    ///
    pub fn remove_inplace_edge_types(&mut self) -> Result<&Graph> {
        self.must_have_edge_types()?;
        self.must_not_be_multigraph()?;
        self.edge_types = Arc::new(None);
        Ok(self)
    }

    /// Remove edge types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_edge_types(&self) -> Result<Graph> {
        let mut graph = self.remove_parallel_edges();
        assert!(!graph.is_multigraph());
        graph.remove_inplace_edge_types()?;
        Ok(graph)
    }

    /// Remove edge weights from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn remove_inplace_edge_weights(&mut self) -> Result<&Graph> {
        self.must_have_edge_weights()?;
        self.weights = Arc::new(None);
        Ok(self)
    }

    /// Remove edge weights from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn remove_edge_weights(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_weights()?;
        Ok(graph)
    }

    /// Divide edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights_inplace(&mut self, denominator: WeightT) -> Result<()> {
        self.must_have_edge_weights()?;
        unsafe { &mut (*self.cache.get()) }.reset_cached_edge_weights();
        if let Some(edge_weights) = Arc::make_mut(&mut self.weights) {
            edge_weights.par_iter_mut().for_each(|edge_weight| {
                *edge_weight /= denominator;
            });
        }
        Ok(())
    }

    /// Divide edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights(&self, denominator: WeightT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.divide_edge_weights_inplace(denominator)?;
        Ok(graph)
    }

    /// Normalize edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights_inplace(&mut self) -> Result<()> {
        self.divide_edge_weights_inplace(self.get_maximum_edge_weight()?)
    }

    /// Normalize edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.normalize_edge_weights_inplace()?;
        Ok(graph)
    }

    /// Multiply edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights_inplace(&mut self, denominator: WeightT) -> Result<()> {
        self.must_have_edge_weights()?;
        if let Some(edge_weights) = Arc::make_mut(&mut self.weights) {
            edge_weights.par_iter_mut().for_each(|edge_weight| {
                *edge_weight *= denominator;
            });
        }
        Ok(())
    }

    /// Multiply edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights(&self, denominator: WeightT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.divide_edge_weights_inplace(denominator)?;
        Ok(graph)
    }
}
