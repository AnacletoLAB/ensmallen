use super::*;
impl Graph {

	#[text_signature = "($self, one, two)"]
	/// Returns product of degrees of given nodes.
	/// 
	/// Paramenters
	/// --------------
	/// one : int,
	/// 	Integer ID of the first node.
	/// two : int,
	/// 	Integer ID of the second node.
	/// =
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn degrees_product(&self, one : NodeT, two : NodeT) -> PyResult<usize> {
		pe!(self.graph.degrees_product(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// Returns the Jaccard index for the two given nodes.
	/// 
	/// Paramenters
	/// --------------
	/// one : int,
	/// 	Integer ID of the first node.
	/// two : int,
	/// 	Integer ID of the second node.
	/// 
	/// # References
	/// [D. Liben-Nowell, J. Kleinberg.
	/// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn jaccard_index(&self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.jaccard_index(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// Returns the Adamic/Adar Index for the given pair of nodes.
	/// 
	/// Paramenters
	/// --------------
	/// one : int,
	/// 	Integer ID of the first node.
	/// two : int,
	/// 	Integer ID of the second node.
	/// 
	/// # Implementation details
	/// Since the Adamic/Adar Index is only defined for graph not containing
	/// node traps (nodes without any outbound edge) and must support all kind
	/// of graphs, the sinks node are excluded from
	/// the computation because they would result in an infinity.
	/// 
	/// # References
	/// [D. Liben-Nowell, J. Kleinberg.
	/// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn adamic_adar_index(&self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.adamic_adar_index(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// Returns the Resource Allocation Index for the given pair of nodes.
	/// 
	/// Paramenters
	/// --------------
	/// one : int,
	/// 	Integer ID of the first node.
	/// two : int,
	/// 	Integer ID of the second node.
	/// 
	/// # References
	/// [T. Zhou, L. Lu, Y.-C. Zhang.
	/// Predicting missing links via local information.
	/// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
	/// 
	/// # Implementation details
	/// Since the Resource Allocation Index is only defined for graph not
	/// containing node traps (nodes without any outbound edge) and
	/// must support all kind of graphs, the sinks node are excluded from
	/// the computation because they would result in an infinity.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn resource_allocation_index(&self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.resource_allocation_index(one, two))
	}
	
}
