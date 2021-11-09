"""Sub-module handling the retrieval and building of graphs from WikiData."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import pandas as pd
from tqdm.auto import tqdm
from .graph_repository import GraphRepository


def ntriples_filter(anchor_text: str) -> bool:
    """Returns whether to keep a given anchor for wikidata.

    Parameters
    -------------------------
    anchor_text: str
        The anchor text
    """
    return anchor_text.endswith(".nt.gz")


def root_white_list_filter(anchor_text: str) -> bool:
    """Returns whether to keep a given anchor for wikidata.

    Parameters
    -------------------------
    anchor_text: str
        The anchor text
    """
    return ntriples_filter(anchor_text) or anchor_text.strip("/").isnumeric()


def get_wikidata_anchors_from_url(url: str, is_root: bool) -> List[str]:
    """Returns anchor names from the given wikidata url.

    Parameters
    -------------------------
    url: str
        The url from where to retrieve the data.
    is_root: bool
        Whether this url is root.
    """
    return [
        anchor.text
        for anchor in BeautifulSoup(
            requests.get(url).text,
            "lxml"
        ).find_all("a")
        if is_root and root_white_list_filter(anchor.text) or not is_root and ntriples_filter(anchor.text)
    ]


class WikiDataGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the WikiData repository."""
        mined_data = {
            "WikiData": {}
        }
        url = "https://dumps.wikimedia.org/wikidatawiki/entities/"
        sub_url_pattern = "https://dumps.wikimedia.org/wikidatawiki/entities/{}"

        anchors = get_wikidata_anchors_from_url(url, is_root=True)
        all_versions = [
            dict(
                file_name=sub_anchor.replace(".nt.gz", ".nt"),
                url=sub_url_pattern.format(anchor + sub_anchor),
                version=sub_anchor.replace(".nt.gz", "")
            )
            for anchor in tqdm(
                anchors,
                desc="Retrieve the anchors from WikiData dumps",
                leave=False
            )
            if not ntriples_filter(anchor)
            for sub_anchor in get_wikidata_anchors_from_url(
                sub_url_pattern.format(anchor),
                is_root=False
            )
        ] + [
            dict(
                file_name=anchor.replace(".nt.gz", ".nt"),
                url=sub_url_pattern.format(anchor),
                version=anchor.replace(".nt.gz", "")
            )
            for anchor in anchors
            if ntriples_filter(anchor)
        ]

        for graph_data in all_versions:
            mined_data["WikiData"][graph_data["version"]] = {
                "urls": [graph_data["url"]],
                "arguments": {
                    "edge_path": graph_data["file_name"],
                    "name": "Wikidata",
                    "sources_column_number": 0,
                    "edge_list_edge_types_column_number": 1,
                    "destinations_column_number": 2,
                    "edge_list_header": False,
                    "edge_list_support_balanced_quotes": True,
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
        return "WikiData"

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
        Citations relative to the WikiData graphs
        """
        return [
            open(
                "{}/models/wikidata.bib".format(
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

