"""Submodule for a pre-parametrized graph TFIDF weighted textual embedding."""
from typing import Optional, Dict, List
import numpy as np
import os
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph
from .get_okapi_tfidf_weighted_textual_embedding import get_okapi_tfidf_weighted_textual_embedding


def get_graph_okapi_tfidf_weighted_textual_embedding(
    graph_name: str,
    version: str,
    repository: str,
    columns: Optional[List[str]] = None,
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    read_csv_kwargs: Optional[Dict] = None,
    bert_tokenizer_kwargs: Optional[Dict] = None,
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.
    
    Parameters
    ------------------------
    graph_name: str
        The name of the graph to be retrieved and loaded.
    version: str
        The version of the graph to be retrieved.
    repository: str
        Name of the repository to load data from.
    columns: Optional[List[str]] = None
        The columns to be taken into consideration for the tokenization
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        Name of the pretrained model to be used.
    read_csv_kwargs: Optional[Dict] = None
        Kwargs to be used when opening a CSV to be read
    bert_tokenizer_kwargs: Optional[Dict] = None
        Kwargs to be used when retrieving the tokenizer
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.
    verbose: bool = True
        Whether to show the loading bars
    """
    # Sanitize data using the automatic graph retrieval
    graph_retriever = AutomaticallyRetrievedGraph(
        graph_name=graph_name,
        version=version,
        repository=repository,
        verbose=verbose,
    )
    # Get the path to the file to read.
    node_path = graph_retriever.get_adjusted_graph_nodes_path()
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
        k1=k1,
        b=b,
        columns=columns,
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        read_csv_kwargs=read_csv_kwargs,
        bert_tokenizer_kwargs=bert_tokenizer_kwargs,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )
