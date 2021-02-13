"""Sub-module handling the retrieval and building of graphs from LINQS."""
from typing import List, Dict
import os
import compress_json
import pandas as pd
from .graph_repository import GraphRepository
from .models.parse_linqs import (
    parse_linqs_incidence_matrix,
    parse_linqs_pubmed_incidence_matrix
)


class LINQSGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = compress_json.local_load("linqs.json")
        self._parse = {
            "parse_linqs_incidence_matrix": parse_linqs_incidence_matrix,
            "parse_linqs_pubmed_incidence_matrix": parse_linqs_pubmed_incidence_matrix
        }

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
        return "LINQS"

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
                "{}/models/{}.bib".format(
                    os.path.dirname(os.path.abspath(__file__)),
                    citation
                ),
                "r"
            ).read()
            for citation in graph_data[1]["citations"]
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
        return None

    def build_graph_parameters(
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
            **super().build_graph_parameters(
                graph_name,
                edge_path,
                node_path
            ),
            "sources_column": "subject",
            "destinations_column": "object",
            "weights_column": "weight",
            "default_weight": 1,
            "edge_types_column": "edge_type",
            "node_types_column": "node_type",
            "nodes_column": "id",
            "edge_separator": "\t",
            "node_separator": "\t",
            "skip_weights_if_unavailable": True
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
        return "\n".join(self._data[graph_name]["imports"])

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
        return self._data[graph_name]["description"]

    def get_callbacks(self, graph_name: str) -> str:
        """Return callbacks to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        callbacks.
        """
        return "\n".join(self._data[graph_name]["callbacks"])

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
            self.build_stored_graph_name(graph_name).lower(),
            "nodes.tsv"
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
            self.build_stored_graph_name(graph_name).lower(),
            "edges.tsv"
        )

    def download(self, graph_data, graph_name: str) -> pd.DataFrame:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Data of the graph to retrieve.
        graph_name: str,
            Name of the graph to retrieve.

        Returns
        -----------------------
        Dataframe with download metadata.
        """
        report = super().download(graph_data, graph_name)
        self._parse.get(self._data[graph_name]["callback"])(
            **{
                parameter: os.path.join(
                    self.repository_package_name,
                    value
                )
                for parameter, value in self._data[graph_name]["callback_arguments"].items()
            },
            node_list_path=self.get_node_list_path(graph_name, report),
            edge_list_path=self.get_edge_list_path(graph_name, report),
        )
        return report
