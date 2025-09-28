use crate::extract_value;
use crate::pe;
use crate::Graph;
use crate::*;
use graph::WeightT;
use pyo3::exceptions::PyValueError;
use rayon::iter::Empty as ParEmpty;
use std::iter::Empty as SeqEmpty;
trait Dtype {
    const DTYPE: &'static str;
}

impl Dtype for u64 {
    const DTYPE: &'static str = "uint64";
}
impl Dtype for i64 {
    const DTYPE: &'static str = "int64";
}
impl Dtype for f32 {
    const DTYPE: &'static str = "float32";
}
impl Dtype for f64 {
    const DTYPE: &'static str = "float64";
}

macro_rules! get_numeric_column_slice {
    ($df:expr, $col_name:expr, $ty:ty) => {{
        let a_col = $df.get_item($col_name)?;
        let a_values = a_col.getattr("values")?;

        let a_dtype = a_values.getattr("dtype")?;
        let a_dtype_str = a_dtype.str()?;
        let expected_dtype = <$ty>::DTYPE;
        if a_dtype_str.to_str()? != expected_dtype {
            return Err(PyValueError::new_err(format!(
                "Column {} has dtype {} and not `object`",
                $col_name, expected_dtype,
            )));
        }

        let a_values_array: &PyArray1<$ty> = a_values.downcast()?;
        unsafe { a_values_array.as_slice().unwrap() }
    }};
}

macro_rules! get_str_columun_iter {
    ($df:expr, $col_name:expr) => {{
        let name_col = $df.get_item($col_name)?;
        let name_values = name_col.getattr("values")?;
        name_values.iter()?.map(|x| x?.str()?.to_str())
    }};
}

macro_rules! check_df {
    ($py:expr, $df:expr) => {
        let df_type = $df.get_type().name()?;

        let inspect = $py.import("inspect")?;
        let getmodule = inspect.getattr("getmodule")?;
        let module_name = getmodule.call1(($df,))?.str()?.to_str()?;

        if df_type != "DataFrame" || !module_name.contains("pandas") {
            return Err(PyValueError::new_err(format!(
                "Expected pandas.DataFrame but got {}.{}",
                module_name, df_type
            )));
        }
    };
}

#[pymethods]
impl Graph {
    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "(directed, edges_df, *, nodes_df, node_name_column, node_type_column, edge_src_column, edge_dst_column, edge_weight_column, edge_type_column, name)"
    )]
    /// Create a new graph from pandas dataframes.
    ///
    /// # Arguments
    /// * `directed` - Whether the graph is directed or not.
    /// * `edges_df` - The dataframe containing the edges.
    /// * `nodes_df` - The dataframe containing the nodes.
    /// * `node_name_column` - The name of the column containing the node names. Default: "name".
    /// * `node_type_column` - The name of the column containing the node types.
    /// * `edge_src_column` - The name of the column containing the source nodes. Default: "subject".
    /// * `edge_dst_column` - The name of the column containing the destination nodes. Default: "object".
    /// * `edge_weight_column` - The name of the column containing the edge weights.
    /// * `edge_type_column` - The name of the column containing the edge types.
    /// * `name` - The name of the graph. Default: "Graph".
    ///
    /// # Example
    ///
    /// ```python
    /// nodes_df = pd.DataFrame(
    ///     [("a", "user"), ("b", "user"), ("c", "product")],
    ///     columns=["name", "type"],
    /// )
    ///
    /// edges_df = pd.DataFrame(
    ///     [("a", "b", 1.0, "knows"), ("b", "c", 2.0, "bought")],
    ///     columns=["subject", "object", "weight", "predicate"],
    /// )
    ///
    /// graph = Graph::from_pd(
    ///     edges_df,
    ///     nodes_df,
    ///     node_name_column="name",
    ///     node_type_column="type",
    ///     edge_src_column="subject",
    ///     edge_dst_column="object",
    ///     edge_weight_column="weight",
    ///     edge_type_column="predicate",
    ///     directed=True,
    ///     name="graph",
    /// )
    /// ```
    fn from_pd(
        py: Python<'_>,
        directed: bool,
        edges_df: Py<PyAny>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Graph> {
        let edges_df = edges_df.as_ref(py);

        let kwargs = py_kwargs.unwrap_or_else(|| PyDict::new(py));

        pe!(validate_kwargs(
            kwargs,
            &[
                "name",
                "nodes_df",
                "node_name_column",
                "edge_src_column",
                "edge_dst_column",
                "node_type_column",
                "edge_weight_column",
                "edge_type_column",
                "node_types_separator"
            ],
        ))?;

        let name = extract_value!(kwargs, "name", String).unwrap_or_else(|| "Graph".to_string());
        let nodes_df = extract_value!(kwargs, "nodes_df", Py<PyAny>);

        let node_name_column =
            extract_value!(kwargs, "node_name_column", String).unwrap_or("name".to_string());
        let edge_src_column =
            extract_value!(kwargs, "edge_src_column", String).unwrap_or("subject".to_string());
        let edge_dst_column =
            extract_value!(kwargs, "edge_dst_column", String).unwrap_or("object".to_string());

        let node_type_column = extract_value!(kwargs, "node_type_column", String);
        let edge_weight_column = extract_value!(kwargs, "edge_weight_column", String);
        let edge_type_column = extract_value!(kwargs, "edge_type_column", String);
        let node_types_separator = extract_value!(kwargs, "node_types_separator", String);

        let nodes_df = nodes_df.as_ref().map(|x| x.as_ref(py));

        check_df!(py, edges_df);

        let has_node_types = node_type_column.is_some();
        let has_edge_types = edge_type_column.is_some();
        let has_edge_weights = edge_weight_column.is_some();

        let nodes_iterator: Option<
            graph::ItersWrapper<
                Result<(usize, (String, Option<Vec<String>>))>,
                Box<dyn Iterator<Item = Result<(usize, (String, Option<Vec<String>>))>>>,
                ParEmpty<Result<(usize, (String, Option<Vec<String>>))>>,
            >,
        > = if let Some(nodes_df) = nodes_df {
            check_df!(py, nodes_df);

            let nodes_iterator = Box::new(
                get_str_columun_iter!(nodes_df, node_name_column).map(|x| x.unwrap().to_string()),
            );

            Some(graph::ItersWrapper::Sequential(
                if let Some(node_type_column) = node_type_column {
                    let node_type_iterator = get_str_columun_iter!(nodes_df, node_type_column);
                    if let Some(node_types_separator) = node_types_separator {
                        Box::new(
                            nodes_iterator
                                .zip(node_type_iterator.map(move |x| {
                                    Some(
                                        x.unwrap()
                                            .split(&node_types_separator)
                                            .map(|x| x.to_string())
                                            .collect::<Vec<_>>(),
                                    )
                                }))
                                .enumerate()
                                .map(Ok),
                        )
                    } else {
                        Box::new(
                            nodes_iterator
                                .zip(node_type_iterator.map(|x| Some(vec![x.unwrap().to_string()])))
                                .enumerate()
                                .map(Ok),
                        )
                    }
                } else {
                    Box::new(
                        nodes_iterator
                            .zip(std::iter::repeat(None))
                            .enumerate()
                            .map(Ok),
                    )
                },
            ))
        } else {
            None
        };

        let src_iterator = Box::new(
            get_str_columun_iter!(edges_df, edge_src_column).map(|x| x.unwrap().to_string()),
        );
        let dst_iterator = Box::new(
            get_str_columun_iter!(edges_df, edge_dst_column).map(|x| x.unwrap().to_string()),
        );

        let edges_iterator: Box<
            dyn Iterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,
        > = match (edge_type_column, edge_weight_column) {
            (Some(et), Some(ew)) => {
                let et_iter = get_str_columun_iter!(edges_df, et).map(|x| x.unwrap().to_string());
                let ew_iter = get_str_columun_iter!(edges_df, ew).map(|x| x.unwrap().to_string());
                Box::new(
                    src_iterator
                        .zip(dst_iterator)
                        .zip(et_iter)
                        .zip(ew_iter)
                        .map(|(((src, dst), edge_type), weight)| {
                            (
                                src,
                                dst,
                                Some(edge_type),
                                weight.parse::<WeightT>().unwrap(),
                            )
                        })
                        .enumerate()
                        .map(Ok),
                )
            }
            (Some(et), None) => {
                let et_iter = get_str_columun_iter!(edges_df, et).map(|x| x.unwrap().to_string());
                Box::new(
                    src_iterator
                        .zip(dst_iterator)
                        .zip(et_iter)
                        .map(|((src, dst), edge_type)| (src, dst, Some(edge_type), 1.0))
                        .enumerate()
                        .map(Ok),
                )
            }
            (None, Some(ew)) => {
                let ew_iter = get_str_columun_iter!(edges_df, ew).map(|x| x.unwrap().to_string());
                Box::new(
                    src_iterator
                        .zip(dst_iterator)
                        .zip(ew_iter)
                        .map(|((src, dst), weight)| {
                            (src, dst, None, weight.parse::<WeightT>().unwrap())
                        })
                        .enumerate()
                        .map(Ok),
                )
            }
            (None, None) => Box::new(
                src_iterator
                    .zip(dst_iterator)
                    .map(|(src, dst)| (src, dst, None, 1.0))
                    .enumerate()
                    .map(Ok),
            ),
        };

        Ok(pe!(graph::build_graph_from_strings(
            None::<graph::ItersWrapper<_, SeqEmpty<_>, ParEmpty<_>>>, // node_types_iterator
            None,                                                     // number_of_node_types
            Some(false),                                              // numeric_node_type_ids
            None,                                                     // minimum_node_type_id
            has_node_types,                                           // has_node_types
            Some(false),                                              // node_types_list_is_correct
            nodes_iterator,                                           // nodes_iterator
            None,                                                     // number_of_nodes
            false,                                                    // node_list_is_correct
            false,                                                    // numeric_node_ids
            false, // numeric_node_list_node_type_ids
            None,  // minimum_node_id
            None::<graph::ItersWrapper<_, SeqEmpty<_>, ParEmpty<_>>>, // edge_types_iterator
            None,  // number_of_edge_types
            Some(false), // numeric_edge_type_ids
            None,  // minimum_edge_type_id
            has_edge_types, // has_edge_types
            Some(false), // edge_types_list_is_correct
            Some(graph::ItersWrapper::<_, _, ParEmpty<_>>::Sequential(
                edges_iterator
            )),
            has_edge_weights, // has_edge_weights
            directed,         // directed
            Some(false),      // correct
            Some(false),      // complete
            Some(true),       // duplicates
            Some(false),      // sorted
            None,             // number_of_edges
            Some(false),      // numeric_edge_list_node_ids
            Some(false),      // numeric_edge_list_edge_type_ids
            Some(true),       // skip_node_types_if_unavailable
            Some(true),       // skip_edge_types_if_unavailable
            true,             // may_have_singletons
            true,             // may_have_singleton_with_selfloops
            name,             // name
        ))?
        .into())
    }
}
