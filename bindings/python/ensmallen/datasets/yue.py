"""Module providing graphs available from Yue.

References
----------
Please cite:

```bib
@article{yue2020graph,
  title={Graph embedding on biomedical networks: methods, applications and evaluations},
  author={Yue, Xiang and Wang, Zhen and Huang, Jingong and Parthasarathy, Srinivasan and Moosavinasab, Soheil and Huang, Yungui and Lin, Simon M and Zhang, Wen and Zhang, Ping and Sun, Huan},
  journal={Bioinformatics},
  volume={36},
  number={4},
  pages={1241--1251},
  year={2020},
  publisher={Oxford University Press}
}
```
"""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def node2vecPPI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return node2vecPPI graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "node2vecPPI", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CTDDDA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return CTDDDA graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "CTDDDA", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DrugBankDDI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return DrugBankDDI graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "DrugBankDDI", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MashupPPI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return MashupPPI graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "MashupPPI", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NDFRTDDA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return NDFRTDDA graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "NDFRTDDA", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ClinTermCOOC(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return ClinTermCOOC graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "ClinTermCOOC", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def StringPPI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return StringPPI graph	

    Parameters
    ----------
    directed = False
        Load as directed or undirected
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will automatically preprocess in Linux and macOS and avoid doing this on Windows.
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
    version = "latest"
        Version to retrieve	
    """
    return AutomaticallyRetrievedGraph(
        "StringPPI", version, "yue", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

