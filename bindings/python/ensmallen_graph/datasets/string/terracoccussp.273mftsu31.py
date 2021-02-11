"""
This file offers the methods to automatically retrieve the graph Terracoccus sp. 273MFTsu31.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 23:07:33.029954

The undirected graph Terracoccus sp. 273MFTsu31 has 4440 nodes and 555851
weighted edges, of which none are self-loops. The graph is dense as it
has a density of 0.05641 and has 9 connected components, where the component
with most nodes has 4411 nodes and the component with the least nodes has
2 nodes. The graph median node degree is 231, the mean node degree is 250.38,
and the node degree mode is 3. The top 5 most central nodes are 1172188.KB911823_gene352
(degree 1213), 1172188.KB911820_gene3134 (degree 1204), 1172188.KB911825_gene3644
(degree 1202), 1172188.KB911821_gene1914 (degree 1186) and 1172188.KB911825_gene3663
(degree 1176).


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
    from ensmallen_graph.datasets.string import TerracoccusSp.273mftsu31

    # Then load the graph
    graph = TerracoccusSp.273mftsu31()

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


def TerracoccusSp.273mftsu31(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Terracoccus sp. 273MFTsu31 graph.

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
    Instace of Terracoccus sp. 273MFTsu31 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 23:07:33.029954
	
	The undirected graph Terracoccus sp. 273MFTsu31 has 4440 nodes and 555851
	weighted edges, of which none are self-loops. The graph is dense as it
	has a density of 0.05641 and has 9 connected components, where the component
	with most nodes has 4411 nodes and the component with the least nodes has
	2 nodes. The graph median node degree is 231, the mean node degree is 250.38,
	and the node degree mode is 3. The top 5 most central nodes are 1172188.KB911823_gene352
	(degree 1213), 1172188.KB911820_gene3134 (degree 1204), 1172188.KB911825_gene3644
	(degree 1202), 1172188.KB911821_gene1914 (degree 1186) and 1172188.KB911825_gene3663
	(degree 1176).
	


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
	    from ensmallen_graph.datasets.string import TerracoccusSp.273mftsu31
	
	    # Then load the graph
	    graph = TerracoccusSp.273mftsu31()
	
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
        "TerracoccusSp.273mftsu31",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
