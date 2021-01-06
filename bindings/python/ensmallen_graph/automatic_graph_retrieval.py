import os

import compress_json
from downloaders import BaseDownloader

from .ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


class AutomaticallyRetrievedGraph:
    def __init__(
        self,
        name: str,
        directed: bool = False,
        verbose: int = 2,
        cache_path: str = "graphs"
    ):
        """Create new automatically retrieved graph.

        Parameters
        -------------------
        name: str,
            The name of the graph to be retrieved and loaded.
        directed: bool = False,
            Wether to load the graph as directed or undirected.
            By default false.
        verbose: int = 2,
            Wether to show loading bars.
        cache_path: str = "graphs",
            Where to store the downloaded graphs.

        Raises
        -------------------
        ValueError,
            If the given graph name is not available.
        """
        graphs = compress_json.local_load("graphs.json")
        if name not in graphs:
            raise ValueError(
                (
                    "Requested graph `{}` is not currently available.\n"
                    "Open an issue on the EnsmallenGraph repository to ask "
                    "for this graph to be added."
                ).format(name)
            )
        self._graph = graphs[name]
        self._directed = directed
        self._name = name
        self._verbose = verbose
        self._cache_path = os.path.join(cache_path, name)
        self._downloader = BaseDownloader(
            auto_extract=True,
            target_directory=self._cache_path,
            verbose=self._verbose,
            process_number=1
        )

    def __call__(self) -> EnsmallenGraph:
        """Return EnsmallenGraph containing required graph."""
        self._downloader.download(self._graph["urls"])
        return EnsmallenGraph.from_unsorted_csv(
            **{
                key: os.path.join(self._cache_path, value)
                if key.endswith("_path") else value
                for key, value in self._graph["arguments"].items()
            },
            directed=self._directed,
            verbose=self._verbose > 0,
            name=self._name
        )
