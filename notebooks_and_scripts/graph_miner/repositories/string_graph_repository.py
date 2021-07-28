"""Sub-module handling the retrieval and building of graphs from STRING."""
from typing import List, Dict
import os
import pandas as pd
from .graph_repository import GraphRepository


class StringGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._base_url = "https://stringdb-static.org/download/protein.links.v11.0/{}.protein.links.v11.0.txt.gz"
        self._organisms = pd.read_csv(
            "https://stringdb-static.org/download/species.v11.0.txt",
            sep="\t"
        )

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
        return "".join([
            term.capitalize()
            for term in partial_graph_name.replace(".", " ").split(" ")
        ])

    def get_formatted_repository_name(self) -> str:
        """Return formatted repository name."""
        return "STRING"

    def get_graph_name(self, graph_data) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        graph_data: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        return graph_data.STRING_name_compact

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
        return [self._base_url.format(graph_data['## taxon_id'])]

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
        """
        return [os.path.join(
            self.repository_package_name,
            "{}.csv.gz".format(
                graph_name.lower().replace(" ", "_")
            )
        )]

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
            "sources_column": "protein1",
            "destinations_column": "protein2",
            "weights_column": "combined_score",
        }

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return [
            row
            for _, row in self._organisms.iterrows()
        ]

    def get_node_path(
        self,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the node files.

        Parameters
        -----------------------
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the node files.
        """
        return None

    def get_edge_path(
        self,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the edge files.

        Parameters
        -----------------------
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the edge files.
        """
        return download_report.extraction_destination[0]
