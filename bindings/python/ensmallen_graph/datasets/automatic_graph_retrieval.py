import os
import shutil
from typing import Callable, Dict, List, Optional

import compress_json
from downloaders import BaseDownloader
from environments_utils import is_windows

from ..ensmallen_graph import EnsmallenGraph, edge_list_utils


class AutomaticallyRetrievedGraph:
    def __init__(
        self,
        graph_name: str,
        version: str,
        dataset: str,
        directed: bool = False,
        preprocess: bool = True,
        verbose: int = 2,
        cache: bool = True,
        cache_path: str = "graphs",
        callbacks: List[Callable] = (),
        callbacks_arguments: List[Dict] = (),
        additional_graph_kwargs: Dict = None
    ):
        """Create new automatically retrieved graph.

        Parameters
        -------------------
        graph_name: str,
            The name of the graph to be retrieved and loaded.
        version: str,
            The version of the graph to be retrieved.
        dataset: str,
            Name of the dataset to load data from.
        directed: bool = False,
            Whether to load the graph as directed or undirected.
            By default false.
        preprocess: bool = True,
            Whether to preprocess the node list and edge list
            to be loaded optimally in both time and memory.
        verbose: int = 2,
            Whether to show loading bars.
        cache: bool = True,
            Whether to use cache, i.e. download files only once
            and preprocess them only once.
        cache_path: str = "graphs",
            Where to store the downloaded graphs.
        callbacks: List[Callable] = (),
            Eventual callbacks to call after download files.
        callbacks_arguments: List[Dict] = (),
            Eventual arguments for callbacks.
        additional_graph_kwargs: Dict = None,
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
            all_versions = compress_json.local_load(os.path.join(
                dataset,
                "{}.json.gz".format(graph_name)
            ))

            if version not in all_versions:
                raise ValueError(
                    (
                        "Requested graph `{}` from dataset `{}` is ",
                        "not available in the requested version `{}`."
                    ).format(graph_name, dataset, version)
                )
                
            self._graph = all_versions[version]
        except FileNotFoundError:
            raise ValueError(
                (
                    "Requested graph `{}` is not currently available.\n"
                    "Open an issue on the EnsmallenGraph repository to ask "
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
        self._directed = directed
        self._preprocess = preprocess
        self._name = graph_name
        self._version = version
        self._cache = cache
        self._verbose = verbose
        self._callbacks = callbacks
        if additional_graph_kwargs is None:
            additional_graph_kwargs = {}
        self._additional_graph_kwargs = additional_graph_kwargs
        self._callbacks_arguments = callbacks_arguments
        self._cache_path = os.path.join(cache_path, graph_name)
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
        """Return the path to file where the metadata of the graph are stored."""
        return os.path.join(
            self.get_preprocessed_graph_directory_path(),
            "nodes.tsv"
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

    def __call__(self) -> EnsmallenGraph:
        """Return EnsmallenGraph containing required graph."""
        paths = self._graph.get("paths", None)
        if paths is not None:
            paths = [
                os.path.join(self._cache_path, path)
                for path in paths
            ]

        root = self.get_preprocessed_graph_directory_path()

        if not self._cache and os.path.exists(root):
            shutil.rmtree(root)

        # Download the necessary data
        self._downloader.download(
            self._graph["urls"],
            paths
        )

        os.makedirs(
            root,
            exist_ok=True
        )

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
                column in self._graph["arguments"]
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
                column in self._graph["arguments"]
                for column in (
                    "edge_list_edge_types_column_number",
                    "edge_list_edge_types_column",
                )
            ):
                target_edge_type_list_path = self.get_preprocessed_graph_edge_types_path()

            target_node_path = self.get_preprocessed_graph_nodes_path()
            target_edge_path = self.get_preprocessed_graph_edges_path()

            # If a node path was specified
            node_path = self._graph["arguments"].get(
                "node_path"
            )

            # And it is not None
            if node_path is not None:
                # We add the cache path to it
                node_path = os.path.join(
                    self._cache_path,
                    self._graph["arguments"]["node_path"]
                )

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
                    # original_node_types_ids_column_number,
                    # original_node_types_ids_column,
                    # original_numeric_node_type_ids,
                    # original_minimum_node_type_id,
                    # original_node_type_list_header,
                    # original_node_type_list_rows_to_skip,
                    # original_node_type_list_max_rows_number,
                    # original_node_type_list_comment_symbol,
                    # original_load_node_type_list_in_parallel,
                    # original_node_type_list_is_correct,
                    # node_types_number,
                    target_node_type_list_path=target_node_type_list_path,
                    target_node_type_list_separator="\t",
                    target_node_types_ids_column_number=0,
                    target_node_type_list_node_types_column_number=1,
                    original_node_path=node_path,
                    original_node_list_header=self._graph["arguments"].get(
                        "node_header"
                    ),
                    node_list_rows_to_skip=self._graph["arguments"].get(
                        "node_list_rows_to_skip"
                    ),
                    node_list_is_correct=self._graph["arguments"].get(
                        "node_list_is_correct"
                    ),
                    node_list_max_rows_number=self._graph["arguments"].get(
                        "node_list_max_rows_number"
                    ),
                    node_list_comment_symbol=self._graph["arguments"].get(
                        "node_list_comment_symbol"
                    ),
                    default_node_type=self._graph["arguments"].get(
                        "default_node_type"
                    ),
                    original_nodes_column_number=self._graph["arguments"].get(
                        "nodes_column_number"
                    ),
                    original_nodes_column=self._graph["arguments"].get(
                        "nodes_column"
                    ),
                    original_node_types_separator=self._graph["arguments"].get(
                        "node_types_separator"
                    ),
                    original_node_list_separator=self._graph["arguments"].get(
                        "node_list_separator"
                    ),
                    original_node_list_node_types_column_number=self._graph["arguments"].get(
                        "node_list_node_types_column_number"
                    ),
                    original_node_list_node_types_column=self._graph["arguments"].get(
                        "node_list_node_types_column"
                    ),
                    original_node_ids_column=self._graph["arguments"].get(
                        "node_ids_column"
                    ),
                    original_node_ids_column_number=self._graph["arguments"].get(
                        "node_ids_column_number"
                    ),
                    nodes_number=self._graph["arguments"].get("nodes_number"),
                    # original_minimum_node_id,
                    # original_numeric_node_ids,
                    # original_node_list_numeric_node_type_ids,
                    original_skip_node_types_if_unavailable=True,
                    original_load_node_list_in_parallel=True,
                    maximum_node_id=self._graph["arguments"].get(
                        "maximum_node_id"
                    ),
                    target_node_path=target_node_path,
                    target_node_list_separator="\t",
                    target_nodes_column=self._graph["arguments"].get(
                        "nodes_column"
                    ),
                    target_node_ids_column_number=0,
                    target_nodes_column_number=1,
                    target_node_list_node_types_column_number=2,
                    target_node_types_separator="|",
                    # original_edge_type_path,
                    # original_edge_type_list_separator,
                    # original_edge_types_column_number,
                    # original_edge_types_column,
                    # original_edge_types_ids_column_number,
                    # original_edge_types_ids_column,
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
                    target_edge_type_list_separator="\t",
                    target_edge_type_list_edge_types_column_number=1,
                    target_edge_types_ids_column_number=0,
                    original_edge_path=os.path.join(
                        self._cache_path, self._graph["arguments"]["edge_path"]),
                    original_edge_list_header=self._graph["arguments"].get(
                        "edge_list_header"
                    ),
                    original_edge_list_separator=self._graph["arguments"].get(
                        "edge_list_separator"
                    ),
                    original_sources_column_number=self._graph["arguments"].get(
                        "sources_column_number"
                    ),
                    original_sources_column=self._graph["arguments"].get(
                        "sources_column"
                    ),
                    original_destinations_column_number=self._graph["arguments"].get(
                        "destinations_column_number"
                    ),
                    original_destinations_column=self._graph["arguments"].get(
                        "destinations_column"
                    ),
                    original_edge_list_edge_types_column_number=self._graph["arguments"].get(
                        "edge_list_edge_types_column_number"
                    ),
                    original_edge_list_edge_types_column=self._graph["arguments"].get(
                        "edge_list_edge_types_column"
                    ),
                    default_edge_type=self._graph["arguments"].get(
                        "default_edge_type"
                    ),
                    original_weights_column_number=self._graph["arguments"].get(
                        "weights_column_number"
                    ),
                    original_weights_column=self._graph["arguments"].get(
                        "weights_column"
                    ),
                    default_weight=self._graph["arguments"].get(
                        "default_weight"
                    ),
                    original_edge_list_numeric_node_ids=self._graph["arguments"].get(
                        "edge_list_numeric_node_ids"
                    ),
                    skip_weights_if_unavailable=True,
                    skip_edge_types_if_unavailable=True,
                    edge_list_comment_symbol=self._graph["arguments"].get(
                        "edge_list_comment_symbol"
                    ),
                    edge_list_max_rows_number=self._graph["arguments"].get(
                        "edge_list_max_rows_number"
                    ),
                    edge_list_rows_to_skip=self._graph["arguments"].get(
                        "edge_list_rows_to_skip"
                    ),
                    load_edge_list_in_parallel=True,
                    edges_number=self._graph["arguments"].get("edges_number"),
                    target_edge_path=target_edge_path,
                    target_edge_list_separator="\t",
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
            # Load the graph
            return EnsmallenGraph.from_csv(**{
                **metadata,
                "node_type_path": target_node_type_list_path,
                "node_types_ids_column_number": 0,
                "node_types_column_number": 1,
                "node_type_list_is_correct": True,

                "node_path": target_node_path,
                "node_list_is_correct": True,
                "node_types_separator": "|",
                "node_ids_column_number": 0,
                "nodes_column_number": 1,
                "node_list_node_types_column_number": None if metadata["node_types_number"] is None else 2,
                "nodes_number": metadata["nodes_number"],
                "node_list_numeric_node_type_ids": True,
                "skip_node_types_if_unavailable": True,

                "edge_type_path": target_edge_type_list_path,
                "edge_types_ids_column_number": 0,
                "edge_types_column_number": 1,
                "edge_type_list_is_correct": True,

                "edge_path": target_edge_path,
                "edge_list_header": False,
                "edge_ids_column_number": 0,
                "sources_column_number": 1,
                "destinations_column_number": 2,
                "edge_list_edge_types_column_number": None if metadata["edge_types_number"] is None else 3,
                "weights_column_number": 3 + int(metadata["edge_types_number"] is not None),
                "edge_list_numeric_edge_type_ids": True,
                "edge_list_numeric_node_ids": True,
                "skip_weights_if_unavailable": True,
                "skip_edge_types_if_unavailable": True,
                "edge_list_is_complete": True,
                "edge_list_may_contain_duplicates": False,
                "edge_list_is_sorted": True,
                "edge_list_is_correct": True,
                "edges_number": metadata["edges_number"],
                "verbose": self._verbose > 0,
                "directed": self._directed,
                "name": self._name,
                **self._additional_graph_kwargs,
            })

        # Otherwise just load the graph.
        return EnsmallenGraph.from_csv(**{
            **{
                key: os.path.join(self._cache_path, value)
                if key.endswith("_path") else value
                for key, value in self._graph["arguments"].items()
            },
            "directed": self._directed,
            "verbose": self._verbose > 0,
            "name": self._name,
            **self._additional_graph_kwargs,
        })
