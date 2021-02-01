"""Sub-module handling the retrieval and building of graphs from STRING."""
from typing import List
import os
import pandas as pd
from .graph_repository import GraphRepository


class StringGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._base_url = "https://string-db.org/cgi/download?species_text={}"
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
            for term in partial_graph_name.split(" ")
        ])

    def get_graph_urls(self, graph_name: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return self._base_url.format(graph_name.replace(" ", "+"))

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
        raise os.path.join(
            self.name,
            "{}.csv.gz".format(
                graph_name.lower().replace(" ", "_")
            )
        )

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(set(self._organisms.STRING_name_compact))
