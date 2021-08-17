"""Sub-module handling the retrieval and building of graphs from KGHUB."""
from typing import List, Dict
import os
import requests
import pandas as pd
import string
import compress_json
from downloaders import BaseDownloader
from .graph_repository import GraphRepository


class PheKnowLatorKGGraphrepository(GraphRepository):

    def __init__(self):
        """Create new PheKnowLator Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the PheKnowLatorKG repository."""
        json_url = "https://storage.googleapis.com/pheknowlator/pheknowlator_builds.json"

        downloader = BaseDownloader(verbose=2, cache=False)
        metadata_report = downloader.download(json_url)
        all_metadata = compress_json.load(metadata_report.iloc[0].destination)

        graph_name = "PheKnowLator"
        stored_graph_name = graph_name
        mined_data = {
            stored_graph_name: {}
        }

        for version, version_data in all_metadata.items():
            if not isinstance(version_data, dict):
                continue
            for sub_version, url in version_data.items():
                if url is None:
                    continue
                full_version_code = "{version}.{sub_version}".format(
                    version=version,
                    sub_version=sub_version
                )
                mined_data[stored_graph_name][full_version_code] = {
                    "urls": [url],
                    "paths": [
                        "edge_list.tsv"
                    ],
                    "arguments": {
                        "edge_path": "edge_list.tsv",
                        "name": graph_name,
                        "sources_column": "subject",
                        "destinations_column": "object",
                        "edge_list_edge_types_column": "predicate",
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
        return "PheKnowLatorKG"

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

        Implementative details
        -----------------------
        It is returned None because the path that is automatically
        used by downloader is sufficiently precise.
        """
        return self._data[graph_name][version]["paths"]

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
        Citations relative to the PheKnowLator.
        """
        return [
            open(
                "{}/models/pheknowlator.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read()
        ]

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())
