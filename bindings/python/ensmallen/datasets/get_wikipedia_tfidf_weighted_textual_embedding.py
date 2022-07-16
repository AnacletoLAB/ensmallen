"""Submodule for a pre-parametrized Wikipedia TFIDF weighted textual embedding."""
from typing import Optional, Dict
import numpy as np
import os
from .wikipedia_graph_retrieval import WikipediaRetrievedGraph
from .get_okapi_tfidf_weighted_textual_embedding import get_okapi_tfidf_weighted_textual_embedding


def get_wikipedia_okapi_tfidf_weighted_textual_embedding(
    name: str,
    version: str = "latest",
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
    version: str
        The version of the graph to be retrieved.
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
    graph_retriever = WikipediaRetrievedGraph(
        name=name,
        version=version,
        verbose=verbose
    )
    # Get the path to the file to read.
    node_path = graph_retriever.get_preprocessed_graph_nodes_path()
    # If the file does not exist, it means that the graph was
    # not retrieved yet and we need to retrieve it.
    if not os.path.exists(node_path):
        _ = graph_retriever()
    # We compute the word embedding
    return get_okapi_tfidf_weighted_textual_embedding(
        path=node_path,
        separator="\t",
        header=True,
        k1=k1,
        b=b,
        columns=["node_names", "node_descriptions"],
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )
