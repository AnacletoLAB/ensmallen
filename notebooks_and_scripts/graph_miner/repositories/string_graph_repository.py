"""Sub-module handling the retrieval and building of graphs from KGHUB."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import pandas as pd
import string
from .graph_repository import GraphRepository


class StringGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the KGHub repository."""
        species_11_0 = pd.read_csv(
            "https://stringdb-static.org/download/species.v11.0.txt",
            sep="\t"
        )
        species_11_5 = pd.read_csv(
            "https://stringdb-static.org/download/species.v11.5.txt",
            sep="\t"
        )

        mined_data = {}

        graph_url_pattern = "https://stringdb-static.org/download/protein.links.v{version}/{taxon_id}.protein.links.v{version}.txt.gz"

        for version, species in (
            ("11.0", species_11_0),
            ("11.5", species_11_5),
        ):
            for _, row in species.iterrows():
                graph_name = self.build_stored_graph_name(row.STRING_name_compact)
                if graph_name not in mined_data:
                    mined_data[graph_name] = {}
                taxon_id = row[0]
                graph_url = graph_url_pattern.format(
                    taxon_id=taxon_id,
                    version=version
                )
                mined_data[graph_name][version] = {
                    "urls": [graph_url],
                    "arguments": {
                        "edge_path": "{taxon_id}.protein.links.v{version}.txt".format(
                            taxon_id=taxon_id,
                            version=version
                        ),
                        "sources_column": "protein1",
                        "destinations_column": "protein2",
                        "weights_column": "combined_score",
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
        for target in string.punctuation:
            partial_graph_name = partial_graph_name.replace(target, " ")
        return "".join([
            term.capitalize()
            for term in partial_graph_name.split()
        ])

    def get_formatted_repository_name(self) -> str:
        """Return formatted repository name."""
        return "STRING"

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
        Citations relative to the STRING.
        """
        return [
            open(
                "{}/models/string_citation.bib".format(
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
