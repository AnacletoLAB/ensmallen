use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, other, verbose)"]
    /// Return graph remapped towards nodes of the given graph.
    ///
    /// Parameters
    /// -----------------------------
    /// other: EnsmallenGraph,
    ///     The graph to remap towards.
    /// verbose: bool,
    ///     Wether to show a loding bar.
    ///
    /// Returns
    /// -----------------------------
    /// New remapped graph.
    pub fn remap(&self, other: &EnsmallenGraph, verbose: bool) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.remap(&other.graph, verbose))?,
        })
    }
}
