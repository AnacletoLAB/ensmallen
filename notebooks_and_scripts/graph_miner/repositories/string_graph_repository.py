"""Sub-module handling the retrieval and building of graphs from KGHUB."""
from typing import List, Dict
import os
import pandas as pd
import string
from .graph_repository import GraphRepository


class StringGraphRepository(GraphRepository):
    """Class representing the STRING repository."""

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the KGHub repository."""
        species_11_0 = pd.read_csv(
            "https://stringdb-downloads.org/download/species.v11.0.txt",
            sep='\t'
        )
        species_11_5 = pd.read_csv(
            "https://stringdb-downloads.org/download/species.v11.5.txt",
            sep='\t'
        )
        species_12_0 = pd.read_csv(
            "https://stringdb-downloads.org/download/species.v12.0.txt",
            sep='\t'
        )

        mined_data = {}

        type_of_graphs = [
            "homology",
            "physical.links",
            "links",
        ]

        parameters_per_type_of_graph = {
            "links": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "links.detailed": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "links.full": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "physical.links": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "physical.links.detailed": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "physical.links.full": {
                "sources_column": "protein1",
                "destinations_column": "protein2",
                "weights_column": "combined_score",
            },
            "homology": {
                "sources_column": "##protein1",
                "destinations_column": "protein2",
                "weights_column": "bitscore",
            },
            "cluster": {
                "sources_column": "source",
                "destinations_column": "destination",
            }
        }

        edge_list_url_pattern = "https://stringdb-downloads.org/download/protein.{type_of_graph}.v{version}/{taxon_id}.protein.{type_of_graph}.v{version}.txt.gz"
        enrichment_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.enrichment.terms.v{version}/{taxon_id}.protein.enrichment.terms.v{version}.txt.gz"
        info_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.info.v{version}/{taxon_id}.protein.info.v{version}.txt.gz"
        sequence_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.sequences.v{version}/{taxon_id}.protein.sequences.v{version}.fa.gz"
        tree_url_pattern = "https://stringdb-downloads.org/download/species.tree.v{version}.txt"
        tree_metadata_url_pattern = "https://stringdb-downloads.org/download/species.v{version}.txt"
        cluster_tree_url_pattern = "https://stringdb-downloads.org/download/clusters.tree.v{version}/{taxon_id}.clusters.tree.v{version}.txt.gz"
        cluster_info_url_pattern = "https://stringdb-downloads.org/download/clusters.info.v{version}/{taxon_id}.clusters.info.v{version}.txt.gz"
        cluster_to_protein_url_pattern = "https://stringdb-downloads.org/download/clusters.proteins.v{version}/{taxon_id}.clusters.proteins.v{version}.txt.gz"

        complete_graph_edgelist_url_pattern = "https://stringdb-downloads.org/download/protein.{edge_list_version}.v{version}.txt.gz"
        complete_graph_enrichment_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.enrichment.terms.v{version}.txt.gz"
        complete_graph_info_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.info.v{version}.txt.gz"
        complete_graph_sequence_node_list_url_pattern = "https://stringdb-downloads.org/download/protein.sequences.v{version}.fa.gz"

        graph_name = "SpeciesTree"
        stored_graph_name = self.build_stored_graph_name(graph_name)
        mined_data[stored_graph_name] = {}

        for version in ("11.0", "11.5", "12.0"):
            full_version_code = f"species.tree.v{version}"

            mined_data[stored_graph_name][full_version_code] = {
                "urls": [
                    tree_url_pattern.format(
                        version=version,
                    ),
                    tree_metadata_url_pattern.format(
                        version=version,
                    ),
                ],
                "paths": [
                    "species.tree.tsv",
                    "species.tsv"
                ],
                "callback": "create_species_tree_node_and_edge_list",
                "callback_arguments": {
                    "tree_path": "species.tree.tsv",
                    "tree_metadata_path": "species.tsv",
                    "node_list_path": "nodes.tsv",
                    "edge_list_path": "edges.tsv"
                },
                "arguments": {
                    "edge_path": "edges.tsv",
                    "node_path": "nodes.tsv",
                    "name": graph_name,
                    "nodes_column": "taxon_name",
                    "node_list_node_types_column": "domain",
                    "node_list_is_correct": True,
                    "edge_list_is_correct": True,
                    "edge_list_edge_types_column": "domain",
                    "sources_column": "sources",
                    "destinations_column": "destinations",
                }
            }

        graph_name = "CompleteString"
        stored_graph_name = self.build_stored_graph_name(graph_name)
        mined_data[stored_graph_name] = {}

        for version in ("11.0", "11.5", "12.0"):
            complete_graph_enrichment_node_list_url = complete_graph_enrichment_node_list_url_pattern.format(
                version=version,
            )
            complete_graph_info_node_list_url = complete_graph_info_node_list_url_pattern.format(
                version=version,
            )
            complete_graph_sequence_node_list_url = complete_graph_sequence_node_list_url_pattern.format(
                version=version,
            )

            node_urls = [
                complete_graph_enrichment_node_list_url,
                complete_graph_info_node_list_url,
                complete_graph_sequence_node_list_url
            ]

            node_paths = [
                "enrichment.terms.tsv.gz",
                "info.tsv.gz",
                "sequence.fa.gz"
            ]

            for edge_list_version in (
                "links",
                "links.detailed",
                "links.full",
                "physical.links",
                "physical.links.detailed",
                "physical.links.full",
            ):
                complete_graph_edgelist_url = complete_graph_edgelist_url_pattern.format(
                    edge_list_version=edge_list_version,
                    version=version,
                )
                full_version_code = f"{edge_list_version}.v{version}"

                mined_data[stored_graph_name][full_version_code] = {
                    "urls": [
                        complete_graph_edgelist_url,
                        *node_urls
                    ],
                    "paths": [
                        "edges.tsv.gz",
                        *node_paths
                    ],
                    "callback": "build_string_graph_node_list",
                    "callback_arguments": {
                        "sequence_path": "sequence.fa",
                        "enrichment_path": "enrichment.terms.tsv",
                        "info_path": "info.tsv",
                        "node_list_path": "nodes.tsv",
                    },
                    "arguments": {
                        "edge_path": "edges.tsv",
                        "node_path": "nodes.tsv",
                        "name": graph_name,
                        "nodes_column_number": 0,
                        "node_types_separator": "|",
                        "node_list_node_types_column": "term",
                        "node_list_is_correct": True,
                        "edge_list_is_correct": True,
                        **parameters_per_type_of_graph[edge_list_version]
                    }
                }

        for type_of_graph in type_of_graphs:
            for version, species in (
                ("11.0", species_11_0),
                ("11.5", species_11_5),
                ("12.0", species_12_0)
            ):
                for _, row in species.iterrows():
                    graph_name = row.STRING_name_compact
                    stored_graph_name = self.build_stored_graph_name(
                        graph_name
                    )
                    if stored_graph_name not in mined_data:
                        mined_data[stored_graph_name] = {}
                    taxon_id = row[0]
                    edge_list_url = edge_list_url_pattern.format(
                        taxon_id=taxon_id,
                        version=version,
                        type_of_graph=type_of_graph
                    )
                    full_version_code = f"{type_of_graph}.v{version}"

                    node_urls = [
                        enrichment_node_list_url_pattern.format(
                            taxon_id=taxon_id,
                            version=version,
                            type_of_graph=type_of_graph
                        ),
                        info_node_list_url_pattern.format(
                            taxon_id=taxon_id,
                            version=version,
                            type_of_graph=type_of_graph
                        ),
                        sequence_node_list_url_pattern.format(
                            taxon_id=taxon_id,
                            version=version,
                            type_of_graph=type_of_graph
                        )
                    ]

                    node_paths = [
                        "enrichment.terms.tsv.gz",
                        "info.tsv.gz",
                        "sequence.fa.gz"
                    ]

                    mined_data[stored_graph_name][full_version_code] = {
                        "urls": [
                            edge_list_url,
                            *node_urls
                        ],
                        "paths": [
                            "edges.tsv.gz",
                            *node_paths
                        ],
                        "callback": "build_string_graph_node_list",
                        "callback_arguments": {
                            "sequence_path": "sequence.fa",
                            "enrichment_path": "enrichment.terms.tsv",
                            "info_path": "info.tsv",
                            "node_list_path": "nodes.tsv",
                        },
                        "arguments": {
                            "edge_path": "edges.tsv",
                            "node_path": "nodes.tsv",
                            "name": graph_name,
                            "nodes_column_number": 0,
                            "node_types_separator": "|",
                            "node_list_node_types_column": "term",
                            "node_list_is_correct": True,
                            "edge_list_is_correct": True,
                            **parameters_per_type_of_graph[type_of_graph]
                        }
                    }

                    ###############################################
                    # Mining metadata for Cluster STRING graphs   #
                    ###############################################

                    graph_name = f"{row.STRING_name_compact} Cluster"
                    stored_graph_name = self.build_stored_graph_name(
                        graph_name
                    )
                    if stored_graph_name not in mined_data:
                        mined_data[stored_graph_name] = {}

                    full_version_code = f"cluster.v{version}"

                    mined_data[stored_graph_name][full_version_code] = {
                        "urls": [
                            cluster_tree_url_pattern.format(
                                taxon_id=taxon_id,
                                version=version,
                            ),
                            cluster_info_url_pattern.format(
                                taxon_id=taxon_id,
                                version=version,
                            ),
                            cluster_to_protein_url_pattern.format(
                                taxon_id=taxon_id,
                                version=version,
                            ),
                            *node_urls
                        ],
                        "paths": [
                            "clusters.tree.tsv.gz",
                            "clusters.info.tsv.gz",
                            "clusters.proteins.tsv.gz",
                            *node_paths
                        ],
                        "callback": "build_string_cluster_graph_node_and_edge_list",
                        "callback_arguments": {
                            "cluster_info_path": "clusters.info.tsv",
                            "cluster_tree_path": "clusters.tree.tsv",
                            "cluster_to_proteins_path": "clusters.proteins.tsv",
                            "sequence_path": "sequence.fa",
                            "enrichment_path": "enrichment.terms.tsv",
                            "info_path": "info.tsv",
                            "node_list_path": "nodes.tsv",
                            "edge_list_path": "edges.tsv",
                        },
                        "arguments": {
                            "edge_path": "edges.tsv",
                            "node_path": "nodes.tsv",
                            "name": graph_name,
                            "nodes_column": "node_name",
                            "node_types_separator": "|",
                            "node_list_node_types_column": "term",
                            "node_list_is_correct": True,
                            "edge_list_is_correct": True,
                            **parameters_per_type_of_graph["cluster"]
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
        if " " in partial_graph_name:
            return "".join([
                term.capitalize()
                for term in partial_graph_name.split(" ")
            ])
        return partial_graph_name

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
        return [self._data[self.build_stored_graph_name(graph_name)][version]["callback"]]

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
        return [self._data[self.build_stored_graph_name(graph_name)][version]["callback_arguments"]]

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
        path = os.path.dirname(os.path.abspath(__file__))
        return [
            open(
                f"{path}/models/string_citation.bib",
                "r",
                encoding="utf8"
            ).read()
        ]

    def get_graph_paths(self, graph_name: str, version: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
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

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())

    def build_all(self):
        """Build graph retrieval methods."""
        super().build_all()
        target_directory_path = os.path.join(
            "../bindings/python/ensmallen/datasets",
            self.repository_package_name,
        )
        file_path = f"{target_directory_path}.py"
        with open(file_path, "a", encoding="utf8") as f:
            with open("graph_miner/repositories/models/parse_string.py", "r", encoding="utf8") as original:
                f.write(original.read())
