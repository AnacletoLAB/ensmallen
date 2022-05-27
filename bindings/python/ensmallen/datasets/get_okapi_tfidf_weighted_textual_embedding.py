"""Sub-module providing TFIDF weighted textual embedding of common embedding."""
from typing import Dict, Optional

import numpy as np
import pandas as pd
from cache_decorator import Cache

from ensmallen import preprocessing


@Cache(
    cache_path="{cache_dir}/{pretrained_model_name_or_path}.npy",
    cache_dir="precomputed_word_embedding"
)
def get_precomputed_word_embedding(
    pretrained_model_name_or_path: str = "bert-base-uncased",
    bert_model_kwargs: Optional[Dict] = None
) -> np.ndarray:
    """Returns numpy array with word embedding from the provided embedding.

    Parameters
    --------------------------------
    pretrained_model_name_or_path: str = "bert-base-uncased"
        The model to be used.
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.

    Implementative details
    --------------------------------
    The pretrained models available can be seen here: https://huggingface.co/models
    """
    from transformers import BertModel

    if bert_model_kwargs is None:
        bert_model_kwargs = {}
    # Retrieve the model.
    bert_model = BertModel.from_pretrained(
        pretrained_model_name_or_path,
        output_hidden_states=True,
        **bert_model_kwargs
    )
    # Build the model.
    # TODO! check if this step is necessary!
    bert_model.eval()
    # Return the numpy array with the word embedding.
    return bert_model.embeddings.word_embeddings.weight.data.numpy()


@Cache(
    cache_path="{cache_dir}/{_hash}.npy",
    cache_dir="okapi_tfidf_weighted_textual_embedding"
)
def get_okapi_tfidf_weighted_textual_embedding(
    path: str,
    separator: str = None,
    header: bool = None,
    k1: float = 1.5,
    b: float = 0.75,
    columns: Optional[str] = None,
    pretrained_model_name_or_path: str = "bert-base-uncased",
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Returns tokens od the textual data available at the given file.

    Parameters
    -------------------------
    path: str
        Path from where to load the CSV.
    separator: Optional[str] = None
        The separator for the CSV.
    header: Optional[bool] = None
        Whether to skip the header.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    columns: Optional[str] = None
        The columns to be taken into consideration for the tokenization
    pretrained_model_name_or_path: str = "bert-base-uncased"
        Name of the pretrained model to be used.
    read_csv_kwargs: Optional[Dict] = None
        Kwargs to be used when opening a CSV to be read
    verbose: bool = True
        Whether to show the loading bars
    """
    # Retrieving the tokenizer and building the necessary metadata
    from transformers import AutoTokenizer
    tokenizer_path = f"okapi_tfidf_weighted_textual_embedding/{pretrained_model_name_or_path}"
    AutoTokenizer.from_pretrained(
        pretrained_model_name_or_path
    ).save_pretrained(tokenizer_path)
        
    # Retrieve the requested pretrained word embedding.
    word_embedding = get_precomputed_word_embedding(
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_model_kwargs=bert_model_kwargs
    )
    
    # Compute the weighted embedding
    return preprocessing.get_okapi_tfidf_weighted_textual_embedding(
        path=path,
        embedding=word_embedding,
        tokenizer_path=f"{tokenizer_path}/tokenizer.json",
        k1=k1,
        b=b,
        columns=columns,
        separator=separator,
        header=header,
        verbose=verbose,
    )
