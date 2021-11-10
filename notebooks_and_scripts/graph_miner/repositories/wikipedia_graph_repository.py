"""Sub-module handling the retrieval and building of graphs from Wikipedia."""
from typing import List, Dict
import requests
from tqdm.auto import tqdm
from bs4 import BeautifulSoup
from .graph_repository import GraphRepository
from ..utils import get_cached_page


def normalize_wikipedia_graph_name(graph_name: str) -> str:
    """Return given wikipedia graph name normalized.

    Parameters
    ------------------------
    graph_name: str
        Graph name to be normalized
    """
    well_formed = {
        "wiki": "Wiki",
        "wikimedia": "WikiMedia",
        "wikisource": "WikiSource",
        "wikiquote": "WikiQuote",
        "wikivoyage": "WikiVoyage",
        "wikibooks": "WikiBooks",
        "wikinews": "WikiNews",
        "wikiversity": "Wikiversity",
    }
    return "{}{}".format(
        well_formed[graph_name[2:]],
        graph_name[:2].upper(),
    )


class WikipediaGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the Wikipedia repository."""
        main_url = "https://dumps.wikimedia.org/backup-index.html"
        root_url_pattern = "https://dumps.wikimedia.org/{main_root}/"
        data_url_pattern = "https://dumps.wikimedia.org/{main_root}/{version}/{main_root}-{version}-pages-articles-multistream.xml.bz2"
        compressed_edge_path_pattern = "{main_root}-{version}-pages-articles-multistream.xml.bz2"
        versions_black_list = ["../"]

        soup = BeautifulSoup(get_cached_page(main_url), "lxml")
        main_roots = [
            anchor["href"].split("/")[0]
            for anchor in soup.find_all("a")
            if anchor["href"][2:6] == "wiki"
        ]

        mined_data = {}
        kwargs = {
            "edge_path": "edge_list.tsv",
            "node_path": "node_list.tsv",
            "node_type_path": "node_type_list.tsv",
            "node_types_separator": "|",
            "nodes_column": "id",
            "node_list_node_types_column": "category",
            "sources_column_number": 0,
            "destinations_column_number": 1,
        }

        for main_root in tqdm(
            main_roots,
            desc="Parsing Wikipedia graphs",
            leave=False
        ):
            soup = BeautifulSoup(get_cached_page(root_url_pattern.format(
                main_root=main_root
            )), "lxml")
            graph_name = normalize_wikipedia_graph_name(main_root)
            versions = [
                anchor["href"][:-1]
                for anchor in soup.find_all("a")
                if anchor["href"] not in versions_black_list
            ]
            mined_data[graph_name] = {
                version: {
                    "urls": [
                        data_url_pattern.format(
                            main_root=main_root,
                            version=version
                        )
                    ],
                    "paths": [
                        compressed_edge_path_pattern.format(
                            main_root=main_root,
                            version=version
                        )
                    ],
                    "arguments": {
                        "edge_list_support_balanced_quotes": True,
                        "name": graph_name,
                    }
                }
                for version in versions
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
        return "Wikipedia"

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
        Citations relative to the Wikipedia graphs
        """
        return None

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

    def get_graph_retrieval_file(self) -> str:
        """Return graph retrieval file."""
        return "wikipedia_graph_retrieval_file"