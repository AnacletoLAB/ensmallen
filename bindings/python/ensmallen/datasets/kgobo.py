"""Module providing graphs available from KGOBO.

References
----------
Please cite:

```bib
@misc{kgobo,
  title        = "KG-OBO",
  year         = "2021",
  author       = "{Reese, Justin and Caufield, Harry}",
  howpublished = {\\url{https://github.com/Knowledge-Graph-Hub/kg-obo}},
  note = {Online; accessed 14 September 2021}
}
```
"""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def MOD(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "10-03-2021-14-36", **kwargs
) -> Graph:
    """Return MOD graph	

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
    version = "10-03-2021-14-36"
        Version to retrieve	
		The available versions are:
			- 1.031.4
			- 10-03-2021-14-36
    """
    return AutomaticallyRetrievedGraph(
        "MOD", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FBBT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-27", **kwargs
) -> Graph:
    """Return FBBT graph	

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
    version = "2022-01-27"
        Version to retrieve	
		The available versions are:
			- 2022-02-24
			- 2021-09-01
			- 2021-10-14
			- 2021-12-09
			- 2022-01-27
    """
    return AutomaticallyRetrievedGraph(
        "FBBT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def BTO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-04-27", **kwargs
) -> Graph:
    """Return BTO graph	

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
    version = "2021-04-27"
        Version to retrieve	
		The available versions are:
			- 2021-10-26
			- 2021-04-27
    """
    return AutomaticallyRetrievedGraph(
        "BTO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CHMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return CHMO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- 2022-02-17
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "CHMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OBA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-03", **kwargs
) -> Graph:
    """Return OBA graph	

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
    version = "2021-12-03"
        Version to retrieve	
		The available versions are:
			- 2022-01-19
			- 13-11-2015-10-21
			- 2021-12-03
    """
    return AutomaticallyRetrievedGraph(
        "OBA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PSO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-05-19", **kwargs
) -> Graph:
    """Return PSO graph	

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
    version = "2020-05-19"
        Version to retrieve	
		The available versions are:
			- 2020-05-19
    """
    return AutomaticallyRetrievedGraph(
        "PSO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OGSF(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "11-22-2014", **kwargs
) -> Graph:
    """Return OGSF graph	

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
    version = "11-22-2014"
        Version to retrieve	
		The available versions are:
			- 11-22-2014
    """
    return AutomaticallyRetrievedGraph(
        "OGSF", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MCO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-05-15", **kwargs
) -> Graph:
    """Return MCO graph	

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
    version = "2019-05-15"
        Version to retrieve	
		The available versions are:
			- 2019-05-15
    """
    return AutomaticallyRetrievedGraph(
        "MCO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OPMI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "Vision-Release--1.0.130", **kwargs
) -> Graph:
    """Return OPMI graph	

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
    version = "Vision-Release--1.0.130"
        Version to retrieve	
		The available versions are:
			- Vision-Release--1.0.130
    """
    return AutomaticallyRetrievedGraph(
        "OPMI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FBDV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-24", **kwargs
) -> Graph:
    """Return FBDV graph	

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
    version = "2022-01-24"
        Version to retrieve	
		The available versions are:
			- 2022-02-25
			- 2021-09-01
			- 2021-10-13
			- 2021-12-06
			- 2022-01-24
    """
    return AutomaticallyRetrievedGraph(
        "FBDV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CEPH(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2016-01-12", **kwargs
) -> Graph:
    """Return CEPH graph	

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
    version = "2016-01-12"
        Version to retrieve	
		The available versions are:
			- 2016-01-12
    """
    return AutomaticallyRetrievedGraph(
        "CEPH", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MPATH(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-05-19", **kwargs
) -> Graph:
    """Return MPATH graph	

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
    version = "2020-05-19"
        Version to retrieve	
		The available versions are:
			- 2020-05-19
    """
    return AutomaticallyRetrievedGraph(
        "MPATH", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SPD(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "1.0", **kwargs
) -> Graph:
    """Return SPD graph	

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
    version = "1.0"
        Version to retrieve	
		The available versions are:
			- 1.0
    """
    return AutomaticallyRetrievedGraph(
        "SPD", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OMIT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "dev", **kwargs
) -> Graph:
    """Return OMIT graph	

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
    version = "dev"
        Version to retrieve	
		The available versions are:
			- dev
    """
    return AutomaticallyRetrievedGraph(
        "OMIT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def VT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "04-10-2021-10-15", **kwargs
) -> Graph:
    """Return VT graph	

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
    version = "04-10-2021-10-15"
        Version to retrieve	
		The available versions are:
			- 04-10-2021-10-15
    """
    return AutomaticallyRetrievedGraph(
        "VT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EHDAA2(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2013-07-04", **kwargs
) -> Graph:
    """Return EHDAA2 graph	

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
    version = "2013-07-04"
        Version to retrieve	
		The available versions are:
			- 2013-07-04
    """
    return AutomaticallyRetrievedGraph(
        "EHDAA2", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FLOPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return FLOPO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "FLOPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WBLS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-08", **kwargs
) -> Graph:
    """Return WBLS graph	

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
    version = "2021-12-08"
        Version to retrieve	
		The available versions are:
			- 2022-03-22
			- 2021-07-06
			- 2021-12-08
    """
    return AutomaticallyRetrievedGraph(
        "WBLS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def RXNO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-06", **kwargs
) -> Graph:
    """Return RXNO graph	

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
    version = "2021-12-06"
        Version to retrieve	
		The available versions are:
			- 2021-12-16
			- 2021-01-21
			- 2021-11-15
			- 2021-12-06
    """
    return AutomaticallyRetrievedGraph(
        "RXNO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OMP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-08", **kwargs
) -> Graph:
    """Return OMP graph	

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
    version = "2022-02-08"
        Version to retrieve	
		The available versions are:
			- 2022-03-04
			- 2021-10-01
			- 2021-12-03
			- 2022-01-07
			- 2022-02-08
    """
    return AutomaticallyRetrievedGraph(
        "OMP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ERO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return ERO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "ERO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GNO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-13", **kwargs
) -> Graph:
    """Return GNO graph	

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
    version = "2021-08-13"
        Version to retrieve	
		The available versions are:
			- 2022-02-23
			- 2021-08-13
    """
    return AutomaticallyRetrievedGraph(
        "GNO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def XCO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "4.46", **kwargs
) -> Graph:
    """Return XCO graph	

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
    version = "4.46"
        Version to retrieve	
		The available versions are:
			- 4.46
    """
    return AutomaticallyRetrievedGraph(
        "XCO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def AMPHX(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-12-18", **kwargs
) -> Graph:
    """Return AMPHX graph	

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
    version = "2020-12-18"
        Version to retrieve	
		The available versions are:
			- 2020-12-18
    """
    return AutomaticallyRetrievedGraph(
        "AMPHX", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EPIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-05-28", **kwargs
) -> Graph:
    """Return EPIO graph	

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
    version = "2021-05-28"
        Version to retrieve	
		The available versions are:
			- 2021-05-28
    """
    return AutomaticallyRetrievedGraph(
        "EPIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CLYH(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-05-29", **kwargs
) -> Graph:
    """Return CLYH graph	

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
    version = "2020-05-29"
        Version to retrieve	
		The available versions are:
			- 2020-05-29
    """
    return AutomaticallyRetrievedGraph(
        "CLYH", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OOSTT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-01-08", **kwargs
) -> Graph:
    """Return OOSTT graph	

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
    version = "2021-01-08"
        Version to retrieve	
		The available versions are:
			- 2021-01-08
    """
    return AutomaticallyRetrievedGraph(
        "OOSTT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FYPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-18", **kwargs
) -> Graph:
    """Return FYPO graph	

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
    version = "2022-01-18"
        Version to retrieve	
		The available versions are:
			- 2022-01-27
			- 2021-10-05
			- 2021-11-08
			- 2021-11-18
			- 2021-12-07
			- 2022-01-18
    """
    return AutomaticallyRetrievedGraph(
        "FYPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NCRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-12-10", **kwargs
) -> Graph:
    """Return NCRO graph	

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
    version = "2015-12-10"
        Version to retrieve	
		The available versions are:
			- 2015-12-10
    """
    return AutomaticallyRetrievedGraph(
        "NCRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def IAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-12-09", **kwargs
) -> Graph:
    """Return IAO graph	

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
    version = "2020-12-09"
        Version to retrieve	
		The available versions are:
			- 2020-12-09
    """
    return AutomaticallyRetrievedGraph(
        "IAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GEO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "production-version-2016-03-26", **kwargs
) -> Graph:
    """Return GEO graph	

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
    version = "production-version-2016-03-26"
        Version to retrieve	
		The available versions are:
			- production-version-2016-03-26
    """
    return AutomaticallyRetrievedGraph(
        "GEO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EXO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.1", **kwargs
) -> Graph:
    """Return EXO graph	

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
    version = "2.1"
        Version to retrieve	
		The available versions are:
			- 2.1
    """
    return AutomaticallyRetrievedGraph(
        "EXO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SWO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "swo.owl", **kwargs
) -> Graph:
    """Return SWO graph	

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
    version = "swo.owl"
        Version to retrieve	
		The available versions are:
			- swo.owl
    """
    return AutomaticallyRetrievedGraph(
        "SWO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OBCS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018-02-22", **kwargs
) -> Graph:
    """Return OBCS graph	

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
    version = "2018-02-22"
        Version to retrieve	
		The available versions are:
			- 2018-02-22
    """
    return AutomaticallyRetrievedGraph(
        "OBCS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ENVO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-05-14", **kwargs
) -> Graph:
    """Return ENVO graph	

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
    version = "2021-05-14"
        Version to retrieve	
		The available versions are:
			- 2021-05-14
    """
    return AutomaticallyRetrievedGraph(
        "ENVO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SYMP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-08-04", **kwargs
) -> Graph:
    """Return SYMP graph	

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
    version = "2020-08-04"
        Version to retrieve	
		The available versions are:
			- 2020-08-04
    """
    return AutomaticallyRetrievedGraph(
        "SYMP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TAXRANK(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2016-04-15", **kwargs
) -> Graph:
    """Return TAXRANK graph	

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
    version = "2016-04-15"
        Version to retrieve	
		The available versions are:
			- 2016-04-15
    """
    return AutomaticallyRetrievedGraph(
        "TAXRANK", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def APO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-08", **kwargs
) -> Graph:
    """Return APO graph	

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
    version = "2022-02-08"
        Version to retrieve	
		The available versions are:
			- 2022-03-23
			- 2021-09-07
			- 2021-10-07
			- 2022-01-03
			- 2022-02-08
    """
    return AutomaticallyRetrievedGraph(
        "APO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CLO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-02-10", **kwargs
) -> Graph:
    """Return CLO graph	

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
    version = "2019-02-10"
        Version to retrieve	
		The available versions are:
			- 2022-03-20
			- 2019-02-10
    """
    return AutomaticallyRetrievedGraph(
        "CLO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-02-19", **kwargs
) -> Graph:
    """Return CMO graph	

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
    version = "2019-02-19"
        Version to retrieve	
		The available versions are:
			- 2019-02-19
    """
    return AutomaticallyRetrievedGraph(
        "CMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OHMI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-09-17", **kwargs
) -> Graph:
    """Return OHMI graph	

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
    version = "2019-09-17"
        Version to retrieve	
		The available versions are:
			- 2019-09-17
    """
    return AutomaticallyRetrievedGraph(
        "OHMI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HSO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-11-28", **kwargs
) -> Graph:
    """Return HSO graph	

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
    version = "2020-11-28"
        Version to retrieve	
		The available versions are:
			- 2021-12-13
			- 2020-11-28
    """
    return AutomaticallyRetrievedGraph(
        "HSO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FBBI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-11-06", **kwargs
) -> Graph:
    """Return FBBI graph	

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
    version = "2020-11-06"
        Version to retrieve	
		The available versions are:
			- 2020-11-06
    """
    return AutomaticallyRetrievedGraph(
        "FBBI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OBI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-18", **kwargs
) -> Graph:
    """Return OBI graph	

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
    version = "2021-08-18"
        Version to retrieve	
		The available versions are:
			- 2022-01-03
			- 2021-08-18
    """
    return AutomaticallyRetrievedGraph(
        "OBI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CDAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-06-26", **kwargs
) -> Graph:
    """Return CDAO graph	

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
    version = "2019-06-26"
        Version to retrieve	
		The available versions are:
			- 2019-06-26
    """
    return AutomaticallyRetrievedGraph(
        "CDAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MFMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2013-11-16", **kwargs
) -> Graph:
    """Return MFMO graph	

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
    version = "2013-11-16"
        Version to retrieve	
		The available versions are:
			- 2013-11-16
    """
    return AutomaticallyRetrievedGraph(
        "MFMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-12-11", **kwargs
) -> Graph:
    """Return CRO graph	

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
    version = "2019-12-11"
        Version to retrieve	
		The available versions are:
			- 2019-12-11
    """
    return AutomaticallyRetrievedGraph(
        "CRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CHEMINF(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.0", **kwargs
) -> Graph:
    """Return CHEMINF graph	

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
    version = "2.0"
        Version to retrieve	
		The available versions are:
			- 2.0
    """
    return AutomaticallyRetrievedGraph(
        "CHEMINF", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-11-04", **kwargs
) -> Graph:
    """Return MP graph	

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
    version = "2021-11-04"
        Version to retrieve	
		The available versions are:
			- releases
			- 2021-09-21
			- 2021-10-15
			- 2021-10-26
			- 2021-11-04
    """
    return AutomaticallyRetrievedGraph(
        "MP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DUO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-02-23", **kwargs
) -> Graph:
    """Return DUO graph	

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
    version = "2021-02-23"
        Version to retrieve	
		The available versions are:
			- 2021-02-23
    """
    return AutomaticallyRetrievedGraph(
        "DUO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def LABO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-06-08", **kwargs
) -> Graph:
    """Return LABO graph	

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
    version = "2021-06-08"
        Version to retrieve	
		The available versions are:
			- 2021-06-08
    """
    return AutomaticallyRetrievedGraph(
        "LABO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OLATDV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-10", **kwargs
) -> Graph:
    """Return OLATDV graph	

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
    version = "2020-03-10"
        Version to retrieve	
		The available versions are:
			- 2020-03-10
    """
    return AutomaticallyRetrievedGraph(
        "OLATDV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MPIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-01-30", **kwargs
) -> Graph:
    """Return MPIO graph	

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
    version = "2019-01-30"
        Version to retrieve	
		The available versions are:
			- 2019-01-30
    """
    return AutomaticallyRetrievedGraph(
        "MPIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CHEBI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "208", **kwargs
) -> Graph:
    """Return CHEBI graph	

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
    version = "208"
        Version to retrieve	
		The available versions are:
			- 209
			- 203
			- 204
			- 205
			- 206
			- 207
			- 208
    """
    return AutomaticallyRetrievedGraph(
        "CHEBI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def AEO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2014-12-05", **kwargs
) -> Graph:
    """Return AEO graph	

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
    version = "2014-12-05"
        Version to retrieve	
		The available versions are:
			- 2014-12-05
    """
    return AutomaticallyRetrievedGraph(
        "AEO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GENO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-08", **kwargs
) -> Graph:
    """Return GENO graph	

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
    version = "2020-03-08"
        Version to retrieve	
		The available versions are:
			- 2020-03-08
    """
    return AutomaticallyRetrievedGraph(
        "GENO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SBO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "28-08-2021-03-13", **kwargs
) -> Graph:
    """Return SBO graph	

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
    version = "28-08-2021-03-13"
        Version to retrieve	
		The available versions are:
			- 28-08-2021-03-13
    """
    return AutomaticallyRetrievedGraph(
        "SBO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-04-06", **kwargs
) -> Graph:
    """Return TO graph	

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
    version = "2021-04-06"
        Version to retrieve	
		The available versions are:
			- 2022-03-09
			- 2021-04-06
    """
    return AutomaticallyRetrievedGraph(
        "TO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def UO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return UO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "UO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MOP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2014-09-03", **kwargs
) -> Graph:
    """Return MOP graph	

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
    version = "2014-09-03"
        Version to retrieve	
		The available versions are:
			- 2022-02-01
			- 2014-09-03
    """
    return AutomaticallyRetrievedGraph(
        "MOP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CHIRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-11-23", **kwargs
) -> Graph:
    """Return CHIRO graph	

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
    version = "2015-11-23"
        Version to retrieve	
		The available versions are:
			- 2015-11-23
    """
    return AutomaticallyRetrievedGraph(
        "CHIRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OGMS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-19", **kwargs
) -> Graph:
    """Return OGMS graph	

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
    version = "2021-08-19"
        Version to retrieve	
		The available versions are:
			- 2021-08-19
    """
    return AutomaticallyRetrievedGraph(
        "OGMS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NCBITAXON(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-14", **kwargs
) -> Graph:
    """Return NCBITAXON graph	

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
    version = "2021-12-14"
        Version to retrieve	
		The available versions are:
			- 2022-02-21
			- 2021-06-10
			- 2021-12-14
    """
    return AutomaticallyRetrievedGraph(
        "NCBITAXON", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FOODON(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-11-25", **kwargs
) -> Graph:
    """Return FOODON graph	

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
    version = "2021-11-25"
        Version to retrieve	
		The available versions are:
			- 2022-02-01
			- 2021-09-15
			- 2021-11-25
    """
    return AutomaticallyRetrievedGraph(
        "FOODON", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PW(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "7.52", **kwargs
) -> Graph:
    """Return PW graph	

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
    version = "7.52"
        Version to retrieve	
		The available versions are:
			- 7.52
    """
    return AutomaticallyRetrievedGraph(
        "PW", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FOVT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-10-29", **kwargs
) -> Graph:
    """Return FOVT graph	

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
    version = "2021-10-29"
        Version to retrieve	
		The available versions are:
			- 2021-11-10
			- 2021-06-02
			- 2021-10-29
    """
    return AutomaticallyRetrievedGraph(
        "FOVT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def XPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-03-05", **kwargs
) -> Graph:
    """Return XPO graph	

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
    version = "2021-03-05"
        Version to retrieve	
		The available versions are:
			- 2021-03-05
    """
    return AutomaticallyRetrievedGraph(
        "XPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ZFS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-10", **kwargs
) -> Graph:
    """Return ZFS graph	

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
    version = "2020-03-10"
        Version to retrieve	
		The available versions are:
			- 2020-03-10
    """
    return AutomaticallyRetrievedGraph(
        "ZFS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def RS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "6.107", **kwargs
) -> Graph:
    """Return RS graph	

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
    version = "6.107"
        Version to retrieve	
		The available versions are:
			- 6.107
    """
    return AutomaticallyRetrievedGraph(
        "RS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CTO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return CTO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "CTO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-06-08", **kwargs
) -> Graph:
    """Return OMO graph	

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
    version = "2020-06-08"
        Version to retrieve	
		The available versions are:
			- 2020-06-08
    """
    return AutomaticallyRetrievedGraph(
        "OMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FIX(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-04-13", **kwargs
) -> Graph:
    """Return FIX graph	

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
    version = "2020-04-13"
        Version to retrieve	
		The available versions are:
			- 2020-04-13
    """
    return AutomaticallyRetrievedGraph(
        "FIX", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MAMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return MAMO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "MAMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def VTO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-11-13", **kwargs
) -> Graph:
    """Return VTO graph	

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
    version = "2020-11-13"
        Version to retrieve	
		The available versions are:
			- 2020-11-13
    """
    return AutomaticallyRetrievedGraph(
        "VTO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def UBERON(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-11-28", **kwargs
) -> Graph:
    """Return UBERON graph	

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
    version = "2021-11-28"
        Version to retrieve	
		The available versions are:
			- 2022-02-21
			- 2021-10-01
			- 2021-11-28
    """
    return AutomaticallyRetrievedGraph(
        "UBERON", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MFOMD(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-04-26", **kwargs
) -> Graph:
    """Return MFOMD graph	

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
    version = "2020-04-26"
        Version to retrieve	
		The available versions are:
			- 2020-04-26
    """
    return AutomaticallyRetrievedGraph(
        "MFOMD", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def BFO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-08-26", **kwargs
) -> Graph:
    """Return BFO graph	

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
    version = "2019-08-26"
        Version to retrieve	
		The available versions are:
			- 2019-08-26
    """
    return AutomaticallyRetrievedGraph(
        "BFO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HTN(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return HTN graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "HTN", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PORO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2016-09-13", **kwargs
) -> Graph:
    """Return PORO graph	

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
    version = "2016-09-13"
        Version to retrieve	
		The available versions are:
			- 2016-10-06
			- 2016-09-13
    """
    return AutomaticallyRetrievedGraph(
        "PORO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def AISM(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-13", **kwargs
) -> Graph:
    """Return AISM graph	

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
    version = "2021-12-13"
        Version to retrieve	
		The available versions are:
			- 2022-03-17
			- 2021-09-08
			- 2021-11-19
			- 2021-12-13
    """
    return AutomaticallyRetrievedGraph(
        "AISM", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WBBT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-14", **kwargs
) -> Graph:
    """Return WBBT graph	

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
    version = "2021-12-14"
        Version to retrieve	
		The available versions are:
			- 2022-03-22
			- 2021-09-27
			- 2021-12-14
    """
    return AutomaticallyRetrievedGraph(
        "WBBT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return HAO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "HAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-07-12", **kwargs
) -> Graph:
    """Return SO graph	

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
    version = "2021-07-12"
        Version to retrieve	
		The available versions are:
			- 2021-11-22
			- 2021-07-12
    """
    return AutomaticallyRetrievedGraph(
        "SO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def RO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-20", **kwargs
) -> Graph:
    """Return RO graph	

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
    version = "2022-01-20"
        Version to retrieve	
		The available versions are:
			- 2022-02-07
			- 2021-08-31
			- 2021-10-27
			- 2021-12-06
			- 2022-01-20
    """
    return AutomaticallyRetrievedGraph(
        "RO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MONDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-04", **kwargs
) -> Graph:
    """Return MONDO graph	

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
    version = "2022-02-04"
        Version to retrieve	
		The available versions are:
			- 2022-03-01
			- 2021-09-01
			- 2021-10-01
			- 2021-11-01
			- 2021-12-01
			- 2021-12-30
			- 2022-02-04
    """
    return AutomaticallyRetrievedGraph(
        "MONDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DDPHENO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-06-19", **kwargs
) -> Graph:
    """Return DDPHENO graph	

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
    version = "2020-06-19"
        Version to retrieve	
		The available versions are:
			- 2022-01-19
			- 2020-06-19
    """
    return AutomaticallyRetrievedGraph(
        "DDPHENO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def IDOMAL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-03-16", **kwargs
) -> Graph:
    """Return IDOMAL graph	

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
    version = "2015-03-16"
        Version to retrieve	
		The available versions are:
			- 2015-03-16
    """
    return AutomaticallyRetrievedGraph(
        "IDOMAL", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MAXO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-19", **kwargs
) -> Graph:
    """Return MAXO graph	

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
    version = "2021-08-19"
        Version to retrieve	
		The available versions are:
			- 2022-03-23
			- 2021-08-19
    """
    return AutomaticallyRetrievedGraph(
        "MAXO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FBCV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-24", **kwargs
) -> Graph:
    """Return FBCV graph	

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
    version = "2022-01-24"
        Version to retrieve	
		The available versions are:
			- 2022-02-24
			- 2021-09-02
			- 2021-10-18
			- 2021-12-13
			- 2022-01-24
    """
    return AutomaticallyRetrievedGraph(
        "FBCV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TRANS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-08-04", **kwargs
) -> Graph:
    """Return TRANS graph	

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
    version = "2020-08-04"
        Version to retrieve	
		The available versions are:
			- 2020-08-04
    """
    return AutomaticallyRetrievedGraph(
        "TRANS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PSDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-12-04", **kwargs
) -> Graph:
    """Return PSDO graph	

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
    version = "2020-12-04"
        Version to retrieve	
		The available versions are:
			- 2020-12-04
    """
    return AutomaticallyRetrievedGraph(
        "PSDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SCDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-04-15", **kwargs
) -> Graph:
    """Return SCDO graph	

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
    version = "2021-04-15"
        Version to retrieve	
		The available versions are:
			- 2021-04-15
    """
    return AutomaticallyRetrievedGraph(
        "SCDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def LEPAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-11-20", **kwargs
) -> Graph:
    """Return LEPAO graph	

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
    version = "2021-11-20"
        Version to retrieve	
		The available versions are:
			- 2021-11-20
    """
    return AutomaticallyRetrievedGraph(
        "LEPAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ONTONEO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-04-29", **kwargs
) -> Graph:
    """Return ONTONEO graph	

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
    version = "2021-04-29"
        Version to retrieve	
		The available versions are:
			- 2021-04-29
    """
    return AutomaticallyRetrievedGraph(
        "ONTONEO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DRON(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-10-20", **kwargs
) -> Graph:
    """Return DRON graph	

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
    version = "2021-10-20"
        Version to retrieve	
		The available versions are:
			- 2022-01-28
			- 2021-08-12
			- 2021-10-20
    """
    return AutomaticallyRetrievedGraph(
        "DRON", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def RBO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-30", **kwargs
) -> Graph:
    """Return RBO graph	

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
    version = "2021-08-30"
        Version to retrieve	
		The available versions are:
			- 2022-01-14
			- 2021-08-30
    """
    return AutomaticallyRetrievedGraph(
        "RBO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NCIT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-20", **kwargs
) -> Graph:
    """Return NCIT graph	

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
    version = "2021-08-20"
        Version to retrieve	
		The available versions are:
			- 2021-08-20
    """
    return AutomaticallyRetrievedGraph(
        "NCIT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FMA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-04-13", **kwargs
) -> Graph:
    """Return FMA graph	

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
    version = "2020-04-13"
        Version to retrieve	
		The available versions are:
			- 2020-04-13
    """
    return AutomaticallyRetrievedGraph(
        "FMA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def REX(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2017-11-19", **kwargs
) -> Graph:
    """Return REX graph	

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
    version = "2017-11-19"
        Version to retrieve	
		The available versions are:
			- 2017-11-19
    """
    return AutomaticallyRetrievedGraph(
        "REX", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def COB(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-13", **kwargs
) -> Graph:
    """Return COB graph	

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
    version = "2021-09-13"
        Version to retrieve	
		The available versions are:
			- 2021-09-13
    """
    return AutomaticallyRetrievedGraph(
        "COB", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SIBO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-06-15", **kwargs
) -> Graph:
    """Return SIBO graph	

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
    version = "2015-06-15"
        Version to retrieve	
		The available versions are:
			- 2015-06-15
    """
    return AutomaticallyRetrievedGraph(
        "SIBO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PDRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-06-08", **kwargs
) -> Graph:
    """Return PDRO graph	

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
    version = "2021-06-08"
        Version to retrieve	
		The available versions are:
			- 2021-06-08
    """
    return AutomaticallyRetrievedGraph(
        "PDRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OGG(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "12-01-2016", **kwargs
) -> Graph:
    """Return OGG graph	

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
    version = "12-01-2016"
        Version to retrieve	
		The available versions are:
			- 12-01-2016
    """
    return AutomaticallyRetrievedGraph(
        "OGG", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def XLMOD(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return XLMOD graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- 2019-10-28
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "XLMOD", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HANCESTRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-12-18", **kwargs
) -> Graph:
    """Return HANCESTRO graph	

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
    version = "2020-12-18"
        Version to retrieve	
		The available versions are:
			- 2020-12-18
    """
    return AutomaticallyRetrievedGraph(
        "HANCESTRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-13", **kwargs
) -> Graph:
    """Return GO graph	

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
    version = "2022-01-13"
        Version to retrieve	
		The available versions are:
			- 2022-03-10
			- 2021-09-01
			- 2021-10-26
			- 2021-11-16
			- 2021-12-15
			- 2022-01-13
    """
    return AutomaticallyRetrievedGraph(
        "GO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MF(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-21", **kwargs
) -> Graph:
    """Return MF graph	

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
    version = "2021-09-21"
        Version to retrieve	
		The available versions are:
			- 2021-11-17
			- 2021-09-21
    """
    return AutomaticallyRetrievedGraph(
        "MF", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GSSO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.0.5", **kwargs
) -> Graph:
    """Return GSSO graph	

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
    version = "2.0.5"
        Version to retrieve	
		The available versions are:
			- 2.0.5
    """
    return AutomaticallyRetrievedGraph(
        "GSSO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def UPHENO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return UPHENO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "UPHENO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PLANA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-10-21", **kwargs
) -> Graph:
    """Return PLANA graph	

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
    version = "2021-10-21"
        Version to retrieve	
		The available versions are:
			- releases
			- 2021-09-29
			- 2021-10-06
			- 2021-10-21
    """
    return AutomaticallyRetrievedGraph(
        "PLANA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OAE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "1.2.44", **kwargs
) -> Graph:
    """Return OAE graph	

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
    version = "1.2.44"
        Version to retrieve	
		The available versions are:
			- 1.2.44
    """
    return AutomaticallyRetrievedGraph(
        "OAE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MMUSDV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-10", **kwargs
) -> Graph:
    """Return MMUSDV graph	

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
    version = "2020-03-10"
        Version to retrieve	
		The available versions are:
			- 2020-03-10
    """
    return AutomaticallyRetrievedGraph(
        "MMUSDV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "4.1.71", **kwargs
) -> Graph:
    """Return MS graph	

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
    version = "4.1.71"
        Version to retrieve	
		The available versions are:
			- 4.1.78
			- 4.1.35
			- 4.1.62
			- 4.1.64
			- 4.1.65
			- 4.1.67
			- 4.1.69
			- 4.1.70
			- 4.1.71
    """
    return AutomaticallyRetrievedGraph(
        "MS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def APOLLO_SV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "v4.1.1.", **kwargs
) -> Graph:
    """Return APOLLO_SV graph	

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
    version = "v4.1.1."
        Version to retrieve	
		The available versions are:
			- v4.1.1.
    """
    return AutomaticallyRetrievedGraph(
        "APOLLO_SV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HSAPDV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-10", **kwargs
) -> Graph:
    """Return HSAPDV graph	

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
    version = "2020-03-10"
        Version to retrieve	
		The available versions are:
			- 2020-03-10
    """
    return AutomaticallyRetrievedGraph(
        "HSAPDV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def VO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "September_18__2021", **kwargs
) -> Graph:
    """Return VO graph	

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
    version = "September_18__2021"
        Version to retrieve	
		The available versions are:
			- September_18__2021
    """
    return AutomaticallyRetrievedGraph(
        "VO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MIRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2014-05-14", **kwargs
) -> Graph:
    """Return MIRO graph	

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
    version = "2014-05-14"
        Version to retrieve	
		The available versions are:
			- 2014-05-14
    """
    return AutomaticallyRetrievedGraph(
        "MIRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EMAPA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-01", **kwargs
) -> Graph:
    """Return EMAPA graph	

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
    version = "2021-09-01"
        Version to retrieve	
		The available versions are:
			- 2021-09-01
    """
    return AutomaticallyRetrievedGraph(
        "EMAPA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GECKO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-01-18", **kwargs
) -> Graph:
    """Return GECKO graph	

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
    version = "2021-01-18"
        Version to retrieve	
		The available versions are:
			- 2021-01-18
    """
    return AutomaticallyRetrievedGraph(
        "GECKO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CARO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-18", **kwargs
) -> Graph:
    """Return CARO graph	

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
    version = "2022-02-18"
        Version to retrieve	
		The available versions are:
			- 2022-02-18
    """
    return AutomaticallyRetrievedGraph(
        "CARO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GENEPIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-26", **kwargs
) -> Graph:
    """Return GENEPIO graph	

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
    version = "2022-01-26"
        Version to retrieve	
		The available versions are:
			- 2022-02-06
			- 2021-05-24
			- 2022-01-26
    """
    return AutomaticallyRetrievedGraph(
        "GENEPIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TADS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-08-20", **kwargs
) -> Graph:
    """Return TADS graph	

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
    version = "2015-08-20"
        Version to retrieve	
		The available versions are:
			- 2015-08-20
    """
    return AutomaticallyRetrievedGraph(
        "TADS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def FAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-05-07", **kwargs
) -> Graph:
    """Return FAO graph	

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
    version = "2020-05-07"
        Version to retrieve	
		The available versions are:
			- 2020-05-07
    """
    return AutomaticallyRetrievedGraph(
        "FAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CVDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-05", **kwargs
) -> Graph:
    """Return CVDO graph	

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
    version = "2020-03-05"
        Version to retrieve	
		The available versions are:
			- 2020-03-05
    """
    return AutomaticallyRetrievedGraph(
        "CVDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ECAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-05-22", **kwargs
) -> Graph:
    """Return ECAO graph	

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
    version = "2020-05-22"
        Version to retrieve	
		The available versions are:
			- 2020-05-22
    """
    return AutomaticallyRetrievedGraph(
        "ECAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OHPI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "releases", **kwargs
) -> Graph:
    """Return OHPI graph	

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
    version = "releases"
        Version to retrieve	
		The available versions are:
			- releases
    """
    return AutomaticallyRetrievedGraph(
        "OHPI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OPL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-01-28", **kwargs
) -> Graph:
    """Return OPL graph	

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
    version = "2021-01-28"
        Version to retrieve	
		The available versions are:
			- 2021-01-28
    """
    return AutomaticallyRetrievedGraph(
        "OPL", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TGMA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2013-06-03", **kwargs
) -> Graph:
    """Return TGMA graph	

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
    version = "2013-06-03"
        Version to retrieve	
		The available versions are:
			- 2013-06-03
    """
    return AutomaticallyRetrievedGraph(
        "TGMA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def BCO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-27", **kwargs
) -> Graph:
    """Return BCO graph	

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
    version = "2020-03-27"
        Version to retrieve	
		The available versions are:
			- 2021-11-14
			- 2020-03-27
    """
    return AutomaticallyRetrievedGraph(
        "BCO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ICO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-04-21", **kwargs
) -> Graph:
    """Return ICO graph	

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
    version = "2021-04-21"
        Version to retrieve	
		The available versions are:
			- 2021-04-21
    """
    return AutomaticallyRetrievedGraph(
        "ICO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ZECO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-06-04", **kwargs
) -> Graph:
    """Return ZECO graph	

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
    version = "2021-06-04"
        Version to retrieve	
		The available versions are:
			- 2022-02-14
			- 2021-06-04
    """
    return AutomaticallyRetrievedGraph(
        "ZECO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PHIPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-07-14", **kwargs
) -> Graph:
    """Return PHIPO graph	

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
    version = "2021-07-14"
        Version to retrieve	
		The available versions are:
			- 2021-07-14
    """
    return AutomaticallyRetrievedGraph(
        "PHIPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PDUMDV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-10", **kwargs
) -> Graph:
    """Return PDUMDV graph	

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
    version = "2020-03-10"
        Version to retrieve	
		The available versions are:
			- 2020-03-10
    """
    return AutomaticallyRetrievedGraph(
        "PDUMDV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ARO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "05-10-2021-09-37", **kwargs
) -> Graph:
    """Return ARO graph	

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
    version = "05-10-2021-09-37"
        Version to retrieve	
		The available versions are:
			- 05-10-2021-09-37
    """
    return AutomaticallyRetrievedGraph(
        "ARO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OARCS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return OARCS graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "OARCS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PCL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-09", **kwargs
) -> Graph:
    """Return PCL graph	

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
    version = "2022-02-09"
        Version to retrieve	
		The available versions are:
			- 2022-03-02
			- 2022-01-24
			- 2022-02-02
			- 2022-02-09
    """
    return AutomaticallyRetrievedGraph(
        "PCL", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CTENO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2016-10-19", **kwargs
) -> Graph:
    """Return CTENO graph	

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
    version = "2016-10-19"
        Version to retrieve	
		The available versions are:
			- 2016-10-19
    """
    return AutomaticallyRetrievedGraph(
        "CTENO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PLANP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-28", **kwargs
) -> Graph:
    """Return PLANP graph	

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
    version = "2020-03-28"
        Version to retrieve	
		The available versions are:
			- 2020-03-28
    """
    return AutomaticallyRetrievedGraph(
        "PLANP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DOID(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-21", **kwargs
) -> Graph:
    """Return DOID graph	

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
    version = "2022-02-21"
        Version to retrieve	
		The available versions are:
			- 2022-03-02
			- 2021-10-01
			- 2021-10-12
			- 2021-11-17
			- 2021-12-15
			- 2022-01-31
			- 2022-02-21
    """
    return AutomaticallyRetrievedGraph(
        "DOID", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OMRSE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-30", **kwargs
) -> Graph:
    """Return OMRSE graph	

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
    version = "2021-08-30"
        Version to retrieve	
		The available versions are:
			- 2021-08-30
    """
    return AutomaticallyRetrievedGraph(
        "OMRSE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018-10-26", **kwargs
) -> Graph:
    """Return PPO graph	

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
    version = "2018-10-26"
        Version to retrieve	
		The available versions are:
			- 2018-10-26
    """
    return AutomaticallyRetrievedGraph(
        "PPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OVAE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "04-11-2016", **kwargs
) -> Graph:
    """Return OVAE graph	

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
    version = "04-11-2016"
        Version to retrieve	
		The available versions are:
			- 04-11-2016
    """
    return AutomaticallyRetrievedGraph(
        "OVAE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ZP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-12", **kwargs
) -> Graph:
    """Return ZP graph	

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
    version = "2021-12-12"
        Version to retrieve	
		The available versions are:
			- 2021-12-12
    """
    return AutomaticallyRetrievedGraph(
        "ZP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def STATO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "RC1.4", **kwargs
) -> Graph:
    """Return STATO graph	

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
    version = "RC1.4"
        Version to retrieve	
		The available versions are:
			- RC1.4
    """
    return AutomaticallyRetrievedGraph(
        "STATO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ONE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return ONE graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "ONE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ECTO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-25", **kwargs
) -> Graph:
    """Return ECTO graph	

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
    version = "2021-08-25"
        Version to retrieve	
		The available versions are:
			- 2022-03-09
			- 2021-08-25
    """
    return AutomaticallyRetrievedGraph(
        "ECTO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def XAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-03-04", **kwargs
) -> Graph:
    """Return XAO graph	

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
    version = "2021-03-04"
        Version to retrieve	
		The available versions are:
			- 2021-03-04
    """
    return AutomaticallyRetrievedGraph(
        "XAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MIAPA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return MIAPA graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "MIAPA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MI(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-04-13", **kwargs
) -> Graph:
    """Return MI graph	

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
    version = "2020-04-13"
        Version to retrieve	
		The available versions are:
			- 2020-04-13
    """
    return AutomaticallyRetrievedGraph(
        "MI", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ECOCORE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-02-17", **kwargs
) -> Graph:
    """Return ECOCORE graph	

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
    version = "2021-02-17"
        Version to retrieve	
		The available versions are:
			- 2022-03-09
			- 2021-02-17
    """
    return AutomaticallyRetrievedGraph(
        "ECOCORE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MMO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.39", **kwargs
) -> Graph:
    """Return MMO graph	

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
    version = "2.39"
        Version to retrieve	
		The available versions are:
			- 2.39
    """
    return AutomaticallyRetrievedGraph(
        "MMO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EUPATH(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-09", **kwargs
) -> Graph:
    """Return EUPATH graph	

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
    version = "2021-09-09"
        Version to retrieve	
		The available versions are:
			- 2022-02-15
			- 2021-09-09
    """
    return AutomaticallyRetrievedGraph(
        "EUPATH", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OBIB(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-02-02", **kwargs
) -> Graph:
    """Return OBIB graph	

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
    version = "2021-02-02"
        Version to retrieve	
		The available versions are:
			- 2021-11-12
			- 2021-02-02
    """
    return AutomaticallyRetrievedGraph(
        "OBIB", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def IDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2017-11-03", **kwargs
) -> Graph:
    """Return IDO graph	

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
    version = "2017-11-03"
        Version to retrieve	
		The available versions are:
			- 2017-11-03
    """
    return AutomaticallyRetrievedGraph(
        "IDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def SEPIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return SEPIO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "SEPIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TTO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "19-07-2012-13-26", **kwargs
) -> Graph:
    """Return TTO graph	

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
    version = "19-07-2012-13-26"
        Version to retrieve	
		The available versions are:
			- 19-07-2012-13-26
    """
    return AutomaticallyRetrievedGraph(
        "TTO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PR(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "64.0", **kwargs
) -> Graph:
    """Return PR graph	

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
    version = "64.0"
        Version to retrieve	
		The available versions are:
			- 65.0
			- 63.0
			- 64.0
    """
    return AutomaticallyRetrievedGraph(
        "PR", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NBO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-02-15", **kwargs
) -> Graph:
    """Return NBO graph	

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
    version = "2021-02-15"
        Version to retrieve	
		The available versions are:
			- 2021-02-15
    """
    return AutomaticallyRetrievedGraph(
        "NBO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WBPHENOTYPE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-20", **kwargs
) -> Graph:
    """Return WBPHENOTYPE graph	

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
    version = "2021-12-20"
        Version to retrieve	
		The available versions are:
			- 2022-03-22
			- 2021-09-27
			- 2021-10-25
			- 2021-12-12
			- 2021-12-20
    """
    return AutomaticallyRetrievedGraph(
        "WBPHENOTYPE", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PECO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-08-21", **kwargs
) -> Graph:
    """Return PECO graph	

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
    version = "2020-08-21"
        Version to retrieve	
		The available versions are:
			- 2020-08-21
    """
    return AutomaticallyRetrievedGraph(
        "PECO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GAZ(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return GAZ graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "GAZ", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-03-10", **kwargs
) -> Graph:
    """Return CIO graph	

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
    version = "2015-03-10"
        Version to retrieve	
		The available versions are:
			- 2015-03-10
    """
    return AutomaticallyRetrievedGraph(
        "CIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def INO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "1.0.112", **kwargs
) -> Graph:
    """Return INO graph	

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
    version = "1.0.112"
        Version to retrieve	
		The available versions are:
			- 1.0.112
    """
    return AutomaticallyRetrievedGraph(
        "INO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CLAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-27", **kwargs
) -> Graph:
    """Return CLAO graph	

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
    version = "2021-09-27"
        Version to retrieve	
		The available versions are:
			- 2021-09-27
    """
    return AutomaticallyRetrievedGraph(
        "CLAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def UPA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018-12-12", **kwargs
) -> Graph:
    """Return UPA graph	

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
    version = "2018-12-12"
        Version to retrieve	
		The available versions are:
			- 2018-12-12
    """
    return AutomaticallyRetrievedGraph(
        "UPA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def NOMEN(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return NOMEN graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "NOMEN", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ZFA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-28", **kwargs
) -> Graph:
    """Return ZFA graph	

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
    version = "2022-02-28"
        Version to retrieve	
		The available versions are:
			- 2022-03-15
			- 2020-04-14
			- 2021-12-09
			- 2022-02-15
			- 2022-02-28
    """
    return AutomaticallyRetrievedGraph(
        "ZFA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DISDRIV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return DISDRIV graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "DISDRIV", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CIDO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "09-03-2021", **kwargs
) -> Graph:
    """Return CIDO graph	

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
    version = "09-03-2021"
        Version to retrieve	
		The available versions are:
			- 09-03-2021
    """
    return AutomaticallyRetrievedGraph(
        "CIDO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def COLAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-12-14", **kwargs
) -> Graph:
    """Return COLAO graph	

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
    version = "2021-12-14"
        Version to retrieve	
		The available versions are:
			- 2021-12-14
    """
    return AutomaticallyRetrievedGraph(
        "COLAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def KISAO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.30", **kwargs
) -> Graph:
    """Return KISAO graph	

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
    version = "2.30"
        Version to retrieve	
		The available versions are:
			- 2.30
    """
    return AutomaticallyRetrievedGraph(
        "KISAO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MA(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2017-02-07", **kwargs
) -> Graph:
    """Return MA graph	

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
    version = "2017-02-07"
        Version to retrieve	
		The available versions are:
			- 2017-02-07
    """
    return AutomaticallyRetrievedGraph(
        "MA", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-08-13", **kwargs
) -> Graph:
    """Return PO graph	

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
    version = "2021-08-13"
        Version to retrieve	
		The available versions are:
			- 2021-08-13
    """
    return AutomaticallyRetrievedGraph(
        "PO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CDNO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-10-20", **kwargs
) -> Graph:
    """Return CDNO graph	

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
    version = "2021-10-20"
        Version to retrieve	
		The available versions are:
			- 2022-01-10
			- 2021-10-20
    """
    return AutomaticallyRetrievedGraph(
        "CDNO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ONS(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "1.2.2", **kwargs
) -> Graph:
    """Return ONS graph	

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
    version = "1.2.2"
        Version to retrieve	
		The available versions are:
			- no_version
			- 1.2.2
    """
    return AutomaticallyRetrievedGraph(
        "ONS", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def OHD(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2016-06-27", **kwargs
) -> Graph:
    """Return OHD graph	

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
    version = "2016-06-27"
        Version to retrieve	
		The available versions are:
			- 2016-06-27
    """
    return AutomaticallyRetrievedGraph(
        "OHD", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def VARIO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "no_version", **kwargs
) -> Graph:
    """Return VARIO graph	

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
    version = "no_version"
        Version to retrieve	
		The available versions are:
			- no_version
    """
    return AutomaticallyRetrievedGraph(
        "VARIO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def AGRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-07-01", **kwargs
) -> Graph:
    """Return AGRO graph	

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
    version = "2021-07-01"
        Version to retrieve	
		The available versions are:
			- 2021-11-05
			- 2021-07-01
    """
    return AutomaticallyRetrievedGraph(
        "AGRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DIDEO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-06-11", **kwargs
) -> Graph:
    """Return DIDEO graph	

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
    version = "2021-06-11"
        Version to retrieve	
		The available versions are:
			- 2021-06-11
    """
    return AutomaticallyRetrievedGraph(
        "DIDEO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def TXPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-03-03", **kwargs
) -> Graph:
    """Return TXPO graph	

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
    version = "2020-03-03"
        Version to retrieve	
		The available versions are:
			- 2020-03-03
    """
    return AutomaticallyRetrievedGraph(
        "TXPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PATO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-02-08", **kwargs
) -> Graph:
    """Return PATO graph	

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
    version = "2022-02-08"
        Version to retrieve	
		The available versions are:
			- 2022-02-20
			- 2021-09-09
			- 2021-11-05
			- 2021-12-03
			- 2022-01-12
			- 2022-02-08
    """
    return AutomaticallyRetrievedGraph(
        "PATO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HOM(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2015-01-07", **kwargs
) -> Graph:
    """Return HOM graph	

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
    version = "2015-01-07"
        Version to retrieve	
		The available versions are:
			- 2015-01-07
    """
    return AutomaticallyRetrievedGraph(
        "HOM", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ECO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-04", **kwargs
) -> Graph:
    """Return ECO graph	

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
    version = "2022-01-04"
        Version to retrieve	
		The available versions are:
			- 2022-02-09
			- 2021-10-20
			- 2021-12-03
			- 2022-01-04
    """
    return AutomaticallyRetrievedGraph(
        "ECO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ICEO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2.1", **kwargs
) -> Graph:
    """Return ICEO graph	

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
    version = "2.1"
        Version to retrieve	
		The available versions are:
			- 2.1
    """
    return AutomaticallyRetrievedGraph(
        "ICEO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DDANAT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2020-04-13", **kwargs
) -> Graph:
    """Return DDANAT graph	

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
    version = "2020-04-13"
        Version to retrieve	
		The available versions are:
			- 2020-04-13
    """
    return AutomaticallyRetrievedGraph(
        "DDANAT", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def BSPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-22", **kwargs
) -> Graph:
    """Return BSPO graph	

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
    version = "2021-09-22"
        Version to retrieve	
		The available versions are:
			- 2021-10-13
			- 2021-09-22
    """
    return AutomaticallyRetrievedGraph(
        "BSPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MRO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-21", **kwargs
) -> Graph:
    """Return MRO graph	

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
    version = "2022-01-21"
        Version to retrieve	
		The available versions are:
			- 2022-03-14
			- 2021-09-24
			- 2021-10-15
			- 2021-11-04
			- 2021-11-29
			- 2021-12-15
			- 2022-01-13
			- 2022-01-21
    """
    return AutomaticallyRetrievedGraph(
        "MRO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def PCO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-05-03", **kwargs
) -> Graph:
    """Return PCO graph	

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
    version = "2021-05-03"
        Version to retrieve	
		The available versions are:
			- 2021-05-03
    """
    return AutomaticallyRetrievedGraph(
        "PCO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EPSO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "", **kwargs
) -> Graph:
    """Return EPSO graph	

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
    version = ""
        Version to retrieve	
		The available versions are:
			- 2021-05-28
			- 
    """
    return AutomaticallyRetrievedGraph(
        "EPSO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def ORNASEQ(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2019-07-08", **kwargs
) -> Graph:
    """Return ORNASEQ graph	

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
    version = "2019-07-08"
        Version to retrieve	
		The available versions are:
			- 2019-07-08
    """
    return AutomaticallyRetrievedGraph(
        "ORNASEQ", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def HP(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-10-10", **kwargs
) -> Graph:
    """Return HP graph	

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
    version = "2021-10-10"
        Version to retrieve	
		The available versions are:
			- 2022-02-14
			- 2021-08-02
			- 2021-10-10
    """
    return AutomaticallyRetrievedGraph(
        "HP", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def DPO(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-24", **kwargs
) -> Graph:
    """Return DPO graph	

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
    version = "2022-01-24"
        Version to retrieve	
		The available versions are:
			- 2022-02-24
			- 2021-09-02
			- 2021-10-15
			- 2021-12-10
			- 2022-01-24
    """
    return AutomaticallyRetrievedGraph(
        "DPO", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def CL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2022-01-05", **kwargs
) -> Graph:
    """Return CL graph	

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
    version = "2022-01-05"
        Version to retrieve	
		The available versions are:
			- 2022-02-16
			- 2021-09-09
			- 2021-11-25
			- 2021-12-07
			- 2021-12-16
			- 2022-01-05
    """
    return AutomaticallyRetrievedGraph(
        "CL", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def MFOEM(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2021-09-21", **kwargs
) -> Graph:
    """Return MFOEM graph	

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
    version = "2021-09-21"
        Version to retrieve	
		The available versions are:
			- 2021-11-17
			- 2021-09-21
    """
    return AutomaticallyRetrievedGraph(
        "MFOEM", version, "kgobo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

