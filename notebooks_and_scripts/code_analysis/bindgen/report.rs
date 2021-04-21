use super::*;
impl Graph {

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
