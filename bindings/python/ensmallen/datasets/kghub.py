"""Module providing graphs available from KGHub."""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def SLDB(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "20220522", **kwargs
) -> Graph:
    """Return sldb graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "20220522"
        Version to retrieve	
		The available versions are:
			- 20220522	
	
	
    """
    return AutomaticallyRetrievedGraph(
        "SLDB", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def KGMicrobe(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "current", **kwargs
) -> Graph:
    """Return kg-microbe graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "current"
        Version to retrieve	
		The available versions are:
			- 20210422
			- 20210517
			- 20210608
			- 20210615
			- 20210617
			- 20210622
			- 20210715
			- current	
	
	References
	----------
	Please cite:
	
	```bib
	@article{joachimiakkg,
	  title={KG-Microbe: a reference knowledge-graph and platform for harmonized microbial information},
	  author={Joachimiak, Marcin P and Reese, Justin T and Hegde, Harshad and Cappelletti, Luca and Mungall, Christopher J and Duncan, William D and Thessen, Anne E}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "KGMicrobe", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def KGIDG(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "current", **kwargs
) -> Graph:
    """Return KG-IDG graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "current"
        Version to retrieve	
		The available versions are:
			- 20211029
			- 20211101
			- 20211112
			- 20211123
			- 20211201
			- 20211202
			- 20211207
			- 20211210
			- 20211213
			- 20211215
			- 20211221
			- 20211223
			- 20220101
			- 20220106
			- 20220107
			- 20220119
			- 20220201
			- 20220203
			- 20220204
			- 20220216
			- 20220223
			- 20220303
			- 20220325
			- 20220329
			- 20220401
			- 20220501
			- 20220512
			- 20220525
			- 20220601
			- current	
	
	
    """
    return AutomaticallyRetrievedGraph(
        "KGIDG", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def KGPhenio(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "current", **kwargs
) -> Graph:
    """Return kg-phenio graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "current"
        Version to retrieve	
		The available versions are:
			- 20220304
			- 20220414
			- 20220428
			- 20220429
			- 20220504
			- 20220506
			- 20220511
			- 20220513
			- 20220516
			- 20220525
			- 20220601
			- 20220606
			- current	
	
	
    """
    return AutomaticallyRetrievedGraph(
        "KGPhenio", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def KGCOVID19(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "current", **kwargs
) -> Graph:
    """Return kg-covid-19 graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "current"
        Version to retrieve	
		The available versions are:
			- 20200925
			- 20200927
			- 20200929
			- 20201001
			- 20201012
			- 20201101
			- 20201202
			- 20210101
			- 20210128
			- 20210201
			- 20210218
			- 20210301
			- 20210412
			- 20210725
			- 20210726
			- 20210727
			- 20210823
			- 20210902
			- 20211002
			- 20211102
			- 20211202
			- 20220102
			- 20220202
			- 20220217
			- 20220223
			- 20220225
			- 20220228
			- 20220328
			- 20220330
			- 20220402
			- 20220502
			- current	
	
	References
	----------
	Please cite:
	
	```bib
	@article{reese2021kg,
	  title={KG-COVID-19: a framework to produce customized knowledge graphs for COVID-19 response},
	  author={Reese, Justin T and Unni, Deepak and Callahan, Tiffany J and Cappelletti, Luca and Ravanmehr, Vida and Carbon, Seth and Shefchek, Kent A and Good, Benjamin M and Balhoff, James P and Fontana, Tommaso and others},
	  journal={Patterns},
	  volume={2},
	  number={1},
	  pages={100155},
	  year={2021},
	  publisher={Elsevier}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "KGCOVID19", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def EcoKG(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "20211102", **kwargs
) -> Graph:
    """Return eco-kg graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    load_nodes = True
        Load node names or use numeric range
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "20211102"
        Version to retrieve	
		The available versions are:
			- 20211025
			- 20211102	
	
	
    """
    return AutomaticallyRetrievedGraph(
        "EcoKG", version, "kghub", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

