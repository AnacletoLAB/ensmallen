import os
from typing import Callable, List, Dict
import compress_json
from downloaders import BaseDownloader

from ..ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


class AutomaticallyRetrievedGraph:
    def __init__(
        self,
        graph_name: str,
        dataset: str,
        directed: bool = False,
        verbose: int = 2,
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
        dataset: str,
            Name of the dataset to load data from.
        directed: bool = False,
            Whether to load the graph as directed or undirected.
            By default false.
        verbose: int = 2,
            Whether to show loading bars.
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
        """
        try:
            self._graph = compress_json.local_load(os.path.join(
                dataset,
                "{}.json.gz".format(graph_name)
            ))
        except FileNotFoundError:
            raise ValueError(
                (
                    "Requested graph `{}` is not currently available.\n"
                    "Open an issue on the EnsmallenGraph repository to ask "
                    "for this graph to be added."
                ).format(graph_name)
            )
        self._directed = directed
        self._name = graph_name
        self._verbose = verbose
        self._callbacks = callbacks
        if additional_graph_kwargs is None:
            additional_graph_kwargs = {}
        self._additional_graph_kwargs = additional_graph_kwargs
        self._callbacks_arguments = callbacks_arguments
        self._cache_path = os.path.join(cache_path, graph_name)
        self._downloader = BaseDownloader(
            auto_extract=True,
            target_directory=self._cache_path,
            verbose=self._verbose,
            process_number=1
        )

    def __call__(self) -> EnsmallenGraph:
        """Return EnsmallenGraph containing required graph."""
        paths = self._graph.get("paths", None)
        if paths is not None:
            paths = [
                os.path.join(self._cache_path, path)
                for path in paths
            ]
        self._downloader.download(
            self._graph["urls"],
            paths
        )
        for callback, arguments in zip(self._callbacks, self._callbacks_arguments):
            callback(**{
                key: os.path.join(self._cache_path, value)
                if key.endswith("_path") else value
                for key, value in arguments.items()
            })
        return EnsmallenGraph.from_unsorted_csv(**{
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
