use super::*;
use graph::{EdgeTypeT, NodeTypeT};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self, name)"]
    /// Set the name of the graph.
    ///
    /// Parameters
    /// -----------------------
    /// name: str,
    ///     Name of the graph.
    fn set_name(&mut self, name: String) {
        self.graph.set_name(name)
    }

    #[text_signature = "($self, edge_type_id)"]
    /// Remove given edge type ID from all edges.
    ///
    ///  If any given edge remains with no edge type, that edge is labeled
    ///  with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// --------------
    /// edge_type_id: int,
    /// 	The edge type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///  * If the given edge type ID does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_type_id(edge_type_id))?,
        }
        .to_owned())
    }

    #[text_signature = "($self, edge_type_name)"]
    /// Remove given edge type name from all edges.
    ///
    ///  If any given edge remains with no edge type, that edge is labeled
    ///  with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// --------------
    /// edge_type_name: str,
    /// 	The edge type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///  * If the given edge type name does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_edge_type_name(&self, edge_type_name: String) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_type_name(&edge_type_name))?,
        }
        .to_owned())
    }

    #[text_signature = "($self)"]
    /// Remove edge types from the graph.
    ///
    ///  Note that the modification does not happen inplace.
    ///
    /// Parameters
    /// --------------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar in the case of a multigraph.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_edge_types(&self, verbose: Option<bool>) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_types(verbose))?,
        }
        .to_owned())
    }

    #[text_signature = "($self)"]
    /// Remove edge weights from the graph.
    ///
    ///  Note that the modification does not happen inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have edge weights.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_edge_weights(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_weights())?,
        }
        .to_owned())
    }

    #[text_signature = "($self, node_type_id)"]
    /// Remove given node type ID from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// --------------
    /// node_type_id: int,
    /// 	The node type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///  * If the given node type ID does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_type_id(node_type_id))?,
        }
        .to_owned())
    }

    #[text_signature = "($self, node_type_name)"]
    /// Remove given node type name from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// --------------
    /// node_type_name: str,
    /// 	The node type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///  * If the given node type name does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_node_type_name(&self, node_type_name: String) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_type_name(&node_type_name))?,
        }
        .to_owned())
    }

    #[text_signature = "($self)"]
    /// Remove node types from the graph.
    ///
    ///  Note that the modification does not happen inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_node_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_types())?,
        }
        .to_owned())
    }

    #[text_signature = "($self, edge_type_id)"]
    /// Remove given edge type ID from all edges.
    ///
    /// Parameters
    /// --------------
    /// edge_type_id: int,
    /// 	The edge type ID to remove.
    ///
    ///  # Raises
    ///  *
    ///  * If the graph does not have edge types.
    ///  * If the given edge type ID does not exists in the graph.
    ///
    ///  TODO!: add support for removal of edge types in the context of multigraphs when the user asks for removing an edge type.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_edge_type_id(&mut self, edge_type_id: EdgeTypeT) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_type_id(edge_type_id))?;
        Ok(())
    }

    #[text_signature = "($self, edge_type_name)"]
    /// Remove given edge type name from all edges.
    ///
    ///  If any given edge remains with no edge type, that edge is labeled
    ///  with edge type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// --------------
    /// edge_type_name: str,
    /// 	The edge type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///  * If the given edge type name does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_edge_type_name(&mut self, edge_type_name: &str) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_type_name(edge_type_name))?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove edge types from the graph.
    ///
    ///  Note that the modification happens inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///  * If the graph is a multigraph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_edge_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_types())?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove edge weights from the graph.
    ///
    ///  Note that the modification happens inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have edge weights.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_edge_weights(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_weights())?;
        Ok(())
    }

    #[text_signature = "($self, node_type_id)"]
    /// Remove given node type ID from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// --------------
    /// node_type_id: int,
    /// 	The node type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///  * If the given node type ID does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_node_type_id(&mut self, node_type_id: NodeTypeT) -> PyResult<()> {
        pe!(self.graph.remove_inplace_node_type_id(node_type_id))?;
        Ok(())
    }

    #[text_signature = "($self, node_type_name)"]
    /// Remove given node type name from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// --------------
    /// node_type_name: str,
    /// 	The node type ID to remove.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///  * If the given node type name does not exists in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_node_type_name(&mut self, node_type_name: String) -> PyResult<()> {
        pe!(self.graph.remove_inplace_node_type_name(&node_type_name))?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove node types from the graph.
    ///
    ///  Note that the modification happens inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_node_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_node_types())?;
        Ok(())
    }

    #[text_signature = "($self, edge_type)"]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    ///  This happens INPLACE, that is edits the current graph instance.
    ///
    /// Parameters
    /// --------------
    /// edge_type: str,
    /// 	The edge type to assing to all the edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn set_inplace_all_edge_types(&mut self, edge_type: String) -> PyResult<()> {
        pe!(self.graph.set_inplace_all_edge_types(edge_type))?;
        Ok(())
    }

    #[text_signature = "($self, node_type)"]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// Parameters
    /// --------------
    /// node_type: str,
    /// 	The node type to assing to all the nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn set_inplace_all_node_types(&mut self, node_type: String) -> PyResult<()> {
        pe!(self.graph.set_inplace_all_node_types(node_type))?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove singleton edge types from all edges.
    ///
    ///  If any given edge remains with no edge type, that edge is labeled
    ///  with edge type None. Note that the modification happens inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_singleton_edge_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_singleton_edge_types())?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove singleton node types from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification happens inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_inplace_singleton_node_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_singleton_node_types())?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Remove singleton edge types from all edges.
    ///
    ///  If any given edge remains with no edge type, that edge is labeled
    ///  with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_singleton_edge_types(&mut self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_singleton_edge_types())?,
        })
    }

    #[text_signature = "($self)"]
    /// Remove singleton node types from all nodes.
    ///
    ///  If any given node remains with no node type, that node is labeled
    ///  with node type None. Note that the modification DOES NOT happen inplace.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn remove_singleton_node_types(&mut self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_singleton_node_types())?,
        })
    }
}
