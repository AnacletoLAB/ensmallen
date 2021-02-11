"""
This file offers the methods to automatically retrieve the graph Butyrivibrio sp. AE3009.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 21:04:45.927302

The undirected graph Butyrivibrio sp. AE3009 has 3456 nodes and 409048
weighted edges, of which none are self-loops. The graph is dense as it
has a density of 0.06851 and has 13 connected components, where the component
with most nodes has 3427 nodes and the component with the least nodes has
2 nodes. The graph median node degree is 220, the mean node degree is 236.72,
and the node degree mode is 3. The top 5 most central nodes are 1280666.ATVS01000001_gene3251
(degree 1658), 1280666.ATVS01000026_gene2720 (degree 1395), 1280666.ATVS01000057_gene2779
(degree 1167), 1280666.ATVS01000004_gene2457 (degree 1083) and 1280666.ATVS01000031_gene1755
(degree 1061).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import ButyrivibrioSp.Ae3009

    # Then load the graph
    graph = ButyrivibrioSp.Ae3009()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Wether to show a loading bar.
        verbose=True
    )

    # Remember that, if you need, you can enable the memory-time trade-offs:
    train_graph.enable(
        vector_sources=True,
        vector_destinations=True,
        vector_outbounds=True
    )

    # Consider using the methods made available in the Embiggen package
    # to run graph embedding or link prediction tasks.
"""
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def ButyrivibrioSp.Ae3009(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Butyrivibrio sp. AE3009 graph.

    The graph is automatically retrieved from the STRING repository. 

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.

    Returns
    -----------------------
    Instace of Butyrivibrio sp. AE3009 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 21:04:45.927302
	
	The undirected graph Butyrivibrio sp. AE3009 has 3456 nodes and 409048
	weighted edges, of which none are self-loops. The graph is dense as it
	has a density of 0.06851 and has 13 connected components, where the component
	with most nodes has 3427 nodes and the component with the least nodes has
	2 nodes. The graph median node degree is 220, the mean node degree is 236.72,
	and the node degree mode is 3. The top 5 most central nodes are 1280666.ATVS01000001_gene3251
	(degree 1658), 1280666.ATVS01000026_gene2720 (degree 1395), 1280666.ATVS01000057_gene2779
	(degree 1167), 1280666.ATVS01000004_gene2457 (degree 1083) and 1280666.ATVS01000031_gene1755
	(degree 1061).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@article{szklarczyk2019string,
	    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
	    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
	    journal={Nucleic acids research},
	    volume={47},
	    number={D1},
	    pages={D607--D613},
	    year={2019},
	    publisher={Oxford University Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.string import ButyrivibrioSp.Ae3009
	
	    # Then load the graph
	    graph = ButyrivibrioSp.Ae3009()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Wether to show a loading bar.
	        verbose=True
	    )
	
	    # Remember that, if you need, you can enable the memory-time trade-offs:
	    train_graph.enable(
	        vector_sources=True,
	        vector_destinations=True,
	        vector_outbounds=True
	    )
	
	    # Consider using the methods made available in the Embiggen package
	    # to run graph embedding or link prediction tasks.
    """
    return AutomaticallyRetrievedGraph(
        "ButyrivibrioSp.Ae3009",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
