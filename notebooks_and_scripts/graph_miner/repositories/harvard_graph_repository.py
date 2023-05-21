"""Sub-module handling the retrieval and building of graphs from HarvardGraphRepository."""
from typing import List
import compress_json
from .graph_repository import GraphRepository


class HarvardGraphRepository(GraphRepository):

    def __init__(self):
        """Create new Harvard Graph Repository object."""
        super().__init__()
        self._data = compress_json.local_load("harvard.json")

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
        return partial_graph_name

    def get_formatted_repository_name(self) -> str:
        """Return formatted repository name."""
        return "Harvard"

    def get_graph_arguments(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return arguments for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel arguments for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The arguments list to use to build the graph.
        """
        return self._data[graph_name][version]["arguments"]

    def get_graph_versions(
        self,
        graph_name: str,
    ) -> List[str]:
        """Return list of versions of the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve versions for.

        Returns
        -----------------------
        List of versions for the given graph.
        """
        return list(self._data[graph_name].keys())

    def get_graph_urls(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return urls for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return self._data[graph_name][version]["urls"]

    def get_graph_references(self, graph_name: str, version: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        Citations relative to the Yue graphs.
        """
        return []

    def get_graph_paths(self, graph_name: str, version: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel paths for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        return self._data[graph_name][version].get("paths")

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())
