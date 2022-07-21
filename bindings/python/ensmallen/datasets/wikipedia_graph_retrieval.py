"""Module providing class for retrieving graphs."""

import os
import shutil
from typing import Dict, List, Optional

from dict_hash import sha256
from ensmallen import Graph, edge_list_utils
from .graph_retrieval import RetrievedGraph
from environments_utils import is_windows


class WikipediaRetrievedGraph(RetrievedGraph):
    """Class definying an retrievable wikipedia graph."""

    def __init__(
        self,
        name: str,
        version: str,
        directed: bool = False,
        load_nodes: bool = True,
        load_node_types: bool = True,
        load_edge_types: bool = True,
        keep_nodes_without_descriptions: bool = True,
        keep_nodes_without_categories: bool = True,
        keep_interwikipedia_nodes: bool = True,
        keep_external_nodes: bool = True,
        compute_node_description: bool = False,
        auto_enable_tradeoffs: bool = True,
        sort_tmp_dir: Optional[str] = None,
        verbose: int = 2,
        cache: bool = True,
        cache_path: Optional[str] = None,
        cache_sys_var: str = "GRAPH_CACHE_DIR",
        graph_kwargs: Dict = None
    ):
        """Create new automatically retrieved graph.

        Parameters
        -------------------
        name: str
            The name of the graph to be retrieved and loaded.
        version: str
            The version of the graph to be retrieved.
        directed: bool = False
            Whether to load the graph as directed or undirected.
            By default false.
        load_nodes: bool = True
            Whether to load the nodes vocabulary or treat the nodes
            simply as a numeric range.
            This feature is only available when the preprocessing is enabled.
        load_node_types: bool = True
            Whether to load the node types if available or skip them entirely.
            This feature is only available when the preprocessing is enabled.
        load_edge_types: bool = True
            Whether to load the edge types if available or skip them entirely.
            This feature is only available when the preprocessing is enabled.
        keep_nodes_without_descriptions: bool = True
            Whether to keep the nodes laking a description
        keep_nodes_without_categories: bool = True
            Whether to keep the nodes laking a category
        keep_interwikipedia_nodes: bool = True
            Whether to keep nodes from external wikipedia websites
        keep_external_nodes: bool = True
            Whether to keep nodes from external websites (non wikipedia ones).
        compute_node_description: bool = False
            Whether to compute the node descriptions.
            Note that this will significantly increase the side of the node lists!
        auto_enable_tradeoffs: bool = True
            Whether to enable the Ensmallen time-memory tradeoffs in small graphs
            automatically. By default True, that is, if a graph has less than
            50 million edges. In such use cases the memory expenditure is minimal.
        sort_tmp_dir: Optional[str] = None
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
        cache_sys_var: str = "GRAPH_CACHE_DIR"
            The system variable with the default graph cache directory.
        graph_kwargs: Dict = None
            Eventual additional kwargs for loading the graph.

        Raises
        -------------------
        ValueError,
            If the given graph name is not available.
        ValueError,
            If the OS is Windows, we cannot process the file.
        """
        if is_windows():
            raise ValueError(
                "On Windows we do not support the processing of "
                "Wikipedia graphs."
            )
        self._keep_nodes_without_descriptions = keep_nodes_without_descriptions
        self._keep_nodes_without_categories = keep_nodes_without_categories
        self._keep_interwikipedia_nodes = keep_interwikipedia_nodes
        self._keep_external_nodes = keep_external_nodes
        self._compute_node_description = compute_node_description

        super().__init__(
            name=name,
            version=version,
            repository="wikipedia",
            directed=directed,
            preprocess=True,
            load_nodes=load_nodes,
            load_node_types=load_node_types,
            load_edge_types=load_edge_types,
            load_edge_weights=False,
            auto_enable_tradeoffs=auto_enable_tradeoffs,
            sort_tmp_dir=sort_tmp_dir,
            verbose=verbose,
            cache=cache,
            cache_path=cache_path,
            cache_sys_var=cache_sys_var,
            graph_kwargs=graph_kwargs,
            hash_seed=sha256(dict(
                keep_nodes_without_descriptions=keep_nodes_without_descriptions,
                keep_nodes_without_categories=keep_nodes_without_categories,
                keep_interwikipedia_nodes=keep_interwikipedia_nodes,
                keep_external_nodes=keep_external_nodes,
                compute_node_description=compute_node_description
            ))
        )

    def __call__(self) -> Graph:
        """Return Graph containing required graph."""
        graph_arguments = self.get_graph_arguments()
        root = self.get_preprocessed_graph_directory_path()

        if not self._cache and os.path.exists(root):
            shutil.rmtree(root)

        paths = self.get_adjusted_graph_paths()
        if not os.path.exists(root):
            # Download the necessary data
            self._downloader.download(
                self._graph["urls"],
                paths
            )

        os.makedirs(root, exist_ok=True)

        node_type_list_path = self.get_preprocessed_graph_node_types_path()
        edge_type_list_path = self.get_preprocessed_graph_edge_types_path()
        node_path = self.get_preprocessed_graph_nodes_path()
        edge_path = self.get_preprocessed_graph_edges_path()

        if not self.is_preprocessed():
            (
                node_types_number,
                nodes_number,
                edges_number
            ) = edge_list_utils.parse_wikipedia_graph(
                source_path=paths[0].replace(".bz2", ""),
                edge_path=edge_path,
                node_path=node_path,
                node_type_path=node_type_list_path,
                edge_type_path=edge_type_list_path,
                node_list_separator="\t",
                node_type_list_separator="\t",
                edge_type_list_separator="\t",
                node_types_separator="|",
                nodes_column="node_names",
                node_types_column="node_type_names",
                node_list_node_types_column="node_type_names",
                edge_types_column="edge_type_names",
                node_descriptions_column="node_descriptions",
                edge_list_separator="\t",
                keep_nodes_without_descriptions=self._keep_nodes_without_descriptions,
                keep_nodes_without_categories=self._keep_nodes_without_categories,
                keep_interwikipedia_nodes=self._keep_interwikipedia_nodes,
                keep_external_nodes=self._keep_external_nodes,
                compute_node_description=self._compute_node_description,
                sort_temporary_directory=self._sort_tmp_dir,
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
        if self._load_node_types:
            node_types_arguments = {
                "node_type_path": node_type_list_path,
                "node_types_number": metadata["node_types_number"],
                "node_types_column": "node_type_names",
                "node_type_list_is_correct": True,
                "node_type_list_separator": "\t",
                "node_types_separator": "|",
                "node_list_node_types_column_number": 1,
                "node_list_numeric_node_type_ids": True,
            }
        else:
            node_types_arguments = {}
        # If the nodes are to be loaded
        if self._load_nodes:
            nodes_arguments = {
                "node_path": node_path,
                "node_list_separator": "\t",
                "nodes_column": "node_names",
                "node_list_is_correct": True,
                **node_types_arguments
            }
        else:
            nodes_arguments = {
                "numeric_node_ids": True,
            }

        # If the edge types are provided
        edge_types_arguments = {
            "edge_type_path": edge_type_list_path,
            "edge_types_number": metadata["edge_types_number"],
            "edge_types_column_number": 0,
            "edge_type_list_is_correct": True,
            "edge_type_list_separator": "\t",
            "edge_list_edge_types_column_number": 2,
            "edge_list_numeric_edge_type_ids": True
        }

        # Load the graph
        graph = Graph.from_csv(**{
            **metadata,
            **graph_arguments,
            **nodes_arguments,
            **edge_types_arguments,

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
        if self._auto_enable_tradeoffs and graph.get_number_of_unique_edges() < 50e6:
            graph.enable()
        return graph
