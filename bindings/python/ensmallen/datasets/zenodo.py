"""Module providing graphs available from Zenodo."""
from ensmallen import Graph  # pylint: disable=import-error
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph

def WikiLinkIT(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkIT2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkIT", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkFR(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkFR2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkFR", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def GiantTN(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "latest", **kwargs
) -> Graph:
    """Return GIANT-TN graph	

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
	
	References
	----------
	Please cite:
	
	```bib
	@article{liu2020supervised,
	  title={Supervised learning is an accurate method for network-based gene classification},
	  author={Liu, Renming and Mancuso, Christopher A and Yannakopoulos, Anna and Johnson, Kayla A and Krishnan, Arjun},
	  journal={Bioinformatics},
	  volume={36},
	  number={11},
	  pages={3457--3465},
	  year={2020},
	  publisher={Oxford University Press}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "GiantTN", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkDE(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkDE2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkDE", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkNL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkNL2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkNL", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkSV(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkSV2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkSV", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkPL(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkPL2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkPL", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkES(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkES2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkES", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkEN(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkEN2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkEN", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

def WikiLinkRU(
    directed = False, preprocess = "auto", load_nodes = True, load_node_types = True,
    load_edge_weights = True, auto_enable_tradeoffs = True,
    sort_tmp_dir = None, verbose = 2, cache = True, cache_path = None,
    cache_sys_var = "GRAPH_CACHE_DIR", version = "2018", **kwargs
) -> Graph:
    """Return WikiLinkRU2001 graph	

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
    version = "2018"
        Version to retrieve	
		The available versions are:
			- 2001
			- 2002
			- 2003
			- 2004
			- 2005
			- 2006
			- 2007
			- 2008
			- 2009
			- 2010
			- 2011
			- 2012
			- 2013
			- 2014
			- 2015
			- 2016
			- 2017
			- 2018	
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{consonni2019wikilinkgraphs,
	  title={WikiLinkGraphs: a complete, longitudinal and multi-language dataset of the Wikipedia link networks},
	  author={Consonni, Cristian and Laniado, David and Montresor, Alberto},
	  booktitle={Proceedings of the International AAAI Conference on Web and Social Media},
	  volume={13},
	  pages={598--607},
	  year={2019}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        "WikiLinkRU", version, "zenodo", directed, preprocess, load_nodes,
        load_node_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir, verbose, cache,
        cache_path, cache_sys_var, kwargs
    )()

