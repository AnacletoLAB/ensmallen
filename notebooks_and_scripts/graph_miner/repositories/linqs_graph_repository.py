"""Sub-module handling the retrieval and building of graphs from LINQSGraphRepository."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import pandas as pd
import shutil
import compress_json
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

    def get_node_path(
        self,
        graph_name: str,
        version: str
    ) -> str:
        """Return path from where to load the node files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The path from where to load the node files.
        """
        return "nodes.tsv"

    def get_edge_path(
        self,
        graph_name: str,
        version: str
    ) -> str:
        """Return path from where to load the edge files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The path from where to load the edge files.
        """
        return "edges.tsv"

    def get_imports(self, graph_name: str, version: str) -> str:
        """Return imports to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        Imports.
        """
        return "\n".join(self._data[graph_name][version]["imports"])

    def get_description(self, graph_name: str, version: str) -> str:
        """Return description to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        description.
        """
        return self._data[graph_name][version]["description"]

    def get_callbacks(self, graph_name: str, version: str) -> List[str]:
        """Return callbacks to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        callbacks.
        """
        return [self._data[graph_name][version]["callback"]]

    def get_callbacks_arguments(self, graph_name: str, version: str) -> List[Dict]:
        """Return dictionary with list of arguments to pass to callbacks.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph to retrieve.

        Returns
        -----------------------
        Arguments to pass to callbacks.
        """
        return [dict(
            **self._data[graph_name][version]["callback_arguments"],
            node_path=self.get_node_path(graph_name, None),
            edge_path=self.get_edge_path(graph_name, None),
        )]

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
        return {
            "node_path": self.get_node_path(graph_name, None),
            "edge_path": self.get_edge_path(graph_name, None),
            "sources_column": "subject",
            "destinations_column": "object",
            "weights_column": "weight",
            "default_weight": 1.0,
            "edge_list_edge_types_column": "edge_type",
            "node_list_node_types_column": "node_type",
            "nodes_column": "id",
            "edge_separator": "\t",
            "node_separator": "\t",
            "skip_weights_if_unavailable": True
        }

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
        return [
            open(
                "{}/models/{}.bib".format(
                    os.path.dirname(os.path.abspath(__file__)),
                    citation
                ),
                "r"
            ).read()
            for citation in self._data[graph_name]["latest"]["references"]
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

        Implementative details
        -----------------------
        It is returned None because the path that is automatically
        used by downloader is sufficiently precise.
        """
        return None

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())

    def build_all(self):
        """Build graph retrieval methods."""
        super().build_all()
        shutil.copyfile(
            "graph_miner/repositories/models/parse_linqs.py",
            os.path.join(
                "../bindings/python/ensmallen_graph/datasets",
                self.repository_package_name,
                "parse_linqs.py"
            )
        )
