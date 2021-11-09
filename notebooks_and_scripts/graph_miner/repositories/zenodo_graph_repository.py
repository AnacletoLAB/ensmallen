"""Sub-module handling the retrieval and building of graphs from ZenodoGraphRepository."""
from typing import List, Dict
import os
import compress_json
from bs4 import BeautifulSoup
from .graph_repository import GraphRepository
from ..utils import get_cached_page


class ZenodoGraphRepository(GraphRepository):

    def __init__(self):
        """Create new Zenodo Graph Repository object."""
        super().__init__()
        self._data = {
            **compress_json.local_load("zenodo.json"),
            **self.load_wikidata_metatada()
        }

    def load_wikidata_metatada(self, ) -> Dict:
        """Return WikiData graphs informations."""
        soup = BeautifulSoup(
            get_cached_page("https://zenodo.org/record/2539424#.YYp0otbMKDV"),
            "lxml"
        )
        download_urls = [
            link["href"]
            for link in soup.find_all("link")
            if link["href"].endswith(".csv.gz")
        ]
        wikidata_metadata = {}
        graph_name_pattern = "WikiLink{nation_code}"
        specific_graph_name_pattern = "WikiLink{nation_code}{year}"
        simplified_file_name_pattern = "{year}.csv.gz"
        uncomprossed_simplified_file_name_pattern = "{year}.csv"
        for download_url in download_urls:
            file_name = download_url.split("/")[-1]
            nation_code = file_name[:2].upper()
            year = file_name.split(".")[2].split("-")[0]
            simplified_file_name = simplified_file_name_pattern.format(
                year=year
            )
            uncomprossed_simplified_file_name = uncomprossed_simplified_file_name_pattern.format(
                year=year
            )
            graph_name = graph_name_pattern.format(
                nation_code=nation_code
            )
            specific_graph_name = specific_graph_name_pattern.format(
                nation_code=nation_code,
                year=year
            )
            this_graph_metadata = wikidata_metadata.get(
                graph_name,
                {}
            )

            this_graph_metadata[year] = {
                "urls": [
                    download_url
                ],
                "paths": [
                    simplified_file_name
                ],
                "arguments": {
                    "edge_path": uncomprossed_simplified_file_name,
                    "edge_list_separator": "\t",
                    "name": specific_graph_name,
                    "sources_column": "page_title_from",
                    "destinations_column_number": "page_title_to"
                }
            }

            wikidata_metadata[graph_name] = this_graph_metadata

        return wikidata_metadata

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
        return "Zenodo"

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
        Citations relative to the WikiData and Zenodo graphs.
        """
        if "wikilink" in graph_name.lower():
            return [
                open(
                    "{}/models/wikilinks.bib".format(
                        os.path.dirname(os.path.abspath(__file__))
                    ),
                    "r"
                ).read()
            ]
        if "GiantTN" == graph_name:
            return [
                open(
                    "{}/models/giant.bib".format(
                        os.path.dirname(os.path.abspath(__file__))
                    ),
                    "r"
                ).read()
            ]

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
