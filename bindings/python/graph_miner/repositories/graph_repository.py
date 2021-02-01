"""Class providing an abstract graph repository."""
from typing import List
from downloaders import BaseDownloader


class GraphRepository:
    def __init__(self):
        """Create new Graph Repository object."""
        self._downloader = BaseDownloader(
            process_number=1,
            verbose=2,
        )

    def build_stored_graph_name(self, partial_graph_name: str) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        partial_graph_name: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        raise NotImplementedError(
            "The method build_stored_graph_name must be implemented in child classes."
        )

    def is_graph_cached(self, graph_name: str) -> bool:
        """Return boolean representing if graph is cached.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is cached.

        Returns
        -----------------------
        Wether the cache if cached.
        """
        raise NotImplementedError(
            "The method is_graph_cached must be implemented in child classes."
        )

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        raise NotImplementedError(
            "The method get_graph_list must be implemented in child classes."
        )

    def get_uncached_graph_list(self) -> List[str]:
        """Return graphs to be parsed."""
        return [
            graph_name
            for graph_name in self.get_graph_list()
            if not self.is_graph_cached(graph_name)
        ]

    def get_graph_urls(self, graph_name: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        raise NotImplementedError(
            "The method get_graph_urls must be implemented in child classes."
        )

    def get_graph_paths(self, graph_name: str, urls: List[str]) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        urls: List[str],
            Urls from where to download the graphs.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        raise NotImplementedError(
            "The method get_graph_paths must be implemented in child classes."
        )

    def download(self, graph_name: str):
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve.
        """
        urls = self.get_graph_urls(graph_name)
        self._downloader.download(urls, self.get_graph_paths(graph_name, urls))
