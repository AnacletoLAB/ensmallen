"""
This file offers the methods to automatically retrieve the graph CiteSeer.

The graph is automatically retrieved from the LINQS repository. 
The CiteSeer dataset consists of 3312 scientific publications classified
into one of six classes. The citation network consists of 4732 links. Each
publication in the dataset is described by a 0/1-valued word vector indicating
the absence/presence of the corresponding word from the dictionary. The
dictionary consists of 3703 unique words.

References
---------------------
Please cite the following if you use the data:

```latex
@incollection{getoor2005link,
  title={Link-based classification},
  author={Getoor, Lise},
  booktitle={Advanced methods for knowledge discovery from complex data},
  pages={189--207},
  year={2005},
  publisher={Springer}
}

@article{sen2008collective,
  title={Collective classification in network data},
  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
  journal={AI magazine},
  volume={29},
  number={3},
  pages={93--93},
  year={2008}
}
```
"""
from typing import Dict
from .parse_linqs import parse_linqs_incidence_matrix
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def CiteSeer(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/linqs",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the CiteSeer graph.

    The graph is automatically retrieved from the LINQS repository.	The CiteSeer dataset consists of 3312 scientific publications classified
	into one of six classes. The citation network consists of 4732 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 3703 unique words.

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
    Instace of CiteSeer graph.

	References
	---------------------
	Please cite the following if you use the data:
	
	```latex
	@incollection{getoor2005link,
	  title={Link-based classification},
	  author={Getoor, Lise},
	  booktitle={Advanced methods for knowledge discovery from complex data},
	  pages={189--207},
	  year={2005},
	  publisher={Springer}
	}
	
	@article{sen2008collective,
	  title={Collective classification in network data},
	  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
	  journal={AI magazine},
	  volume={29},
	  number={3},
	  pages={93--93},
	  year={2008}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="CiteSeer",
        repository="linqs",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs,
		callbacks=[
			parse_linqs_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "citeseer/citeseer/citeseer.cites",
		        "content_path": "citeseer/citeseer/citeseer.content",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
