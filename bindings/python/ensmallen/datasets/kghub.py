"""Module providing graphs available from KGHub."""
from ensmallen import Graph  # pylint: disable=import-error
from .graph_retrieval import RetrievedGraph

def EcoKG(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="20211102", **kwargs
) -> Graph:
    """Return eco-kg graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
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
    return RetrievedGraph(
        "EcoKG", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def KGCOVID19(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="contact the KG-Hub team by email", **kwargs
) -> Graph:
    """Return kg-covid-19 graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "contact the KG-Hub team by email"
        Version to retrieve	
		The available versions are:
			- contact the KG-Hub team by email	
	
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
    return RetrievedGraph(
        "KGCOVID19", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def KGIDG(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="contact the KG-Hub team by email", **kwargs
) -> Graph:
    """Return KG-IDG graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "contact the KG-Hub team by email"
        Version to retrieve	
		The available versions are:
			- contact the KG-Hub team by email	
	
	
    """
    return RetrievedGraph(
        "KGIDG", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def KGMicrobe(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="current", **kwargs
) -> Graph:
    """Return kg-microbe graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
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
			- 20230109
			- 20230116
			- 20230206
			- 20230216
			- 20230303
			- 20230310
			- 20230316
			- 20230416
			- 20230516
			- 20230616
			- 20230716
			- 20230816
			- 20230914
			- 20230916
			- 20230919
			- 20231016
			- 20231024
			- 20231027
			- 20231103
			- 20231104
			- 20231114
			- 20231115
			- 20231116
			- 20231216
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
    return RetrievedGraph(
        "KGMicrobe", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def KGPhenio(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="current", **kwargs
) -> Graph:
    """Return kg-phenio graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
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
			- 20220607
			- 20220623
			- 20220722
			- 20220819
			- 20220930
			- 20221006
			- 20221027
			- 20221215
			- 20230210
			- 20230302
			- 20230307
			- 20230313
			- 20230502
			- 20230623
			- 20230712
			- 20230803
			- 20230808
			- 20230814
			- 20230818
			- 20230821
			- 20230822
			- 20230825
			- 20230908
			- 20230913
			- 20231008
			- 20231114
			- 20231121
			- 20231213
			- 20240104
			- 20240108
			- 20240111
			- 20240125
			- 20240208
			- 20240311
			- 20240313
			- 20240404
			- 20240405
			- 20240408
			- 20240411
			- 20240514
			- 20240606
			- 20240607
			- 20240608
			- 20240708
			- 20240802
			- 20240808
			- 20240809
			- 20240904
			- 20241008
			- 20241104
			- 20241105
			- 20241108
			- 20241127
			- 20241203
			- 20241208
			- 20241210
			- 20250108
			- 20250130
			- 20250208
			- 20250210
			- 20250216
			- 20250307
			- 20250308
			- 20250321
			- 20250408
			- 20250414
			- 20250522
			- 20250604
			- 20250608
			- 20250708
			- 20250808
			- 20250912
			- current	
	
	
    """
    return RetrievedGraph(
        "KGPhenio", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def SLDB(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="20220522", **kwargs
) -> Graph:
    """Return sldb graph	

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
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
    return RetrievedGraph(
        "SLDB", version, "kghub", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
