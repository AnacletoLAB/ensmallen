"""
This file offers the methods to automatically retrieve the graph bn-mouse_retina_1.

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

@article {bigbrain,
     author = {Amunts, Katrin and Lepage, Claude and Borgeat, Louis and Mohlberg, Hartmut and Dickscheid, Timo and Rousseau, Marc-{'E}tienne and Bludau,
Sebastian and Bazin, Pierre-Louis and Lewis, Lindsay B. and Oros-Peusquens, Ana-Maria and Shah, Nadim J. and Lippert, Thomas and Zilles, Karl and Evans, Alan C.},
     title = {BigBrain: An Ultrahigh-Resolution 3D Human Brain Model},
     volume = {340},
     number = {6139},
     pages = {1472--1475},
     year = {2013},
     publisher = {AAAS},
     journal = {Science}
}
```
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen import Graph  # pylint: disable=import-error


def BNMouseRetina(
    directed: bool = False,
    preprocess: bool = True,
    verbose: int = 2,
    cache: bool = True,
    cache_path: str = "graphs/networkrepository",
    version: str = "latest",
    **additional_graph_kwargs: Dict
) -> Graph:
    """Return new instance of the bn-mouse_retina_1 graph.

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
    Instace of bn-mouse_retina_1 graph.

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
	
	@article {bigbrain,
	     author = {Amunts, Katrin and Lepage, Claude and Borgeat, Louis and Mohlberg, Hartmut and Dickscheid, Timo and Rousseau, Marc-{'E}tienne and Bludau,
	Sebastian and Bazin, Pierre-Louis and Lewis, Lindsay B. and Oros-Peusquens, Ana-Maria and Shah, Nadim J. and Lippert, Thomas and Zilles, Karl and Evans, Alan C.},
	     title = {BigBrain: An Ultrahigh-Resolution 3D Human Brain Model},
	     volume = {340},
	     number = {6139},
	     pages = {1472--1475},
	     year = {2013},
	     publisher = {AAAS},
	     journal = {Science}
	}
	```
    """
    return AutomaticallyRetrievedGraph(
        graph_name="BNMouseRetina",
        repository="networkrepository",
        version=version,
        directed=directed,
        preprocess=preprocess,
        verbose=verbose,
        cache=cache,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
