"""
This file offers the methods to automatically retrieve the graph soc-epinions.

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

@incollection{richardson2003trust,
        title={Trust management for the semantic web},
        author={Richardson, Matthew and Agrawal, Rakesh and Domingos, Pedro},
        booktitle={The Semantic Web-ISWC 2003},
        pages={351--368},
        year={2003},
        publisher={Springer}
}

@inproceedings{nr:massa05,
        title = {Controversial Users Demand Local Trust Metrics: An Experimental Study on epinions.com Community},
        author = {Paolo Massa and Paolo Avesani},
        booktitle = {AAAI},
        year = {2005},
        pages = {121--126},
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def SocEpinions(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/networkrepository",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the soc-epinions graph.

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
    Instace of soc-epinions graph.

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
	
	@incollection{richardson2003trust,
	        title={Trust management for the semantic web},
	        author={Richardson, Matthew and Agrawal, Rakesh and Domingos, Pedro},
	        booktitle={The Semantic Web-ISWC 2003},
	        pages={351--368},
	        year={2003},
	        publisher={Springer}
	}
	
	@inproceedings{nr:massa05,
	        title = {Controversial Users Demand Local Trust Metrics: An Experimental Study on epinions.com Community},
	        author = {Paolo Massa and Paolo Avesani},
	        booktitle = {AAAI},
	        year = {2005},
	        pages = {121--126},
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="SocEpinions",
        repository="networkrepository",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
