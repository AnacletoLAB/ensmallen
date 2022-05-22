"""Sub-module handling the retrieval and building of graphs from KGHUB."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import pandas as pd
from .graph_repository import GraphRepository
from ..utils import get_cached_page


class KGHubGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the KGHub repository."""
        mined_data = {}
        black_list = ["README", ".."]
        graph_names_mapping = {
            "kg-covid-19": "KGCOVID19",
            "kg-microbe": "KGMicrobe",
            "KG-IDG": "KGIDG",
            "kg-phenio": "KGPhenio",
            "eco-kg": "EcoKG",
            "sldb": "SLDB"
        }
        root_pattern = "https://kg-hub.berkeleybop.io/{lower_graph_name}/"
        graph_url_pattern = "https://kg-hub.berkeleybop.io/{lower_graph_name}/{version}/{graph_name}.tar.gz"

        for graph_name in graph_names_mapping:
            url = root_pattern.format(lower_graph_name=graph_name.lower())
            anchors = BeautifulSoup(
                get_cached_page(url),
                "lxml"
            ).find_all("a")
            versions = [
                anchor.text
                for anchor in anchors
                if anchor.text not in black_list
            ]
            callable_graph_name = graph_names_mapping[graph_name]
            mined_data[callable_graph_name] = {}
            for version in versions:
                graph_url = graph_url_pattern.format(
                    lower_graph_name=graph_name.lower(),
                    graph_name=graph_name,
                    version=version
                )
                if version == "placeholder":
                    continue
                mined_data[callable_graph_name][version] = {
                    "urls": [graph_url],
                    "arguments": {
                        "edge_path": "{}/merged-kg_edges.tsv".format(graph_name),
                        "node_path": "{}/merged-kg_nodes.tsv".format(graph_name),
                        "name": graph_name,
                        "sources_column": "subject",
                        "destinations_column": "object",
                        "edge_list_edge_types_column": "predicate",
                        "nodes_column": "id",
                        "node_list_node_types_column": "category",
                        "node_types_separator": "|",
                        "node_list_is_correct": True,
                        "edge_list_is_correct": True,
                    }
                }
        return mined_data

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
        if graph_name == "KGCOVID19":
            return [
                open(
                    "{}/models/KGCOVID19.bib".format(
                        os.path.dirname(os.path.abspath(__file__)),
                        graph_name
                    ),
                    "r"
                ).read()
            ]
        if graph_name == "KGMicrobe":
            return [
                open(
                    "{}/models/kgmicrobe.bib".format(
                        os.path.dirname(os.path.abspath(__file__)),
                        graph_name
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
