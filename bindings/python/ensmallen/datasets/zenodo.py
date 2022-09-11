"""Module providing graphs available from Zenodo."""
from ensmallen import Graph  # pylint: disable=import-error
from .graph_retrieval import RetrievedGraph

def WikiLinkIT(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkIT2001 graph	

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
    return RetrievedGraph(
        "WikiLinkIT", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkFR(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkFR2001 graph	

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
    return RetrievedGraph(
        "WikiLinkFR", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def NPKG(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return NPKG graph	

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
    version = "latest"
        Version to retrieve		
	
	
    """
    return RetrievedGraph(
        "NPKG", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def GiantTN(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return GIANT-TN graph	

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
    return RetrievedGraph(
        "GiantTN", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def PharMeBINet(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return PharMeBINet graph	

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
    version = "latest"
        Version to retrieve		
	
	
    """
    return RetrievedGraph(
        "PharMeBINet", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def Ubergraph(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return Ubergraph graph	

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
    version = "latest"
        Version to retrieve		
	
	
    """
    return RetrievedGraph(
        "Ubergraph", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkDE(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkDE2001 graph	

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
    return RetrievedGraph(
        "WikiLinkDE", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkNL(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkNL2001 graph	

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
    return RetrievedGraph(
        "WikiLinkNL", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkSV(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkSV2001 graph	

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
    return RetrievedGraph(
        "WikiLinkSV", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkPL(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkPL2001 graph	

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
    return RetrievedGraph(
        "WikiLinkPL", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkES(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkES2001 graph	

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
    return RetrievedGraph(
        "WikiLinkES", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def GameOfThronesCharactersInteractions(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return Game of Thrones Characters Interactions graph	

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
    version = "latest"
        Version to retrieve		
	
	
    """
    return RetrievedGraph(
        "GameOfThronesCharactersInteractions", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkEN(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkEN2001 graph	

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
    return RetrievedGraph(
        "WikiLinkEN", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def FAVA(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return FAVA graph	

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
    version = "latest"
        Version to retrieve		
	
	
    """
    return RetrievedGraph(
        "FAVA", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
def WikiLinkRU(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="2018", **kwargs
) -> Graph:
    """Return WikiLinkRU2001 graph	

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
    return RetrievedGraph(
        "WikiLinkRU", version, "zenodo", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs
    )()
