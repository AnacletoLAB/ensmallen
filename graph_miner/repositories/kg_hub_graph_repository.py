"""Sub-module handling the retrieval and building of graphs from KGHUB."""
from typing import List, Dict
import os
import compress_json
import pandas as pd
from .graph_repository import GraphRepository


class KGHubGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = compress_json.local_load("kg_hub.json")

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
        return "KGHub"

    def get_graph_name(self, graph_data) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        graph_data,
            Data loaded for given graph.

        Returns
        -----------------------
        Complete name of the graph.
        """
        return graph_data[0]

    def get_graph_urls(self, graph_data) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Graph data to use to retrieve the URLs.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return graph_data[1]["urls"]

    def get_graph_citations(self, graph_data) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Graph data to use to retrieve the citations.

        Returns
        -----------------------
        Citations relative to the STRING graphs.
        """
        return [
            open(
                "{}/models/kg_hub.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read()
        ]

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
        if graph_name == "KGCOVID19":
            return None
        return [
            self.get_edge_list_path(graph_name, None),
            self.get_node_list_path(graph_name, None),
        ]

    def from_integer_sorted_parameters(
        self,
        graph_name: str,
        edge_path: str,
        node_path: str = None,
    ) -> Dict:
        """Return dictionary with kwargs to load graph.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to load.
        edge_path: str,
            Path from where to load the edge list.
        node_path: str = None,
            Optionally, path from where to load the nodes.

        Returns
        -----------------------
        Dictionary to build the graph object.
        """
        return {
            **super().from_integer_sorted_parameters(
                graph_name,
                edge_path,
                node_path
            ),
            **{
                key: value
                for key, value in self._data[graph_name]["arguments"].items()
                if not key.endswith("_path")
            }
        }

    def get_graph_list(self) -> List:
        """Return list of graph data."""
        return list(self._data.items())

    def get_imports(self, graph_name: str) -> str:
        """Return imports to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        Imports.
        """
        return ""

    def get_description(self, graph_name: str) -> str:
        """Return description to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        description.
        """
        return ""

    def get_node_list_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the node files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the node files.
        """
        return os.path.join(
            self.repository_package_name,
            self._data[graph_name]["arguments"]["node_path"]
        )

    def get_edge_list_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the edge files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the edge files.
        """
        return os.path.join(
            self.repository_package_name,
            self._data[graph_name]["arguments"]["edge_path"]
        )
