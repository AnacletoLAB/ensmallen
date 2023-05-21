use super::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
/// HyperBallJaccard models.
enum InnerModel {
    /// HyperBallJaccard model.
    HyperBallJaccard6_5(cpu_models::HyperBallJaccard<6, 5>),
    HyperBallJaccard7_5(cpu_models::HyperBallJaccard<7, 5>),
    HyperBallJaccard8_5(cpu_models::HyperBallJaccard<8, 5>),
    HyperBallJaccard9_5(cpu_models::HyperBallJaccard<9, 5>),
    HyperBallJaccard10_5(cpu_models::HyperBallJaccard<10, 5>),
    HyperBallJaccard11_5(cpu_models::HyperBallJaccard<11, 5>),
    HyperBallJaccard12_5(cpu_models::HyperBallJaccard<12, 5>),
    HyperBallJaccard13_5(cpu_models::HyperBallJaccard<13, 5>),
    HyperBallJaccard14_5(cpu_models::HyperBallJaccard<14, 5>),
    HyperBallJaccard15_5(cpu_models::HyperBallJaccard<15, 5>),
    HyperBallJaccard16_5(cpu_models::HyperBallJaccard<16, 5>),
    HyperBallJaccard6_6(cpu_models::HyperBallJaccard<6, 6>),
    HyperBallJaccard7_6(cpu_models::HyperBallJaccard<7, 6>),
    HyperBallJaccard8_6(cpu_models::HyperBallJaccard<8, 6>),
    HyperBallJaccard9_6(cpu_models::HyperBallJaccard<9, 6>),
    HyperBallJaccard10_6(cpu_models::HyperBallJaccard<10, 6>),
    HyperBallJaccard11_6(cpu_models::HyperBallJaccard<11, 6>),
    HyperBallJaccard12_6(cpu_models::HyperBallJaccard<12, 6>),
    HyperBallJaccard13_6(cpu_models::HyperBallJaccard<13, 6>),
    HyperBallJaccard14_6(cpu_models::HyperBallJaccard<14, 6>),
    HyperBallJaccard15_6(cpu_models::HyperBallJaccard<15, 6>),
    HyperBallJaccard16_6(cpu_models::HyperBallJaccard<16, 6>),
}

impl InnerModel {
    fn new(number_of_hops: Option<usize>, precision: usize, register: usize) -> Result<Self> {
        Ok(match (precision, register) {
            (6, 5) => {
                InnerModel::HyperBallJaccard6_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (7, 5) => {
                InnerModel::HyperBallJaccard7_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (8, 5) => {
                InnerModel::HyperBallJaccard8_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (9, 5) => {
                InnerModel::HyperBallJaccard9_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (10, 5) => {
                InnerModel::HyperBallJaccard10_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (11, 5) => {
                InnerModel::HyperBallJaccard11_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (12, 5) => {
                InnerModel::HyperBallJaccard12_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (13, 5) => {
                InnerModel::HyperBallJaccard13_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (14, 5) => {
                InnerModel::HyperBallJaccard14_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (15, 5) => {
                InnerModel::HyperBallJaccard15_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (16, 5) => {
                InnerModel::HyperBallJaccard16_5(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (6, 6) => {
                InnerModel::HyperBallJaccard6_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (7, 6) => {
                InnerModel::HyperBallJaccard7_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (8, 6) => {
                InnerModel::HyperBallJaccard8_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (9, 6) => {
                InnerModel::HyperBallJaccard9_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (10, 6) => {
                InnerModel::HyperBallJaccard10_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (11, 6) => {
                InnerModel::HyperBallJaccard11_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (12, 6) => {
                InnerModel::HyperBallJaccard12_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (13, 6) => {
                InnerModel::HyperBallJaccard13_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (14, 6) => {
                InnerModel::HyperBallJaccard14_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (15, 6) => {
                InnerModel::HyperBallJaccard15_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            (16, 6) => {
                InnerModel::HyperBallJaccard16_6(cpu_models::HyperBallJaccard::new(number_of_hops)?)
            }
            _ => {
                return Err(format!(
                    "Jaccard HyperBall model with precision {} and register {} not supported.",
                    precision, register
                ))
            }
        })
    }

    /// Fit the HyperBall model to the provided graph.
    ///
    /// Parameters
    /// ------------------------
    /// graph: &Graph
    ///    The graph whose topology is to be learned.
    fn fit(&mut self, graph: &graph::Graph) -> Result<()> {
        match self {
            InnerModel::HyperBallJaccard6_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard7_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard8_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard9_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard10_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard11_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard12_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard13_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard14_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard15_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard16_5(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard6_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard7_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard8_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard9_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard10_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard11_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard12_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard13_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard14_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard15_6(inner) => inner.fit(graph),
            InnerModel::HyperBallJaccard16_6(inner) => inner.fit(graph),
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
    fn get_jaccard(&self, src: usize, dst: usize) -> Result<f32> {
        match self {
            InnerModel::HyperBallJaccard6_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard7_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard8_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard9_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard10_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard11_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard12_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard13_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard14_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard15_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard16_5(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard6_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard7_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard8_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard9_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard10_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard11_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard12_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard13_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard14_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard15_6(inner) => inner.get_jaccard(src, dst),
            InnerModel::HyperBallJaccard16_6(inner) => inner.get_jaccard(src, dst),
        }
    }

    /// Return numpy array with Jaccard coefficients for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: &Graph
    ///    The graph whose Jaccard coefficients are to be computed.
    ///
    fn predict(&self, predictions: &mut [f32], graph: &graph::Graph) -> Result<()> {
        match self {
            InnerModel::HyperBallJaccard6_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard7_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard8_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard9_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard10_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard11_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard12_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard13_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard14_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard15_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard16_5(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard6_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard7_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard8_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard9_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard10_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard11_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard12_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard13_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard14_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard15_6(inner) => inner.predict(predictions, graph),
            InnerModel::HyperBallJaccard16_6(inner) => inner.predict(predictions, graph),
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

/// Jaccard HyperBall model.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(*, number_of_hops, precision, bits)")]
pub struct HyperBallJaccard {
    inner: InnerModel,
}

#[pymethods]
impl HyperBallJaccard {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the HyperBallJaccard model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_hops: int = 1
    ///     The number of hops for the Jaccard. By default, `1`.
    /// precision: int = 6
    ///     The precision of the HyperLogLog counters. By default, `6`.
    /// bits: int = 5
    ///     The number of bits of the HyperLogLog counters. By default, `5`.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<HyperBallJaccard> {
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
impl HyperBallJaccard {
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
    /// Return Jaccard coefficient for the provided edge.
    ///
    /// Parameters
    /// ----------------
    /// src: int
    ///     The source node of the edge.
    /// dst: int
    ///     The destination node of the edge.
    fn get_jaccard(&self, src: usize, dst: usize) -> PyResult<f32> {
        pe!(self.inner.get_jaccard(src, dst))
    }

    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy array with Jaccard coefficients for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose Jaccard coefficients are to be computed.
    fn predict(&self, graph: &Graph) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let predictions = unsafe {
            PyArray1::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize],
                false,
            )
        };
        let predictions_ref = unsafe { predictions.as_slice_mut()? };

        pe!(self.inner.predict(predictions_ref, &graph.inner,))?;

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
        Ok(HyperBallJaccard {
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
        Ok(HyperBallJaccard {
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
