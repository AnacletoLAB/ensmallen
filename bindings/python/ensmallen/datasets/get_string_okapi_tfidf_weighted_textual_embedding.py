"""Submodule for a pre-parametrized graph TFIDF weighted textual embedding."""
from typing import Optional, Dict, List
import numpy as np
import os
from .get_graph_okapi_tfidf_weighted_textual_embedding import get_graph_okapi_tfidf_weighted_textual_embedding


def get_string_okapi_tfidf_weighted_textual_embedding(
    name: str,
    version: str = "links.v11.5",
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "bert-base-uncased",
    columns_to_use: List[str] = ("preferred_name", "annotation", "description"),
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.
    
    Parameters
    ------------------------
    name: str
        The name of the graph to be retrieved and loaded.
    version: str = "links.v11.5"
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
    return get_graph_okapi_tfidf_weighted_textual_embedding(
        name=name,
        version=version,
        repository="string",
        k1=k1,
        b=b,
        columns=list(columns_to_use),
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )