"""Module providing class for automatically retrieving graphs."""

import os
import shutil
from typing import Callable, Dict, List, Optional

import compress_json
from downloaders import BaseDownloader
from environments_utils import is_windows
from userinput.utils import set_validator, closest
from ..ensmallen import Graph, edge_list_utils
from .get_dataset import validate_graph_version
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph


class WikipediaAutomaticallyRetrievedGraph(AutomaticallyRetrievedGraph):
    """Class definying an automatically retrievable wikipedia graph."""

    def __init__(
        self,
        graph_name: str,
        version: str,
        repository: str,
        directed: bool = False,
        preprocess: bool = True,
        load_nodes: bool = True,
        load_node_types: bool = True,
        automatically_enable_speedups_for_small_graphs: bool = True,
        sort_temporary_directory: Optional[str] = None,
        verbose: int = 2,
        cache: bool = True,
        cache_path: Optional[str] = None,
        cache_path_system_variable: str = "GRAPH_CACHE_DIR",
        additional_graph_kwargs: Dict = None
    ):
        """Create new automatically retrieved graph.

        Parameters
        -------------------
        graph_name: str
            The name of the graph to be retrieved and loaded.
        version: str
            The version of the graph to be retrieved.
        repository: str
            Name of the repository to load data from.
        directed: bool = False
            Whether to load the graph as directed or undirected.
            By default false.
        preprocess: bool = True
            Whether to preprocess the node list and edge list
            to be loaded optimally in both time and memory.
        load_nodes: bool = True
            Whether to load the nodes vocabulary or treat the nodes
            simply as a numeric range.
            This feature is only available when the preprocessing is enabled.
        load_node_types: bool = True
            Whether to load the node types if available or skip them entirely.
            This feature is only available when the preprocessing is enabled.
        automatically_enable_speedups_for_small_graphs: bool = True
            Whether to enable the Ensmallen time-memory tradeoffs in small graphs
            automatically. By default True, that is, if a graph has less than
            50 million edges. In such use cases the memory expenditure is minimal.
        sort_temporary_directory: Optional[str] = None
            Which folder to use to store the temporary files needed to sort in 
            parallel the edge list when building the optimal preprocessed file.
            This defaults to the same folder of the edge list when no value is 
            provided.
        verbose: int = 2
            Whether to show loading bars.
        cache: bool = True
            Whether to use cache, i.e. download files only once
            and preprocess them only once.
        cache_path: Optional[str] = None
            Where to store the downloaded graphs.
            If no path is provided, first we check the system variable
            provided below is set, otherwise we use the directory `graphs`.
        cache_path_system_variable: str = "GRAPH_CACHE_DIR"
            The system variable with the default graph cache directory.
        additional_graph_kwargs: Dict = None
            Eventual additional kwargs for loading the graph.

        Raises
        -------------------
        ValueError,
            If the given graph name is not available.
        ValueError,
            If the preprocess flag is provided but the system
            is Windows, which does not provide the sort command.
        """
        super().__init__(
            graph_name=graph_name,
            version=version,
            repository=repository,
            directed=directed,
            preprocess=True,
            load_nodes=load_nodes,
            load_node_types=load_node_types,
            load_edge_weights=False,
            automatically_enable_speedups_for_small_graphs=automatically_enable_speedups_for_small_graphs,
            sort_temporary_directory=sort_temporary_directory,
            verbose=verbose,
            cache=cache,
            cache_path=cache_path,
            cache_path_system_variable=cache_path_system_variable,
            additional_graph_kwargs=additional_graph_kwargs
        )

    def __call__(self) -> Graph:
        """Return Graph containing required graph."""
        graph_arguments = self.get_graph_arguments()
        root = self.get_preprocessed_graph_directory_path()

        if not self._cache and os.path.exists(root):
            shutil.rmtree(root)

        if not os.path.exists(root):
            # Download the necessary data
            self._downloader.download(
                self._graph["urls"],
                self.get_adjusted_graph_paths()
            )

        os.makedirs(root, exist_ok=True)

        node_type_list_path = self.get_preprocessed_graph_node_types_path()
        node_path = self.get_preprocessed_graph_nodes_path()
        edge_path = self.get_preprocessed_graph_edges_path()

        if not self.is_preprocessed():
            (
                node_types_number,
                nodes_number,
                edges_number
            ) = edge_list_utils.parse_wikipedia_graph(
                source_path=graph_arguments["source_path"],
                edge_path=edge_path,
                node_path=node_path,
                node_type_path=node_type_list_path,
                node_list_separator="\t",
                node_type_list_separator="\t",
                node_types_separator="|",
                nodes_column="node_names",
                node_types_column="node_type_names",
                node_list_node_types_column="node_type_names",
                edge_list_separator="\t",
                sort_temporary_directory=self._sort_temporary_directory,
                directed=self._directed,
                verbose=self._verbose > 0,
            )
            # Store the obtained metadata
            self.store_preprocessed_metadata(
                node_types_number,
                nodes_number,
                None,
                edges_number
            )
        # Load the stored metadata
        metadata = self.get_preprocessed_metadata()
        # If the node types are provided
        has_node_types = metadata["node_types_number"] is not None and self._load_node_types
        if has_node_types:
            node_types_arguments = {
                "node_type_path": node_type_list_path,
                "node_types_column": "node_type_names",
                "node_type_list_is_correct": True,
                "node_type_list_separator": "\t"
            }
        else:
            node_types_arguments = {}
        # If the nodes are to be loaded
        if self._load_nodes:
            nodes_arguments = {
                "node_path": node_path,
                "node_list_separator": "\t",
                "nodes_column": "node_names",
                "node_types_separator": "|" if has_node_types else None,
                "node_list_node_types_column_number": 1 if has_node_types else None,
                "node_list_numeric_node_type_ids": True if has_node_types else None,
                "skip_node_types_if_unavailable": True if has_node_types else None,
                "node_list_is_correct": True,
                **node_types_arguments
            }
        else:
            nodes_arguments = {
                "numeric_node_ids": True,
            }

        # Load the graph
        graph = Graph.from_csv(**{
            **metadata,
            **nodes_arguments,

            "edge_path": edge_path,
            "edge_list_header": False,
            "sources_column_number": 0,
            "destinations_column_number": 1,
            "edge_list_numeric_node_ids": True,
            "edge_list_is_complete": True,
            "edge_list_may_contain_duplicates": False,
            "edge_list_is_sorted": True,
            "edge_list_is_correct": True,
            "edges_number": metadata["edges_number"],
            "nodes_number": metadata["nodes_number"],
            "may_have_singletons": True,
            "verbose": self._verbose > 0,
            "directed": self._directed,
            "name": self._name,
        })
        if self._automatically_enable_speedups_for_small_graphs and graph.get_unique_edges_number() < 50e6:
            graph.enable()
        return graph
