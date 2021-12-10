def {graph_method_name}(
    directed = False, preprocess = True, load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "{default_version}", **kwargs
) -> Graph:
    """Return {graph_name} graph{tabbed_description}

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = True
        Preprocess for optimal load time & memory peak
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
        Load node types
    load_edge_weights = True
        Load edge weights
    auto_enable_tradeoffs = True
        Enable tradeoffs when graph has < 50M edges
    sort_tmp_dir = None
        Path to sorting tmp folder
    verbose = 2
    cache = True
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
        Sys var with cache directory
    version = "{default_version}"
        Version to retrieve{available_graph_versions}{tabbed_references}
    """
    return AutomaticallyRetrievedGraph(
        "{graph_method_name}", version, "{repository_package_name}", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs{callbacks_data}
    )()

