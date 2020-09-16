#[pymethods]
impl EnsmallenGraph {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        sources: Vec<NodeT>,
        destinations: Vec<NodeT>,
        directed: bool,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let mut graph = Graph::builder(sources, destinations, directed);

        if py_kwargs.is_none() {
            return match graph.build(None) {
                Ok(g) => Ok(EnsmallenGraph { graph: g }),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }
        let kwargs = py_kwargs.unwrap();
        validate_kwargs(
            kwargs,
            &[
                "weights",
                "nodes_mapping",
                "nodes_reverse_mapping",
                "node_types",
                "node_types_mapping",
                "node_types_reverse_mapping",
                "edge_types",
                "edge_types_mapping",
                "edge_types_reverse_mapping",
                "force_conversion_to_undirected",
            ],
        )?;

        let weights = extract_value!(kwargs, "weights", Vec<WeightT>);

        if let Some(w) = weights {
            graph = graph.add_weights(w);
        }

        let nodes_mapping = extract_value!(kwargs, "nodes_mapping", HashMap<String, NodeT>);
        let nodes_reverse_mapping = extract_value!(kwargs, "nodes_reverse_mapping", Vec<String>);
        // check passage consistency
        if !((nodes_mapping.is_some() && nodes_reverse_mapping.is_some())
            || (nodes_mapping.is_none() && nodes_reverse_mapping.is_none()))
        {
            return Err(PyErr::new::<exceptions::ValueError, _>(concat!(
                "You must either pass both nodes_mapping, and nodes_reverse_mapping \n",
                "Or none of them."
            )));
        }
        if let Some(nm) = nodes_mapping {
            if let Some(nrm) = nodes_reverse_mapping {
                graph = graph.add_nodes(
                    nm,
                    nrm,
                    extract_value!(kwargs, "node_types", Vec<NodeTypeT>),
                    extract_value!(kwargs, "node_types_mapping", HashMap<String, NodeTypeT>),
                    extract_value!(kwargs, "node_types_reverse_mapping", Vec<String>),
                );
            }
        }

        let edge_types = extract_value!(kwargs, "edge_types", Vec<EdgeTypeT>);
        let edge_types_mapping =
            extract_value!(kwargs, "edge_types_mapping", HashMap<String, EdgeTypeT>);
        let edge_types_reverse_mapping =
            extract_value!(kwargs, "edge_types_reverse_mapping", Vec<String>);
        // check passage consistency
        if !((edge_types.is_some()
            && edge_types_mapping.is_some()
            && edge_types_reverse_mapping.is_some())
            || (edge_types.is_none()
                && edge_types_mapping.is_none()
                && edge_types_reverse_mapping.is_none()))
        {
            return Err(PyErr::new::<exceptions::ValueError, _>(concat!(
                "You must either pass all edge_types, edge_types_mapping, and edge_types_reverse_mapping \n",
                "Or none of them."
            )));
        }

        if let Some(et) = edge_types {
            if let Some(etm) = edge_types_mapping {
                if let Some(etrm) = edge_types_reverse_mapping {
                    graph = graph.add_edge_types(et, etm, etrm);
                }
            }
        }

        match graph.build(
            kwargs
                .get_item("force_conversion_to_undirected")
                .map(|val| val.extract::<bool>().unwrap()),
        ) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}
