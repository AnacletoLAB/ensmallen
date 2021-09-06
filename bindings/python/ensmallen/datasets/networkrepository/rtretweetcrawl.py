"""
This file offers the methods to automatically retrieve the graph rt-retweet-crawl.

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

@article{rossi2012fastclique,
        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
        journal={arXiv preprint arXiv:1210.5802},
        pages={1--11},
        year={2012}
}

@inproceedings{rossi2014pmc-www,
        title={Fast Maximum Clique Algorithms for Large Graphs},
        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
        year={2014}
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def RtRetweetCrawl(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/networkrepository",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the rt-retweet-crawl graph.

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
    Instace of rt-retweet-crawl graph.

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
	
	@article{rossi2012fastclique,
	        title={What if CLIQUE were fast? Maximum Cliques in Information Networks and Strong Components in Temporal Networks},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and Mostofa A. Patwary},
	        journal={arXiv preprint arXiv:1210.5802},
	        pages={1--11},
	        year={2012}
	}
	
	@inproceedings{rossi2014pmc-www,
	        title={Fast Maximum Clique Algorithms for Large Graphs},
	        author={Ryan A. Rossi and David F. Gleich and Assefaw H. Gebremedhin and     Mostofa A. Patwary},
	        booktitle={Proceedings of the 23rd International Conference on World     Wide Web (WWW)},
	        year={2014}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="RtRetweetCrawl",
        repository="networkrepository",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
