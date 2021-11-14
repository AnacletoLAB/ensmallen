"""Submodule for a pre-parametrized graph TFIDF weighted textual embedding."""
from typing import Optional, Dict, List
import numpy as np
import os
from .automatic_graph_retrieval import AutomaticallyRetrievedGraph
from .get_graph_okapi_tfidf_weighted_textual_embedding import get_graph_okapi_tfidf_weighted_textual_embedding


def get_kgx_okapi_tfidf_weighted_textual_embedding(
    graph_name: str,
    repository: str,
    version: str = "current",
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    bert_tokenizer_kwargs: Optional[Dict] = None,
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.
    
    Parameters
    ------------------------
    graph_name: str
        The name of the graph to be retrieved and loaded.
    repository: str,
        The kgx repository to be used.
    version: str = "current"
        The version of the graph to be retrieved.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        Name of the pretrained model to be used.
    bert_tokenizer_kwargs: Optional[Dict] = None
        Kwargs to be used when retrieving the tokenizer
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.
    verbose: bool = True
        Whether to show the loading bars
    """
    return get_graph_okapi_tfidf_weighted_textual_embedding(
        graph_name=graph_name,
        version=version,
        repository=repository,
        k1=k1,
        b=b,
        columns=["id", "category", "name", "description", "synonym"],
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        read_csv_kwargs=dict(
            sep=r"\t",
            encoding="utf8",
            engine="python"
        ),
        bert_tokenizer_kwargs=bert_tokenizer_kwargs,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )

def get_kghub_okapi_tfidf_weighted_textual_embedding(
    graph_name: str,
    version: str = "current",
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    bert_tokenizer_kwargs: Optional[Dict] = None,
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.
    
    Parameters
    ------------------------
    graph_name: str
        The name of the graph to be retrieved and loaded.
    version: str = "current"
        The version of the graph to be retrieved.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        Name of the pretrained model to be used.
    bert_tokenizer_kwargs: Optional[Dict] = None
        Kwargs to be used when retrieving the tokenizer
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.
    verbose: bool = True
        Whether to show the loading bars
    """
    return get_kgx_okapi_tfidf_weighted_textual_embedding(
        graph_name=graph_name,
        version=version,
        repository="kghub",
        k1=k1,
        b=b,
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_tokenizer_kwargs=bert_tokenizer_kwargs,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )

def get_kgobo_okapi_tfidf_weighted_textual_embedding(
    graph_name: str,
    version: str = "current",
    k1: float = 1.5,
    b: float = 0.75,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    bert_tokenizer_kwargs: Optional[Dict] = None,
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Return OKAPI TFIDF-weighted textual embedding of the data available for the selected graph.
    
    Parameters
    ------------------------
    graph_name: str
        The name of the graph to be retrieved and loaded.
    version: str = "current"
        The version of the graph to be retrieved.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        Name of the pretrained model to be used.
    bert_tokenizer_kwargs: Optional[Dict] = None
        Kwargs to be used when retrieving the tokenizer
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.
    verbose: bool = True
        Whether to show the loading bars
    """
    return get_kgx_okapi_tfidf_weighted_textual_embedding(
        graph_name=graph_name,
        version=version,
        repository="kgobo",
        k1=k1,
        b=b,
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_tokenizer_kwargs=bert_tokenizer_kwargs,
        bert_model_kwargs=bert_model_kwargs,
        verbose=verbose
    )