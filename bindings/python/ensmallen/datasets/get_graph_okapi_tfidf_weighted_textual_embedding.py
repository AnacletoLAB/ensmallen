"""Submodule for a pre-parametrized graph TFIDF weighted textual embedding."""
from typing import Optional, Dict, List
import numpy as np
import os
from .graph_retrieval import RetrievedGraph
from .get_okapi_tfidf_weighted_textual_embedding import get_okapi_tfidf_weighted_textual_embedding


def get_graph_okapi_tfidf_weighted_textual_embedding(
    name: str,
    version: Optional[str] = None,
    repository: Optional[str] = None,
    columns: Optional[List[str]] = None,
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "bert-base-uncased",
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.

    Parameters
    ------------------------
    name: str
        The name of the graph to be retrieved and loaded.
    version: Optional[str] = None
        The version of the graph to be retrieved.
    repository: Optional[str] = None
        Name of the repository to load data from.
    columns: Optional[List[str]] = None
        The columns to be taken into consideration for the tokenization
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    pretrained_model_name_or_path: str = "bert-base-uncased"
        Name of the pretrained model to be used.
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.
    verbose: bool = True
        Whether to show the loading bars
    """
    # Sanitize data using the automatic graph retrieval
    graph_retriever = RetrievedGraph(
        name=name,
        version=version,
        repository=repository,
        verbose=verbose,
    )
    # Get the path to the file to read.
    node_path = graph_retriever.get_adjusted_graph_nodes_path()
    arguments = graph_retriever.get_graph_arguments()
    if node_path is None:
        # If the current graph does not have a node list, it does
        # not make sense to require for a textual embedding of the node list.
        raise ValueError(
            "The requested graph does not come with "
            "a node list, making it impossible to compute "
            "the textual embedding of such a node list textual data"
        )
    # We retrieve the node data
    # This is automally cached if previously dowloaded
    graph_retriever.download()
    # We compute the word embedding
    return get_okapi_tfidf_weighted_textual_embedding(
        path=node_path,
        separator=arguments.get(
            "node_list_separator"
        ),
        header=arguments.get(
            "node_list_header"
        ),
        k1=k1,
        b=b,
        columns=columns,
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )
