"""
This file offers the methods to automatically retrieve the graph WikiLinkIT2001.

The graph is automatically retrieved from the Zenodo repository. 


References
---------------------
Please cite the following if you use the data:

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
from typing import Dict, Optional

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def WikiLinkIT(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "2018",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the WikiLinkIT2001 graph.

    The graph is automatically retrieved from the Zenodo repository.	

    Parameters
    -------------------
    directed: bool = False
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    load_nodes: bool = True,
        Whether to load the nodes vocabulary or treat the nodes
        simply as a numeric range.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: Optional[str] = None,
        Where to store the downloaded graphs.
        If no path is provided, first we check the system variable
        provided below is set, otherwise we use the directory `graphs`.
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
        The system variable with the default graph cache directory.
    version: str = "2018"
        The version of the graph to retrieve.		
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
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of WikiLinkIT2001 graph.

	References
	---------------------
	Please cite the following if you use the data:
	
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
        graph_name="WikiLinkIT",
        repository="zenodo",
        version=version,
        directed=directed,
        preprocess=preprocess,
        load_nodes=load_nodes,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        cache_path_system_variable=cache_path_system_variable,
        additional_graph_kwargs=additional_graph_kwargs
    )()
