use super::*;
use graph::{NodeT, WeightT};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, src)"]
    ///
    /// Parameters
    /// -----------------------
    /// src: int,
    ///     Root of the tree of minimum paths.
    ///
    /// Returns
    /// -----------------------
    /// Tuple with vector of distances and vector of parent nodes.
    fn dijkstra(&self, src: NodeT) -> PyResult<(Vec<WeightT>, Vec<NodeT>)> {
        pe!(self.graph.dijkstra(src))
    }
}
