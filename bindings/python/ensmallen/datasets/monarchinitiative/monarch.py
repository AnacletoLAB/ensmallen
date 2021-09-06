"""
This file offers the methods to automatically retrieve the graph Monarch.

The graph is automatically retrieved from the MonarchInitiative repository. 


References
---------------------
Please cite the following if you use the data:

```latex
@article{mungall2017monarch,
  title={The Monarch Initiative: an integrative data and analytic platform connecting phenotypes to genotypes across species},
  author={Mungall, Christopher J and McMurry, Julie A and K{\"o}hler, Sebastian and Balhoff, James P and Borromeo, Charles and Brush, Matthew and Carbon, Seth and Conlin, Tom and Dunn, Nathan and Engelstad, Mark and others},
  journal={Nucleic acids research},
  volume={45},
  number={D1},
  pages={D712--D722},
  year={2017},
  publisher={Oxford University Press}
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def Monarch(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/monarchinitiative",
    version: str = "202103",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the Monarch graph.

    The graph is automatically retrieved from the MonarchInitiative repository.	

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
    version: str = "202103",
        The version of the graph to retrieve.		
	The available versions are:
			- 202012
			- 202103
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of Monarch graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```latex
	@article{mungall2017monarch,
	  title={The Monarch Initiative: an integrative data and analytic platform connecting phenotypes to genotypes across species},
	  author={Mungall, Christopher J and McMurry, Julie A and K{\"o}hler, Sebastian and Balhoff, James P and Borromeo, Charles and Brush, Matthew and Carbon, Seth and Conlin, Tom and Dunn, Nathan and Engelstad, Mark and others},
	  journal={Nucleic acids research},
	  volume={45},
	  number={D1},
	  pages={D712--D722},
	  year={2017},
	  publisher={Oxford University Press}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="Monarch",
        repository="monarchinitiative",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
