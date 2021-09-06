"""
This file offers the methods to automatically retrieve the graph kg-covid-19.

The graph is automatically retrieved from the KGHub repository. 


References
---------------------
Please cite the following if you use the data:

```latex
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
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def KGCOVID19(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/kghub",
    version: str = "current",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the kg-covid-19 graph.

    The graph is automatically retrieved from the KGHub repository.	

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    preprocess: bool = True,
        Whether to preprocess the graph to be loaded in 
        optimal time and memory.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache: bool = True,
        Whether to use cache, i.e. download files only once
        and preprocess them only once.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    version: str = "current",
        The version of the graph to retrieve.		
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
			- current
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of kg-covid-19 graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```latex
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
        graph_name="KGCOVID19",
        repository="kghub",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
