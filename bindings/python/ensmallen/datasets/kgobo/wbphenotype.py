"""
This file offers the methods to automatically retrieve the graph WBPHENOTYPE.

The graph is automatically retrieved from the KGOBO repository. 


References
---------------------
Please cite the following if you use the data:

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
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def WBPHENOTYPE(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/kgobo",
    version: str = "2021-07-06",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the WBPHENOTYPE graph.

    The graph is automatically retrieved from the KGOBO repository.	

    Parameters
    -------------------
    directed: bool = False
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    verbose: int = 2
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: str = "graphs"
        Where to store the downloaded graphs.
    version: str = "2021-07-06"
        The version of the graph to retrieve.		
	The available versions are:
			- 2021-07-06
    additional_graph_kwargs: Dict
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of WBPHENOTYPE graph.

	References
	---------------------
	Please cite the following if you use the data:
	
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
    return AutomaticallyRetrievedGraph(
        graph_name="WBPHENOTYPE",
        repository="kgobo",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
