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
    /// verbose: bool = True,
    ///     Wether to show a loading bar. By default True.
    ///
    /// Returns
    /// -----------------------------
    /// New remapped graph.
    pub fn remap(&self, other: &EnsmallenGraph, verbose: Option<bool>) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .remap(&other.graph, verbose.unwrap_or(true)))?,
        })
    }
}
