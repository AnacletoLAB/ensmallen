use super::*;
use cpu_models::HyperJaccard as HJ;
use hyperloglog_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
/// HyperJaccard models.
enum InnerModel {
    /// HyperJaccard model.
    /// HJ{precision}_{bits}(HJ<Precision{precision}, {bits}>), {python_macro}
    HJ4_4(HJ<Precision4, 4>), // {python_generated}
    HJ4_5(HJ<Precision4, 5>),   // {python_generated}
    HJ4_6(HJ<Precision4, 6>),   // {python_generated}
    HJ5_4(HJ<Precision5, 4>),   // {python_generated}
    HJ5_5(HJ<Precision5, 5>),   // {python_generated}
    HJ5_6(HJ<Precision5, 6>),   // {python_generated}
    HJ6_4(HJ<Precision6, 4>),   // {python_generated}
    HJ6_5(HJ<Precision6, 5>),   // {python_generated}
    HJ6_6(HJ<Precision6, 6>),   // {python_generated}
    HJ7_4(HJ<Precision7, 4>),   // {python_generated}
    HJ7_5(HJ<Precision7, 5>),   // {python_generated}
    HJ7_6(HJ<Precision7, 6>),   // {python_generated}
    HJ8_4(HJ<Precision8, 4>),   // {python_generated}
    HJ8_5(HJ<Precision8, 5>),   // {python_generated}
    HJ8_6(HJ<Precision8, 6>),   // {python_generated}
    HJ9_4(HJ<Precision9, 4>),   // {python_generated}
    HJ9_5(HJ<Precision9, 5>),   // {python_generated}
    HJ9_6(HJ<Precision9, 6>),   // {python_generated}
    HJ10_4(HJ<Precision10, 4>), // {python_generated}
    HJ10_5(HJ<Precision10, 5>), // {python_generated}
    HJ10_6(HJ<Precision10, 6>), // {python_generated}
    HJ11_4(HJ<Precision11, 4>), // {python_generated}
    HJ11_5(HJ<Precision11, 5>), // {python_generated}
    HJ11_6(HJ<Precision11, 6>), // {python_generated}
    HJ12_4(HJ<Precision12, 4>), // {python_generated}
    HJ12_5(HJ<Precision12, 5>), // {python_generated}
    HJ12_6(HJ<Precision12, 6>), // {python_generated}
    HJ13_4(HJ<Precision13, 4>), // {python_generated}
    HJ13_5(HJ<Precision13, 5>), // {python_generated}
    HJ13_6(HJ<Precision13, 6>), // {python_generated}
    HJ14_4(HJ<Precision14, 4>), // {python_generated}
    HJ14_5(HJ<Precision14, 5>), // {python_generated}
    HJ14_6(HJ<Precision14, 6>), // {python_generated}
    HJ15_4(HJ<Precision15, 4>), // {python_generated}
    HJ15_5(HJ<Precision15, 5>), // {python_generated}
    HJ15_6(HJ<Precision15, 6>), // {python_generated}
    HJ16_4(HJ<Precision16, 4>), // {python_generated}
    HJ16_5(HJ<Precision16, 5>), // {python_generated}
    HJ16_6(HJ<Precision16, 6>), // {python_generated}
}

impl InnerModel {
    fn new(number_of_hops: Option<usize>, precision: usize, bits: usize) -> Result<Self> {
        // Since actually writing the code for the following match would make
        // for very hard to read code, we proceed instead with a Python script.

        match (precision, bits) {
            // ({precision}, {bits}) => Ok(InnerModel::HJ{precision}_{bits}(HJ::new(number_of_hops)?)), {python_macro}
            (4, 4) => Ok(InnerModel::HJ4_4(HJ::new(number_of_hops)?)), // {python_generated}
            (4, 5) => Ok(InnerModel::HJ4_5(HJ::new(number_of_hops)?)), // {python_generated}
            (4, 6) => Ok(InnerModel::HJ4_6(HJ::new(number_of_hops)?)), // {python_generated}
            (5, 4) => Ok(InnerModel::HJ5_4(HJ::new(number_of_hops)?)), // {python_generated}
            (5, 5) => Ok(InnerModel::HJ5_5(HJ::new(number_of_hops)?)), // {python_generated}
            (5, 6) => Ok(InnerModel::HJ5_6(HJ::new(number_of_hops)?)), // {python_generated}
            (6, 4) => Ok(InnerModel::HJ6_4(HJ::new(number_of_hops)?)), // {python_generated}
            (6, 5) => Ok(InnerModel::HJ6_5(HJ::new(number_of_hops)?)), // {python_generated}
            (6, 6) => Ok(InnerModel::HJ6_6(HJ::new(number_of_hops)?)), // {python_generated}
            (7, 4) => Ok(InnerModel::HJ7_4(HJ::new(number_of_hops)?)), // {python_generated}
            (7, 5) => Ok(InnerModel::HJ7_5(HJ::new(number_of_hops)?)), // {python_generated}
            (7, 6) => Ok(InnerModel::HJ7_6(HJ::new(number_of_hops)?)), // {python_generated}
            (8, 4) => Ok(InnerModel::HJ8_4(HJ::new(number_of_hops)?)), // {python_generated}
            (8, 5) => Ok(InnerModel::HJ8_5(HJ::new(number_of_hops)?)), // {python_generated}
            (8, 6) => Ok(InnerModel::HJ8_6(HJ::new(number_of_hops)?)), // {python_generated}
            (9, 4) => Ok(InnerModel::HJ9_4(HJ::new(number_of_hops)?)), // {python_generated}
            (9, 5) => Ok(InnerModel::HJ9_5(HJ::new(number_of_hops)?)), // {python_generated}
            (9, 6) => Ok(InnerModel::HJ9_6(HJ::new(number_of_hops)?)), // {python_generated}
            (10, 4) => Ok(InnerModel::HJ10_4(HJ::new(number_of_hops)?)), // {python_generated}
            (10, 5) => Ok(InnerModel::HJ10_5(HJ::new(number_of_hops)?)), // {python_generated}
            (10, 6) => Ok(InnerModel::HJ10_6(HJ::new(number_of_hops)?)), // {python_generated}
            (11, 4) => Ok(InnerModel::HJ11_4(HJ::new(number_of_hops)?)), // {python_generated}
            (11, 5) => Ok(InnerModel::HJ11_5(HJ::new(number_of_hops)?)), // {python_generated}
            (11, 6) => Ok(InnerModel::HJ11_6(HJ::new(number_of_hops)?)), // {python_generated}
            (12, 4) => Ok(InnerModel::HJ12_4(HJ::new(number_of_hops)?)), // {python_generated}
            (12, 5) => Ok(InnerModel::HJ12_5(HJ::new(number_of_hops)?)), // {python_generated}
            (12, 6) => Ok(InnerModel::HJ12_6(HJ::new(number_of_hops)?)), // {python_generated}
            (13, 4) => Ok(InnerModel::HJ13_4(HJ::new(number_of_hops)?)), // {python_generated}
            (13, 5) => Ok(InnerModel::HJ13_5(HJ::new(number_of_hops)?)), // {python_generated}
            (13, 6) => Ok(InnerModel::HJ13_6(HJ::new(number_of_hops)?)), // {python_generated}
            (14, 4) => Ok(InnerModel::HJ14_4(HJ::new(number_of_hops)?)), // {python_generated}
            (14, 5) => Ok(InnerModel::HJ14_5(HJ::new(number_of_hops)?)), // {python_generated}
            (14, 6) => Ok(InnerModel::HJ14_6(HJ::new(number_of_hops)?)), // {python_generated}
            (15, 4) => Ok(InnerModel::HJ15_4(HJ::new(number_of_hops)?)), // {python_generated}
            (15, 5) => Ok(InnerModel::HJ15_5(HJ::new(number_of_hops)?)), // {python_generated}
            (15, 6) => Ok(InnerModel::HJ15_6(HJ::new(number_of_hops)?)), // {python_generated}
            (16, 4) => Ok(InnerModel::HJ16_4(HJ::new(number_of_hops)?)), // {python_generated}
            (16, 5) => Ok(InnerModel::HJ16_5(HJ::new(number_of_hops)?)), // {python_generated}
            (16, 6) => Ok(InnerModel::HJ16_6(HJ::new(number_of_hops)?)), // {python_generated}
            _ => {
                return Err(format!(
                    concat!(
                        "The HyperJaccard model supports precisions ranging from 4 ",
                        "to 16 and bits ranging from 4 to 6. ",
                        "Provided precision: {}, bits: {}."
                    ),
                    precision, bits
                ))
            }
        }
    }

    /// Fit the HyperBall model to the provided graph.
    ///
    /// Parameters
    /// ------------------------
    /// graph: &Graph
    ///    The graph whose topology is to be learned.
    fn fit(&mut self, graph: &graph::Graph) -> Result<()> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.fit(graph), {python_macro}
            InnerModel::HJ4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.fit(graph), // {python_generated}
        }
    }

    /// Return Jaccard coefficient for the provided edge.
    ///
    /// Parameters
    /// ----------------
    /// src: usize
    ///     The source node of the edge.
    /// dst: usize
    ///     The destination node of the edge.
    fn get_jaccard_from_node_ids(&self, src: usize, dst: usize) -> Result<f32> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.get_jaccard_from_node_ids(src, dst), {python_macro}
            InnerModel::HJ4_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.get_jaccard_from_node_ids(src, dst), // {python_generated}
        }
    }

    /// Return Union coefficient for the provided edge.
    ///
    /// Parameters
    /// ----------------
    /// src: usize
    ///     The source node of the edge.
    /// dst: usize
    ///     The destination node of the edge.
    fn get_union(&self, src: usize, dst: usize) -> Result<f32> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.get_union_cardinality(src, dst), {python_macro}
            InnerModel::HJ4_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.get_union_cardinality(src, dst), // {python_generated}
        }
    }

    /// Returns the estimated neighbourhood cardinality of a given node.
    ///
    /// Parameters
    /// ----------------
    /// node: usize
    ///     The node whose neighbourhood cardinality is to be estimated.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///     If the provided node is not in the graph.
    ///     If the model has not been trained.
    fn get_neighbourhood_cardinality(&self, node: usize) -> Result<f32> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.get_neighbourhood_cardinality(node), {python_macro}
            InnerModel::HJ4_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.get_neighbourhood_cardinality(node), // {python_generated}
        }
    }

    /// Return numpy array with Jaccard coefficients for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// predictions: &mut [f32]
    ///     The array where the predictions will be stored.
    /// graph: &Graph
    ///    The graph whose Jaccard coefficients are to be computed.
    ///
    fn get_jaccard_for_all_edges(
        &self,
        predictions: &mut [f32],
        graph: &graph::Graph,
    ) -> Result<()> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.get_jaccard_for_all_edges(predictions, graph), {python_macro}
            InnerModel::HJ4_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.get_jaccard_for_all_edges(predictions, graph), // {python_generated}
        }
    }

    /// Return numpy array with k-hops Degree for each node in the graph.
    ///
    /// Parameters
    /// ----------------
    /// predictions: &mut [f32]
    ///     The array where the predictions will be stored.
    /// graph: &Graph
    ///    The graph whose Jaccard coefficients are to be computed.
    ///
    fn get_degree_for_all_nodes(
        &self,
        predictions: &mut [f32],
        graph: &graph::Graph,
    ) -> Result<()> {
        match self {
            // InnerModel::HJ{precision}_{bits}(inner) => inner.get_degree_for_all_nodes(predictions, graph), {python_macro}
            InnerModel::HJ4_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ4_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ4_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ5_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ5_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ5_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ6_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ6_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ6_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ7_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ7_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ7_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ8_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ8_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ8_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ9_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ9_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ9_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ10_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ10_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ10_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ11_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ11_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ11_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ12_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ12_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ12_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ13_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ13_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ13_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ14_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ14_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ14_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ15_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ15_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ15_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ16_4(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ16_5(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
            InnerModel::HJ16_6(inner) => inner.get_degree_for_all_nodes(predictions, graph), // {python_generated}
        }
    }

    pub fn dump(&self, path: &str) -> Result<()> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self> {
        serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

/// HyperJaccard model.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(*, number_of_hops, precision, bits)")]
pub struct HyperJaccard {
    inner: InnerModel,
}

#[pymethods]
impl HyperJaccard {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the HyperJaccard model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_hops: int = 1
    ///     The number of hops for the Jaccard. By default, `1`.
    /// precision: int = 6
    ///     The precision of the HyperLogLog counters. By default, `6`.
    ///     The supported values range from `4` to `16`.
    /// bits: int = 5
    ///     The number of bits of the HyperLogLog counters. By default, `5`.
    ///     The supported values range from `4` to `6`.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<HyperJaccard> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["number_of_hops", "precision", "bits"]
        ))?;

        Ok(Self {
            inner: pe!(InnerModel::new(
                extract_value_rust_result!(kwargs, "number_of_hops", usize),
                extract_value_rust_result!(kwargs, "precision", usize).unwrap_or(6),
                extract_value_rust_result!(kwargs, "bits", usize).unwrap_or(5),
            ))?,
        })
    }
}

#[pymethods]
impl HyperJaccard {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Fit the HyperBall model to the provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose topology is to be learned.
    fn fit(&mut self, graph: &Graph) -> PyResult<()> {
        pe!(self.inner.fit(&graph.inner,))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Return estimated Jaccard coefficient for the provided edge.
    ///
    /// Parameters
    /// ----------------
    /// src: int
    ///     The source node of the edge.
    /// dst: int
    ///     The destination node of the edge.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///     If the provided nodes are not in the graph.
    ///     If the model has not been trained.
    fn get_jaccard_from_node_ids(&self, src: usize, dst: usize) -> PyResult<f32> {
        pe!(self.inner.get_jaccard_from_node_ids(src, dst))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Return estimated Union cardinality for the provided edge.
    ///
    /// Parameters
    /// ----------------
    /// src: int
    ///     The source node of the edge.
    /// dst: int
    ///     The destination node of the edge.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///     If the provided nodes are not in the graph.
    ///     If the model has not been trained.
    fn get_union(&self, src: usize, dst: usize) -> PyResult<f32> {
        pe!(self.inner.get_union(src, dst))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, node)")]
    /// Returns the estimated neighbourhood cardinality of a given node.
    ///
    /// Parameters
    /// ----------------
    /// node: int
    ///    The node whose neighbourhood cardinality is to be estimated.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///    If the provided node is not in the graph.
    ///    If the model has not been trained.
    fn get_neighbourhood_cardinality(&self, node: usize) -> PyResult<f32> {
        pe!(self.inner.get_neighbourhood_cardinality(node))
    }

    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy array with Jaccard coefficients for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose Jaccard coefficients are to be computed.
    fn get_jaccard_for_all_edges(&self, graph: &Graph) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let predictions = unsafe {
            PyArray1::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize],
                false,
            )
        };
        let predictions_ref = unsafe { predictions.as_slice_mut()? };

        pe!(self
            .inner
            .get_jaccard_for_all_edges(predictions_ref, &graph.inner,))?;

        Ok(predictions.to_owned())
    }

    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy array with k-hops degree for each node in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose Jaccard coefficients are to be computed.
    fn get_degree_for_all_nodes(&self, graph: &Graph) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let predictions =
            unsafe { PyArray1::new(gil.python(), [graph.get_number_of_nodes() as usize], false) };
        let predictions_ref = unsafe { predictions.as_slice_mut()? };

        pe!(self
            .inner
            .get_degree_for_all_nodes(predictions_ref, &graph.inner,))?;

        Ok(predictions.to_owned())
    }

    #[staticmethod]
    #[pyo3(text_signature = "(path,)")]
    /// Loads model from the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path from where to load the model.
    fn load(path: String) -> PyResult<Self> {
        Ok(HyperJaccard {
            inner: pe!(InnerModel::load(path.as_ref()))?,
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(json,)")]
    /// Loads model from provided JSON string.
    ///
    /// Parameters
    /// ----------------
    /// json: str
    ///     JSON string containing model metadata.
    fn loads(json: String) -> PyResult<Self> {
        Ok(HyperJaccard {
            inner: pe!(InnerModel::loads(json.as_str()))?,
        })
    }

    #[pyo3(text_signature = "(&self, path)")]
    /// Dump model to the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path where to dump the model.
    fn dump(&self, path: String) -> PyResult<()> {
        pe!(self.inner.dump(path.as_ref()))
    }

    #[pyo3(text_signature = "(&self)")]
    /// Dumps model to JSON string.
    fn dumps(&self) -> PyResult<String> {
        pe!(self.inner.dumps())
    }
}
