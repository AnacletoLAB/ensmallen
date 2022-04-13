"""Sub-module handling the retrieval and building of graphs from MonarchInitiative."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import pandas as pd
import compress_json
from .graph_repository import GraphRepository


class MonarchInitiativeGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        # We load the data that cannot be automatically scraped
        self._data = compress_json.local_load("monarch_initiative.json")
        # The arguments keys used to load this graph
        general_kwargs = {
            "sources_column": "subject",
            "destinations_column": "object",
            "edge_list_edge_types_column": "predicate",
            "nodes_column": "id",
            "node_list_node_types_column": "category",
            "node_types_separator": "|",
            "name": "Monarch"
        }
        # We extend the data through scraping the Google Bucket
        base_url = "https://storage.googleapis.com/monarch-ingest/"
        xml = pd.read_xml(base_url).fillna("NaN")
        xml = xml[xml.Key.str.endswith("/monarch-kg.tar.gz")]
        for path in xml.Key:
            version = path.split("/")[0]
            self._data["Monarch"][version] = {
                "urls": [
                    "{}/{}".format(base_url, path)
                ],
                "arguments": {
                    "edge_path": "monarch-kg/monarch-kg_edges.tsv",
                    "node_path": "monarch-kg/monarch-kg_nodes.tsv",
                    **general_kwargs
                }
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
        return "MonarchInitiative"

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
        Citations relative to the Kg graphs.
        """
        return [
            open(
                "{}/models/MonarchInitiative.bib".format(
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

        Implementative details
        -----------------------
        It is returned None because the path that is automatically
        used by downloader is sufficiently precise.
        """
        return None

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())
