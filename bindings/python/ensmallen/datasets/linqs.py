"""Module providing graphs available from LINQS."""
from ensmallen import Graph  # pylint: disable=import-error
from .graph_retrieval import RetrievedGraph

def PubMedDiabetes(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return PubMedDiabetes graph	The Pubmed Diabetes dataset consists of 19717 scientific publications from
	PubMed database pertaining to diabetes classified into one of three classes.
	The citation network consists of 44338 links. Each publication in the dataset
	is described by a TF/IDF weighted word vector from a dictionary which consists
	of 500 unique words.

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "latest"
        Version to retrieve		
	
	References
	----------
	Please cite:
	
	```bib
	@inproceedings{namata2012query,
	  title={Query-driven active surveying for collective classification},
	  author={Namata, Galileo and London, Ben and Getoor, Lise and Huang, Bert and EDU, UMD},
	  booktitle={10th International Workshop on Mining and Learning with Graphs},
	  volume={8},
	  year={2012}
	}
	```
    """
    return RetrievedGraph(
        "PubMedDiabetes", version, "linqs", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs,
		callbacks=[
			parse_linqs_pubmed_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.DIRECTED.cites.tab",
		        "content_path": "Pubmed-Diabetes/Pubmed-Diabetes/data/Pubmed-Diabetes.NODE.paper.tab",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
def Cora(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return Cora graph	The Cora dataset consists of 2708 scientific publications classified into
	one of seven classes. The citation network consists of 5429 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 1433 unique words.

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "latest"
        Version to retrieve		
	
	References
	----------
	Please cite:
	
	```bib
	@incollection{getoor2005link,
	  title={Link-based classification},
	  author={Getoor, Lise},
	  booktitle={Advanced methods for knowledge discovery from complex data},
	  pages={189--207},
	  year={2005},
	  publisher={Springer}
	}
	
	@article{sen2008collective,
	  title={Collective classification in network data},
	  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
	  journal={AI magazine},
	  volume={29},
	  number={3},
	  pages={93--93},
	  year={2008}
	}
	```
    """
    return RetrievedGraph(
        "Cora", version, "linqs", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs,
		callbacks=[
			parse_linqs_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "cora/cora/cora.cites",
		        "content_path": "cora/cora/cora.content",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
def CiteSeer(
    directed=False, preprocess="auto", bioregistry=False, load_nodes=True, load_node_types=True,
    load_edge_types=True, load_edge_weights=True, auto_enable_tradeoffs=True,
    sort_tmp_dir=None, verbose=2, ring_bell=False, cache=True, cache_path=None,
    cache_sys_var="GRAPH_CACHE_DIR", version="latest", **kwargs
) -> Graph:
    """Return CiteSeer graph	The CiteSeer dataset consists of 3312 scientific publications classified
	into one of six classes. The citation network consists of 4732 links. Each
	publication in the dataset is described by a 0/1-valued word vector indicating
	the absence/presence of the corresponding word from the dictionary. The
	dictionary consists of 3703 unique words.

    Parameters
    ----------
    directed = False
    preprocess = "auto"
        Preprocess for optimal load time & memory peak.
        Will preprocess in Linux/macOS but not Windows.
    bioregistry=False
    load_nodes = True
        Load node names or use numeric range
    load_node_types = True
    load_edge_types = True
    auto_enable_tradeoffs = True
        Enable when graph has < 50M edges
    cache_path = None
        Path to store graphs
        Defaults either to `GRAPH_CACHE_DIR` sys var or `graphs`
    cache_sys_var = "GRAPH_CACHE_DIR"
    version = "latest"
        Version to retrieve		
	
	References
	----------
	Please cite:
	
	```bib
	@incollection{getoor2005link,
	  title={Link-based classification},
	  author={Getoor, Lise},
	  booktitle={Advanced methods for knowledge discovery from complex data},
	  pages={189--207},
	  year={2005},
	  publisher={Springer}
	}
	
	@article{sen2008collective,
	  title={Collective classification in network data},
	  author={Sen, Prithviraj and Namata, Galileo and Bilgic, Mustafa and Getoor, Lise and Galligher, Brian and Eliassi-Rad, Tina},
	  journal={AI magazine},
	  volume={29},
	  number={3},
	  pages={93--93},
	  year={2008}
	}
	```
    """
    return RetrievedGraph(
        "CiteSeer", version, "linqs", directed, preprocess, bioregistry, load_nodes,
        load_node_types, load_edge_types, load_edge_weights, auto_enable_tradeoffs, sort_tmp_dir,
        verbose, ring_bell, cache, cache_path, cache_sys_var, kwargs,
		callbacks=[
			parse_linqs_incidence_matrix
		],
		callbacks_arguments=[
		    {
		        "cites_path": "citeseer/citeseer/citeseer.cites",
		        "content_path": "citeseer/citeseer/citeseer.content",
		        "node_path": "nodes.tsv",
		        "edge_path": "edges.tsv"
		    }
		]
    )()
import re
import os
import pandas as pd
import numpy as np
from typing import Tuple
from ensmallen import Graph
import warnings


def get_words_data(
    graph: Graph,
    remove_nodes_without_features: bool = True,
) -> Tuple[Graph, pd.DataFrame]:
    """Return dataframe with words features.

    Parameters
    --------------------
    graph: Graph
        Graph containing the words features to be extracted.
    remove_nodes_without_features: bool = True
        Whether to remove the nodes without known node features.

    Returns
    --------------------
    Tuple containing:
        - Provided graph without the Word nodes.
        - Pandas DataFrame with words features as columns and nodes as rows.
    """
    word_node_type = graph.get_node_type_id_from_node_type_name("Word")
    # Extracting node features
    node_features = pd.DataFrame({
        node_name: {
            graph.get_node_name_from_node_id(dst): graph.get_edge_weight_from_node_ids(src, dst) if graph.has_edge_weights() else 1.0
            for dst in graph.get_neighbour_node_ids_from_node_id(src)
        }
        for src, node_name in enumerate(graph.get_node_names())
        if word_node_type in graph.get_node_type_ids_from_node_id(src)
    }).fillna(0.0)
    
    # Filtering graph
    filtered_graph = graph.filter_from_names(
        node_type_name_to_remove=["Word"],
    ).remove_edge_weights().remove_edge_types()
    # Check if there are unavailable nodes.
    unavailable_nodes = list(set(filtered_graph.get_node_names()) - set(node_features.index))
    if len(unavailable_nodes) > 0:
        # If requested, compute the set of nodes to remove because
        # we do not have known features for these nodes in CiteSeer.
        if remove_nodes_without_features:
            warnings.warn(
                (
                    "Note that some nodes did not come with node features! "
                    "As requested, these nodes will be removed. "
                    "If you want to change this behaviour, set the `remove_nodes_without_features` parameter to false.\n"
                    "Specifically, the names of the nodes without features are:\n"
                    "\t{}"
                ).format(
                    "\n\t".join(unavailable_nodes)
                )
            )
            # Aligning node features with filtered graph node names.
            filtered_graph = filtered_graph.filter_from_names(
                node_names_to_remove=unavailable_nodes
            )
            node_features = node_features.loc[filtered_graph.get_node_names()]
        else:
            warnings.warn(
                (
                    "Note that some nodes did not come with node features! "
                    "Specifically, the names of the nodes without features are:\n"
                    "\t{}"
                ).format(
                    "\n\t - ".join(unavailable_nodes)
                )
            )

    # Returning elaborared graph and node features.
    return (filtered_graph, node_features)


def parse_linqs_pubmed_incidence_matrix(
    cites_path: str,
    content_path: str,
    edge_path: str,
    node_path: str
):
    """Parse PubMed incidence matrix and generates proper edge list and node file.

    Parameters
    -------------------
    cites_path: str,
        Path from where to load the cites file.
    content_path: str,
        Path from where to load the content file.
    edge_path: str,
        Path where to store the edge list.
    node_path: str,
        Path where to store the node list.
    """
    # If the file has already been created, we skip
    # this preprocessing.
    if os.path.exists(node_path) and os.path.exists(edge_path):
        return
    # Creating directories
    os.makedirs(os.path.dirname(edge_path), exist_ok=True)
    os.makedirs(os.path.dirname(node_path), exist_ok=True)
    # Loading data
    with open(content_path) as f:
        content = f.read()
    with open(cites_path) as f:
        cites = f.read()

    separator = '\t'

    edge_list_file = open(edge_path, "w")
    node_list_file = open(node_path, "w")

    unique_words = set()
    edge_regex = re.compile(r"paper:(\d+)")
    node_regex = re.compile(r"(\d+)\s+label=(\d+)")
    word_regex = re.compile(r"w-(\w+)=(\S+)")

    labels = [
        "Diabetes Mellitus, Experimental",
        "Diabetes Mellitus Type 1",
        "Diabetes Mellitus Type 2"
    ]

    edge_list_file.write(
        separator.join(("subject", "object", "edge_type", "weight")) + "\n"
    )
    node_list_file.write(separator.join(("id", "node_type")) + "\n")

    for line in cites.split("\n")[2:-1]:
        edge = re.findall(edge_regex, line)
        if len(edge) != 2:
            continue
        # Writing the edges between papers and papers
        edge_list_file.write(separator.join((*edge, "Paper2Paper", "")) + "\n")

    for line in content.split("\n")[2:]:
        vals = re.findall(node_regex, line)
        if len(vals) != 1:
            break

        src, label = vals[0]
        # Writing node and its node type to the node list.
        node_list_file.write(separator.join(
            (src, labels[int(label)-1])) + "\n")

        # Writing the edges between papers and words
        for (word, weight) in re.findall(word_regex, line):
            edge_list_file.write(
                separator.join((src, word, "Paper2Word", weight)) + "\n")
            # Add word to the unique words set
            unique_words.add(word)

    # Writing the nodes representing words
    for word in unique_words:
        node_list_file.write(separator.join((word, "Word")) + "\n")

    edge_list_file.close()
    node_list_file.close()


def parse_linqs_incidence_matrix(
    cites_path: str,
    content_path: str,
    edge_path: str,
    node_path: str
):
    """Parse Cora and Citeseer incidence matrix and generates proper edge list and node file.

    Parameters
    -------------------
    cites_path: str,
        Path from where to load the cites file.
    content_path: str,
        Path from where to load the content file.
    edge_path: str,
        Path where to store the edge list.
    node_path: str,
        Path where to store the node list.
    """
    # If the file has already been created, we skip
    # this preprocessing.
    if os.path.exists(node_path) and os.path.exists(edge_path):
        return
    # Creating directories
    os.makedirs(os.path.dirname(edge_path), exist_ok=True)
    os.makedirs(os.path.dirname(node_path), exist_ok=True)
    # Loading the content file (incidence matrix)
    content = pd.read_csv(
        content_path,
        sep='\t',
        header=None,
        index_col=0,
        dtype=str
    )
    # Loading the citations file (edge list)
    cities = pd.read_csv(
        cites_path,
        sep='\t',
        header=None,
        dtype=str
    )
    # Standardizing dataframe
    cities.columns = ["subject", "object"]
    cities["edge_type"] = "Paper2Paper"
    # Extract labels from incidence matrix
    labels = content[content.columns[-1]]
    # Removing labels column
    content.drop(columns=content.columns[-1], inplace=True)
    # Converting incidence matrix to edge list
    node_indices, word_indices = np.where(content.values.astype(int) == 1)
    # Create words vector
    words = np.array([
        "word_{}".format(word_id)
        for word_id in np.arange(max(word_indices)+1)
    ])
    # Create the node list
    node_list = pd.concat([
        pd.DataFrame({
            "id": content.index.astype(str),
            "node_type": labels
        }),
        pd.DataFrame({
            "id": words,
            "node_type": "Word"
        }),
        pd.DataFrame({
            "id": list(
                set(cities[["subject", "object"]].values.flatten()
                    ) - set(content.index.astype(str))
            ),
            "node_type": "Unknown"
        })
    ]).reset_index(drop=True)
    # Create the edge list
    edge_list = pd.concat([
        cities,
        pd.DataFrame({
            "subject": content.index[node_indices].astype(str),
            "object": words[word_indices],
            "edge_type": "Paper2Word"
        })
    ]).reset_index(drop=True)
    # Storing the generated node list
    node_list.to_csv(node_path, sep='\t', index=False)
    # Storing the generated edge list
    edge_list.to_csv(edge_path, sep='\t', index=False)