use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn must_be_multigraph(&self) -> PyResult<()> {
        pe!(self.graph.must_be_multigraph())
    }

    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn must_be_undirected(&self) -> PyResult<()> {
        pe!(self.graph.must_be_undirected())
    }

    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have any edge.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn must_have_edges(&self) -> PyResult<()> {
        pe!(self.graph.must_have_edges())
    }

    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have any node.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn must_have_nodes(&self) -> PyResult<()> {
        pe!(self.graph.must_have_nodes())
    }

    #[text_signature = "($self, edge_id)"]
    /// Validates provided edge ID.
    ///
    /// Parameters
    /// --------------
    /// edge_id: int,
    /// 	Edge ID to validate.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_edge_id(&self, edge_id: EdgeT) -> PyResult<EdgeT> {
        pe!(self.graph.validate_edge_id(edge_id))
    }

    #[text_signature = "($self, edge_type_id)"]
    /// Validates provided edge type ID.
    ///
    /// Parameters
    /// --------------
    /// edge_type_id: Union[int, None],
    /// 	edge type ID to validate.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Option<EdgeTypeT>> {
        pe!(self.graph.validate_edge_type_id(edge_type_id))
    }

    #[text_signature = "($self, node_id)"]
    /// Validates provided node ID.
    ///
    /// Parameters
    /// --------------
    /// node_id: int,
    /// 	node ID to validate.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        pe!(self.graph.validate_node_id(node_id))
    }

    #[text_signature = "($self, node_type_id)"]
    /// Validates provided node type ID.
    ///
    /// Parameters
    /// --------------
    /// node_type_id: Union[int, None],
    /// 	Node type ID to validate.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> PyResult<Option<NodeTypeT>> {
        pe!(self.graph.validate_node_type_id(node_type_id))
    }

    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn must_not_be_multigraph(&self) -> PyResult<()> {
        pe!(self.graph.must_not_be_multigraph())
    }

    #[text_signature = "($self, edge_type_ids)"]
    /// Validates provided edge type IDs.
    ///
    /// Parameters
    /// --------------
    /// edge_type_ids: List[Union[int], None],
    /// 	Vector of edge type IDs to validate.
    ///
    ///  # Raises
    ///  * If there are no edge types in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_edge_type_ids(
        &self,
        edge_type_ids: Vec<Option<EdgeTypeT>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self.graph.validate_edge_type_ids(edge_type_ids))
    }

    #[text_signature = "($self, node_type_ids)"]
    /// Validates provided node type IDs.
    ///
    /// Parameters
    /// --------------
    /// node_type_ids: List[Union[int], None],
    /// 	Vector of node type IDs to validate.
    ///
    ///  # Raises
    ///  * If there are no node types in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn validate_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        pe!(self.graph.validate_node_type_ids(node_type_ids))
    }
}
