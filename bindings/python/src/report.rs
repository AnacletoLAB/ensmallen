use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, verbose)"]
    /// Return rendered textual report of the graph.
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show loading bar.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn textual_report(&self, verbose: bool) -> PyResult<String> {
        pe!(self.graph.textual_report(verbose))
    }

    #[text_signature = "($self)"]
    /// Return human-readable markdown report of the graph peculiarities.
    ///
    ///  The report, by default, is rendered using Markdown.
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show a loading bar in graph operations.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_peculiarities_report_markdown(&self) -> String {
        self.graph.get_peculiarities_report_markdown()
    }
}
