"""Submodule with methods to parse and standardize STRING graphs."""
import pandas as pd


def create_species_tree_node_and_edge_list(
    tree_path: str, tree_metadata_path: str, node_list_path: str, edge_list_path: str
):
    """Create the node and edge lists for the species tree at given path.

    Parameters
    -------------------
    tree_path: str
        The path from where to load the tree data.
    tree_metadata_path: str
        The path from where to load the tree metadata.
    node_list_path: str
        The path where to store the tree node list.
    edge_list_path: str
        The path where to store the tree edge list.
    """
    tree = pd.read_csv(tree_path, sep="\t", index_col=0)
    metadata = pd.read_csv(tree_metadata_path, sep="\t", index_col=0)

    # Imputing the missing domains in the Species Tree.
    tree.loc[metadata.index, "domain"] = metadata.domain
    tree.loc[tree.parent_taxon_id == 1, "domain"] = "Ancestral"
    tree.loc[tree.domain == "Eukaryotes", "domain"] = "Eukaryota"

    while tree.domain.isna().any():
        child_nodes = tree[
            tree.loc[tree.parent_taxon_id].domain.isna().values
            & tree.domain.notna().values
        ]
        tree.loc[child_nodes.parent_taxon_id, "domain"] = child_nodes.domain.values

    # We drop the temporary node index 1
    tree.drop(index=1, inplace=True)

    # Making taxon names unique, so that corner cases
    # such as `Drosophila Drosophila` can be handled.
    tree.loc[tree.duplicated("taxon_name"), "taxon_name"] = [
        f"{taxon_name}.{index}"
        for index, taxon_name in tree.loc[
            tree.duplicated("taxon_name"), "taxon_name"
        ].iteritems()
    ]

    # Writing the node list
    node_list = tree[["taxon_name", "domain"]]
    node_list.to_csv(node_list_path, sep="\t", index=False)

    # We drop edges from the root node to LUCA
    tree.drop(index=tree[tree.parent_taxon_id == 1].index[0], inplace=True)

    # Writing the edge list
    pd.DataFrame(
        {
            "sources": node_list.loc[tree.parent_taxon_id].taxon_name.values,
            "destinations": node_list.loc[tree.index].taxon_name.values,
            "domain": tree.domain.values,
        }
    ).to_csv(edge_list_path, sep="\t", index=False)


def build_string_cluster_graph_node_and_edge_list(
    cluster_info_path: str,
    cluster_tree_path: str,
    cluster_to_proteins_path: str,
    node_list_path: str,
    edge_list_path: str,
):
    """Build labeled edge and node list for cluster graphs.

    Parameters
    -----------------------
    cluster_info_path: str
        Path from where to load the cluster node list informations.
    cluster_tree_path: str
        Path from where to load the cluster to cluster edge list.
    cluster_to_proteins_path: str
        Path from where to load the cluster to protein edge list.
    target_path: str
        Path where to write the resulting node list TSV.
    """
    cluster_only_node_list = pd.read_csv(cluster_info_path, sep="\t")
    cluster_only_node_list["term"] = "Cluster"
    cluster_only_node_list.rename(
        columns={"best_described_by": "description", "cluster_id": "node_name"},
        inplace=True,
    )
    string_only_node_list = pd.read_csv(node_list_path, sep="\t")
    string_only_node_list.rename(
        columns={"#string_protein_id": "node_name"}, inplace=True
    )
    node_list = pd.concat((cluster_only_node_list, string_only_node_list), axis=0)
    node_list.to_csv(node_list_path, sep="\t")

    cluster_tree_df = pd.read_csv(cluster_tree_path, sep="\t")
    cluster_to_proteins_df = pd.read_csv(cluster_to_proteins_path, sep="\t")

    cluster_tree_df.rename(
        columns={
            "child_cluster_id": "source",
            "parent_cluster_id": "destination",
        },
        inplace=True,
    )

    cluster_to_proteins_df.rename(
        columns={
            "cluster_id": "source",
            "protein_id": "destination",
        },
        inplace=True,
    )

    edge_list = pd.concat(
        (
            cluster_tree_df[["source", "destination"]],
            cluster_to_proteins_df[["source", "destination"]],
        ),
        axis=0,
    )

    edge_list.to_csv(edge_list_path, sep="\t", index=False)
