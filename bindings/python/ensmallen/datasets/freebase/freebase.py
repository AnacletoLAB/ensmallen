"""
This file offers the methods to automatically retrieve the graph FreeBase.

The graph is automatically retrieved from the FreeBase repository. 


References
---------------------
Please cite the following if you use the data:

```bib
@inproceedings{bollacker2008freebase,
  title={Freebase: a collaboratively created graph database for structuring human knowledge},
  author={Bollacker, Kurt and Evans, Colin and Paritosh, Praveen and Sturge, Tim and Taylor, Jamie},
  booktitle={Proceedings of the 2008 ACM SIGMOD international conference on Management of data},
  pages={1247--1250},
  year={2008}
}

@misc{freebase:datadumps,
  title = "Freebase Data Dumps"
  author = "Google",
  howpublished = "\url{https://developers.google.com/freebase/data}",
}
```
"""
from typing import Dict, Optional

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def FreeBase(
    directed: bool = False,
    preprocess: bool = True,
    load_nodes: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: Optional[str] = None,
    cache_path_system_variable: str = "GRAPH_CACHE_DIR",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the FreeBase graph.

    The graph is automatically retrieved from the FreeBase repository.	

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
    version: str = "latest"
        The version of the graph to retrieve.	
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of FreeBase graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```bib
	@inproceedings{bollacker2008freebase,
	  title={Freebase: a collaboratively created graph database for structuring human knowledge},
	  author={Bollacker, Kurt and Evans, Colin and Paritosh, Praveen and Sturge, Tim and Taylor, Jamie},
	  booktitle={Proceedings of the 2008 ACM SIGMOD international conference on Management of data},
	  pages={1247--1250},
	  year={2008}
	}
	
	@misc{freebase:datadumps,
	  title = "Freebase Data Dumps"
	  author = "Google",
	  howpublished = "\url{https://developers.google.com/freebase/data}",
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="FreeBase",
        repository="freebase",
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
