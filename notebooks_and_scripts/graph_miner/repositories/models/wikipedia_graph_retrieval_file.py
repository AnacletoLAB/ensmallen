def {graph_method_name}(
    directed: bool = False,
    load_nodes: bool = True,
    load_node_types: bool = True,
    keep_nodes_without_descriptions: bool = True,
    keep_nodes_without_categories: bool = True,
    keep_interwikipedia_nodes: bool = True,
    keep_external_nodes: bool = True,
    compute_node_description: bool = False,
    auto_enable_tradeoffs: bool = True,
    sort_tmp_dir: str = None,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = None,
    cache_sys_var: str = "GRAPH_CACHE_DIR",
    version: str = "{default_version}",
    **graph_kwargs
) -> Graph:
    """Return new instance of the {graph_name} graph.

    The graph is automatically retrieved from the {repository_name} repository.{tabbed_description}

    Parameters
    -------------------
    directed: bool = False
        Wether to load the graph as directed or undirected.
        By default false.
    load_nodes: bool = True
        Whether to load the nodes vocabulary or treat the nodes
        simply as a numeric range.
    load_node_types: bool = True
        Whether to load the node types or skip them entirely.
        This feature is only available when the preprocessing is enabled.
    keep_nodes_without_descriptions: bool = True
        Whether to keep the nodes laking a description
    keep_nodes_without_categories: bool = True
        Whether to keep the nodes laking a category
    keep_interwikipedia_nodes: bool = True
        Whether to keep nodes from external wikipedia websites
    keep_external_nodes: bool = True
        Whether to keep nodes from external websites (non wikipedia ones).
    compute_node_description: bool = False
        Whether to compute the node descriptions.
        Note that this will significantly increase the side of the node lists!
    auto_enable_tradeoffs: bool = True
        Whether to enable the Ensmallen time-memory tradeoffs in small graphs
        automatically. By default True, that is, if a graph has less than
        50 million edges. In such use cases the memory expenditure is minimal.
    sort_tmp_dir: str = None
        Which folder to use to store the temporary files needed to sort in 
        parallel the edge list when building the optimal preprocessed file.
        This defaults to the same folder of the edge list when no value is 
        provided.
    verbose: int = 2
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: str = None
        Where to store the downloaded graphs.
        If no path is provided, first we check the system variable
        provided below is set, otherwise we use the directory `graphs`.
    cache_sys_var: str = "GRAPH_CACHE_DIR"
        The system variable with the default graph cache directory.
    version: str = "{default_version}"
        The version of the graph to retrieve.{available_graph_versions}
    graph_kwargs
        Additional graph kwargs.{tabbed_references}
    """
    return WikipediaAutomaticallyRetrievedGraph(
        graph_name="{graph_method_name}",
        version=version,
        directed=directed,
        load_nodes=load_nodes,
        load_node_types=load_node_types,
        keep_nodes_without_descriptions=keep_nodes_without_descriptions,
        keep_nodes_without_categories=keep_nodes_without_categories,
        keep_interwikipedia_nodes=keep_interwikipedia_nodes,
        keep_external_nodes=keep_external_nodes,
        compute_node_description=compute_node_description,
        auto_enable_tradeoffs=auto_enable_tradeoffs,
        sort_tmp_dir=sort_tmp_dir,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_sys_var=cache_sys_var,
        graph_kwargs=graph_kwargs
    )()

