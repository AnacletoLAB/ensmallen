"""
This file offers the methods to automatically retrieve the graph bio-diseasome.

The graph is automatically retrieved from the NetworkRepository repository. 


References
---------------------
Please cite the following if you use the data:

```latex
@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}

@article{goh2007human,
        title={The human disease network},
        author={Goh, Kwang-Il and Cusick, Michael E and Valle, David and Childs, Barton and Vidal, Marc and Barab{\'a}si,
Albert-L{\'a}szl{\'o}
},
        journal={Proceedings of the National Academy of Sciences},
        volume={104},
        number={21},
        pages={8685--8690},
        year={2007},
        publisher={National Acad Sciences}
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def BioDiseasome(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/networkrepository",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the bio-diseasome graph.

    The graph is automatically retrieved from the NetworkRepository repository.	

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
    version: str = "latest",
        The version of the graph to retrieve.	
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of bio-diseasome graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```latex
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	
	@article{goh2007human,
	        title={The human disease network},
	        author={Goh, Kwang-Il and Cusick, Michael E and Valle, David and Childs, Barton and Vidal, Marc and Barab{\'a}si,
	Albert-L{\'a}szl{\'o}
	},
	        journal={Proceedings of the National Academy of Sciences},
	        volume={104},
	        number={21},
	        pages={8685--8690},
	        year={2007},
	        publisher={National Acad Sciences}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="BioDiseasome",
        repository="networkrepository",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
