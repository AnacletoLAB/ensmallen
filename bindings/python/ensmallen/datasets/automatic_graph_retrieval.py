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


class AutomaticallyRetrievedGraph:
    """Class definying an automatically retrievable graph."""

    def __init__(
        self,
        graph_name: str,
        version: str,
        repository: str,
        directed: bool = False,
        preprocess: bool = True,
        load_nodes: bool = True,
        load_node_types: bool = True,
        load_edge_weights: bool = True,
        automatically_enable_speedups_for_small_graphs: bool = True,
        sort_temporary_directory: Optional[str] = None,
        verbose: int = 2,
        cache: bool = True,
        cache_path: Optional[str] = None,
        cache_path_system_variable: str = "GRAPH_CACHE_DIR",
        callbacks: List[Callable] = (),
        callbacks_arguments: List[Dict] = (),
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
        load_edge_weights: bool = True
            Whether to load the edge weights if available or skip them entirely.
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
        callbacks: List[Callable] = ()
            Eventual callbacks to call after download files.
        callbacks_arguments: List[Dict] = ()
            Eventual arguments for callbacks.
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
        try:
            validate_graph_version(graph_name, repository, version)

            all_versions = compress_json.local_load(os.path.join(
                repository,
                "{}.json.gz".format(graph_name)
            ))

            self._graph = all_versions[version]
        except FileNotFoundError:
            raise ValueError(
                (
                    "Requested graph `{}` is not currently available.\n"
                    "Open an issue on the Graph repository to ask "
                    "for this graph to be added."
                ).format(graph_name)
            )

        if preprocess and is_windows():
            raise ValueError(
                "Currently preprocessing to optimal edge list is not supported "
                "on Windows because the sorting step is based upon the `sort` "
                "command, which is only available to our knowledge on Linux and "
                "macOS systems."
            )

        # If the cache path was not provided
        # we either check the system variable
        # and if it is not set we use `graphs`
        if cache_path is None:
            cache_path = os.getenv(cache_path_system_variable, "graphs")

        cache_path = os.path.join(cache_path, repository)

        self._directed = directed
        self._preprocess = preprocess
        self._load_nodes = load_nodes
        self._load_node_types = load_node_types
        self._load_edge_weights = load_edge_weights
        self._name = graph_name
        self._version = version
        self._automatically_enable_speedups_for_small_graphs = automatically_enable_speedups_for_small_graphs
        self._sort_temporary_directory = sort_temporary_directory
        self._cache = cache
        self._verbose = verbose
        self._callbacks = callbacks
        if additional_graph_kwargs is None:
            additional_graph_kwargs = {}
        self._additional_graph_kwargs = additional_graph_kwargs
        self._callbacks_arguments = callbacks_arguments
        self._cache_path = os.path.join(cache_path, graph_name, version)
        self._downloader = BaseDownloader(
            auto_extract=True,
            cache=cache,
            target_directory=self._cache_path,
            verbose=self._verbose,
            process_number=1
        )

    def get_preprocessed_graph_directory_path(self) -> str:
        """Return the path to the directory where to store the preprocessed graph."""
        return os.path.join(
            self._cache_path,
            "preprocessed",
            "directed" if self._directed else "undirected"
        )

    def get_preprocessed_graph_node_types_path(self) -> str:
        """Return the path to file where the metadata of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "node_types.tsv"
        )

    def get_preprocessed_graph_nodes_path(self) -> str:
        """Return the path to file where the nodes of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "nodes.tsv"
        )

    def get_adjusted_graph_nodes_path(self) -> str:
        """Return the path to file where the nodes of the graph are downloaded."""
        node_path = self.get_graph_arguments().get("node_path")
        if node_path is not None:
            # We add the cache path to it
            return os.path.join(
                self._cache_path,
                node_path
            )

    def get_preprocessed_graph_edge_types_path(self) -> str:
        """Return the path to file where the metadata of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "edge_types.tsv"
        )

    def get_preprocessed_graph_edges_path(self) -> str:
        """Return the path to file where the metadata of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "edges.tsv"
        )

    def get_preprocessed_graph_metadata_path(self) -> str:
        """Return the path to file where the metadata of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "metadata.json"
        )

    def is_preprocessed(self) -> bool:
        """Return whether this graph was preprocessed."""
        return os.path.exists(
            self.get_preprocessed_graph_metadata_path()
        )

    def store_preprocessed_metadata(
        self,
        node_types_number: Optional[int],
        nodes_number: int,
        edge_types_number: Optional[int],
        edges_number: int,
    ):
        """Store the provided metadata.

        Parameters
        --------------------------------
        node_types_number: Optional[int],
            The number of unique node types existing in this graph.
        nodes_number: int,
            The number of unique nodes existing in this graph.
        edge_types_number: Optional[int],
            The number of unique edge types existing in this graph.
        edges_number: int,
            The number of edges existing in this graph.
        """
        compress_json.dump(
            dict(
                node_types_number=node_types_number,
                nodes_number=nodes_number,
                edge_types_number=edge_types_number,
                edges_number=edges_number
            ),
            self.get_preprocessed_graph_metadata_path()
        )

    def get_preprocessed_metadata(self) -> Dict:
        """Return the stored metadata.

        Returns
        --------------------------------
        Dictionary with the metadata.
        """
        return compress_json.load(
            self.get_preprocessed_graph_metadata_path()
        )

    def get_graph_arguments(self) -> Dict:
        """Return the dictionary of arguments of the Graph."""
        return {
            **self._graph["arguments"],
            **self._additional_graph_kwargs
        }

    def get_adjusted_graph_paths(self) -> str:
        """Return adjusted list of paths."""
        paths = self._graph.get("paths", None)
        if paths is not None:
            paths = [
                os.path.join(self._cache_path, path)
                for path in paths
            ]
        return paths

    def download(self):
        if not os.path.exists(self.get_preprocessed_graph_directory_path()):
            # Download the necessary data
            self._downloader.download(
                self._graph["urls"],
                self.get_adjusted_graph_paths()
            )

    def __call__(self) -> Graph:
        """Return Graph containing required graph."""
        graph_arguments = self.get_graph_arguments()
        root = self.get_preprocessed_graph_directory_path()

        if not self._cache and os.path.exists(root):
            shutil.rmtree(root)

        self.download()
        os.makedirs(root, exist_ok=True)

        # Call the provided callbacks to process the edge lists, if any.
        for callback, arguments in zip(self._callbacks, self._callbacks_arguments):
            callback(**{
                key: os.path.join(self._cache_path, value)
                if key.endswith("_path") else value
                for key, value in arguments.items()
            })

        # Preprocess the edge list to an optimal edge list
        # if this is enabled.
        if self._preprocess:
            # If any of the node types columns have been provided,
            # we compute the target node types column
            target_node_type_list_path = None
            if any(
                graph_arguments.get(column) is not None
                for column in (
                    "node_list_node_types_column_number",
                    "node_list_node_types_column",
                )
            ):
                target_node_type_list_path = self.get_preprocessed_graph_node_types_path()

            # If any of the edge types columns have been provided,
            # we compute the target edge types column
            target_edge_type_list_path = None
            if any(
                graph_arguments.get(column) is not None
                for column in (
                    "edge_list_edge_types_column_number",
                    "edge_list_edge_types_column",
                )
            ):
                target_edge_type_list_path = self.get_preprocessed_graph_edge_types_path()

            target_node_path = self.get_preprocessed_graph_nodes_path()
            target_edge_path = self.get_preprocessed_graph_edges_path()

            # If a node path was specified
            node_path = self.get_adjusted_graph_nodes_path()

            may_have_singletons = graph_arguments.get(
                "may_have_singletons", True
            ) and node_path is not None

            if not self.is_preprocessed():
                (
                    node_types_number,
                    nodes_number,
                    edge_types_number,
                    edges_number
                ) = edge_list_utils.build_optimal_lists_files(
                    # original_node_type_path,
                    # original_node_type_list_separator,
                    # original_node_types_column_number,
                    # original_node_types_column,
                    # original_numeric_node_type_ids,
                    # original_minimum_node_type_id,
                    # original_node_type_list_header,
                    # original_node_type_list_support_balanced_quotes,
                    # original_node_type_list_rows_to_skip,
                    # original_node_type_list_max_rows_number,
                    # original_node_type_list_comment_symbol,
                    # original_load_node_type_list_in_parallel,
                    # original_node_type_list_is_correct,
                    # node_types_number,
                    target_node_type_list_path=target_node_type_list_path,
                    target_node_type_list_separator='\t',
                    target_node_type_list_node_types_column_number=0,
                    original_node_path=node_path,
                    original_node_list_header=graph_arguments.get(
                        "node_list_header"
                    ),
                    original_node_list_support_balanced_quotes=graph_arguments.get(
                        "node_list_support_balanced_quotes"
                    ),
                    node_list_rows_to_skip=graph_arguments.get(
                        "node_list_rows_to_skip"
                    ),
                    node_list_is_correct=graph_arguments.get(
                        "node_list_is_correct"
                    ),
                    node_list_max_rows_number=graph_arguments.get(
                        "node_list_max_rows_number"
                    ),
                    node_list_comment_symbol=graph_arguments.get(
                        "node_list_comment_symbol"
                    ),
                    default_node_type=graph_arguments.get(
                        "default_node_type"
                    ),
                    original_nodes_column_number=graph_arguments.get(
                        "nodes_column_number"
                    ),
                    original_nodes_column=graph_arguments.get(
                        "nodes_column"
                    ),
                    original_node_types_separator=graph_arguments.get(
                        "node_types_separator"
                    ),
                    original_node_list_separator=graph_arguments.get(
                        "node_list_separator"
                    ),
                    original_node_list_node_types_column_number=graph_arguments.get(
                        "node_list_node_types_column_number"
                    ),
                    original_node_list_node_types_column=graph_arguments.get(
                        "node_list_node_types_column"
                    ),
                    nodes_number=graph_arguments.get("nodes_number"),
                    # original_minimum_node_id,
                    # original_numeric_node_ids,
                    # original_node_list_numeric_node_type_ids,
                    original_skip_node_types_if_unavailable=True,
                    # It make sense to load the node list in parallel only when
                    # you have to preprocess the node types, since otherwise the nodes number
                    # would be unknown.
                    original_load_node_list_in_parallel=target_node_type_list_path is not None,
                    maximum_node_id=graph_arguments.get(
                        "maximum_node_id"
                    ),
                    target_node_path=target_node_path,
                    target_node_list_separator='\t',
                    target_nodes_column=graph_arguments.get(
                        "nodes_column"
                    ),
                    target_nodes_column_number=0,
                    target_node_list_node_types_column_number=1,
                    target_node_types_separator="|",
                    # original_edge_type_path,
                    # original_edge_type_list_separator,
                    # original_edge_types_column_number,
                    # original_edge_types_column,
                    # original_numeric_edge_type_ids,
                    # original_minimum_edge_type_id,
                    # original_edge_type_list_header,
                    # edge_type_list_rows_to_skip,
                    # edge_type_list_max_rows_number,
                    # edge_type_list_comment_symbol,
                    # load_edge_type_list_in_parallel=True,
                    # edge_type_list_is_correct,
                    # edge_types_number,
                    target_edge_type_list_path=target_edge_type_list_path,
                    target_edge_type_list_separator='\t',
                    target_edge_type_list_edge_types_column_number=0,
                    original_edge_path=os.path.join(
                        self._cache_path, graph_arguments["edge_path"]),
                    original_edge_list_header=graph_arguments.get(
                        "edge_list_header"
                    ),
                    original_edge_list_support_balanced_quotes=graph_arguments.get(
                        "edge_list_support_balanced_quotes"
                    ),
                    original_edge_list_separator=graph_arguments.get(
                        "edge_list_separator"
                    ),
                    original_sources_column_number=graph_arguments.get(
                        "sources_column_number"
                    ),
                    original_sources_column=graph_arguments.get(
                        "sources_column"
                    ),
                    original_destinations_column_number=graph_arguments.get(
                        "destinations_column_number"
                    ),
                    original_destinations_column=graph_arguments.get(
                        "destinations_column"
                    ),
                    original_edge_list_edge_types_column_number=graph_arguments.get(
                        "edge_list_edge_types_column_number"
                    ),
                    original_edge_list_edge_types_column=graph_arguments.get(
                        "edge_list_edge_types_column"
                    ),
                    default_edge_type=graph_arguments.get(
                        "default_edge_type"
                    ),
                    original_weights_column_number=graph_arguments.get(
                        "weights_column_number"
                    ),
                    original_weights_column=graph_arguments.get(
                        "weights_column"
                    ),
                    default_weight=graph_arguments.get(
                        "default_weight"
                    ),
                    original_edge_list_numeric_node_ids=graph_arguments.get(
                        "edge_list_numeric_node_ids"
                    ),
                    skip_weights_if_unavailable=graph_arguments.get(
                        "skip_weights_if_unavailable"
                    ),
                    skip_edge_types_if_unavailable=graph_arguments.get(
                        "skip_edge_types_if_unavailable"
                    ),
                    edge_list_comment_symbol=graph_arguments.get(
                        "edge_list_comment_symbol"
                    ),
                    edge_list_max_rows_number=graph_arguments.get(
                        "edge_list_max_rows_number"
                    ),
                    edge_list_rows_to_skip=graph_arguments.get(
                        "edge_list_rows_to_skip"
                    ),
                    load_edge_list_in_parallel=True,
                    edges_number=graph_arguments.get("edges_number"),
                    target_edge_path=target_edge_path,
                    target_edge_list_separator='\t',
                    sort_temporary_directory=self._sort_temporary_directory,
                    directed=self._directed,
                    verbose=self._verbose > 0,
                    name=self._name,
                )
                # Store the obtained metadata
                self.store_preprocessed_metadata(
                    node_types_number,
                    nodes_number,
                    edge_types_number,
                    edges_number
                )
            # Load the stored metadata
            metadata = self.get_preprocessed_metadata()
            # If the node types are provided
            has_node_types = metadata["node_types_number"] is not None
            if has_node_types and self._load_node_types:
                node_types_arguments = {
                    "node_type_path": target_node_type_list_path,
                    "node_types_column_number": 0,
                    "node_type_list_is_correct": True,
                    "node_type_list_separator": "\t",
                    "node_types_separator": "|",
                    "node_list_node_types_column_number": 1,
                    "node_list_numeric_node_type_ids": True,
                    "skip_node_types_if_unavailable": True,
                }
            else:
                node_types_arguments = {}
            # If the nodes are to be loaded
            if self._load_nodes:
                nodes_arguments = {
                    "node_path": target_node_path,
                    "node_list_separator": "\t",
                    "nodes_column_number": 0,
                    "node_list_is_correct": True,
                    **node_types_arguments
                }
            else:
                nodes_arguments = {
                    "numeric_node_ids": True,
                }

            # If the edge types are provided
            has_edge_types = metadata["edge_types_number"] is not None
            if has_edge_types:
                edge_types_arguments = {
                    "edge_type_path": target_edge_type_list_path,
                    "edge_types_column_number": 0,
                    "edge_type_list_is_correct": True,
                    "edge_type_list_separator": "\t",
                    "edge_list_edge_types_column_number": 2,
                    "edge_list_numeric_edge_type_ids": True,
                    "skip_edge_types_if_unavailable": True,
                }
            else:
                edge_types_arguments = {}

            has_edge_weights = any(
                column in graph_arguments
                for column in (
                    "weights_column_number",
                    "weights_column",
                    "default_weight"
                )
            )
            if has_edge_weights and self._load_edge_weights:
                edge_weights_arguments = {
                    "weights_column_number": 2 + int(metadata["edge_types_number"] is not None),
                    "skip_weights_if_unavailable": True,
                }
            else:
                edge_weights_arguments = {}

            # Load the graph
            graph = Graph.from_csv(**{
                **metadata,
                **nodes_arguments,
                **edge_types_arguments,
                **edge_weights_arguments,

                "edge_path": target_edge_path,
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
                "may_have_singletons": may_have_singletons,
                "verbose": self._verbose > 0,
                "directed": self._directed,
                "name": self._name,
            })
        else:
            # Otherwise just load the graph.
            graph = Graph.from_csv(**{
                **{
                    key: os.path.join(self._cache_path, value)
                    if key.endswith("_path") else value
                    for key, value in graph_arguments.items()
                },
                "directed": self._directed,
                "verbose": self._verbose > 0,
                "name": self._name,
                **self._additional_graph_kwargs,
            })
        if self._automatically_enable_speedups_for_small_graphs and graph.get_unique_edges_number() < 50e6:
            graph.enable()
        return graph
