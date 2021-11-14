"""Sub-module providing TFIDF weighted textual embedding of common embedding."""
from multiprocessing import Pool, cpu_count
from typing import Dict, List, Optional

import numpy as np
import pandas as pd
from cache_decorator import Cache
from tqdm.auto import tqdm
from transformers import BertModel, BertTokenizer

from ..ensmallen import preprocessing


def _compute_tokens(df: pd.DataFrame, tokenizer: BertTokenizer, dtype: str) -> List[np.ndarray]:
    """Returns tokenized tokens from the provided dataframe using the provided tokenizer.

    Parameters
    --------------------------
    df: pd.DataFrame
        DataFrame to tokenize
    tokenizer: BertTokenizer
        Tokenizer to use
    """
    return [
        np.array(tokenizer.encode(" ".join((
            e
            for e in row
            if pd.notna(e)
        ))), dtype=dtype)
        for _, row in df.iterrows()
    ]


def _compute_tokens_wrapper(args):
    """Wrapper for the tokenization method."""
    return _compute_tokens(*args)


@Cache(
    cache_path="{cache_dir}/{pretrained_model_name_or_path}.npy",
    cache_dir="precomputed_word_embedding"
)
def get_precomputed_word_embedding(
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    bert_model_kwargs: Optional[Dict] = None
) -> np.ndarray:
    """Returns numpy array with word embedding from the provided embedding.

    Parameters
    --------------------------------
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        The model to be used.
    bert_model_kwargs: Optional[Dict] = None
        Arguments to be used to retrieve the model.

    Implementative details
    --------------------------------
    The pretrained models available can be seen here: https://huggingface.co/models
    """
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


def get_tfidf_scores(
    path: str,
    k1: float = 1.5,
    b: float = 0.75,
    columns: Optional[str] = None,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    read_csv_kwargs: Optional[Dict] = None,
    bert_tokenizer_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> List[Dict[int, float]]:
    """Returns tokens od the textual data available at the given file.

    Parameters
    -------------------------
    path: str
        Path from where to load the CSV.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    columns: Optional[str] = None
        The columns to be taken into consideration for the tokenization
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased"
        Name of the pretrained model to be used.
    read_csv_kwargs: Optional[Dict] = None
        Kwargs to be used when opening a CSV to be read
    bert_tokenizer_kwargs: Optional[Dict] = None
        Kwargs to be used when retrieving the tokenizer
    verbose: bool = True
        Whether to show the loading bars
    """
    # Initialize the kwargs if none where provided.
    if read_csv_kwargs is None:
        read_csv_kwargs = {}
    if bert_tokenizer_kwargs is None:
        bert_tokenizer_kwargs = {}

    # Get the CSV columns
    csv_columns = pd.read_csv(
        path,
        nrows=1,
        **read_csv_kwargs
    ).columns

    # Initialize the column to be used if non were provided
    if columns is None:
        columns = csv_columns

    # First we check that the provided columns are correct.
    for column in columns:
        if column not in csv_columns:
            raise ValueError((
                "Requested column {} is not available in the set of "
                "columns present in the provided set of columns: {}."
            ).format(column, csv_columns))

    # Create set of columns to be actually read by pandas.
    column_numbers = [
        i
        for i, column in enumerate(csv_columns)
        if column in columns
    ]

    # Create the requested tokenizer.
    tokenizer = BertTokenizer.from_pretrained(
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        **bert_tokenizer_kwargs
    )

    # Get the vocabulary size
    vocabulary_size = len(tokenizer.get_vocab())

    if vocabulary_size < 2**8:
        dtype = "uint8"
    elif vocabulary_size < 2**16:
        dtype = "uint16"
    elif vocabulary_size < 2**32:
        dtype = "uint16"
    else:
        dtype = "uint64"

    # Initialize the pool to compute the tokens in parallel.
    with Pool(cpu_count()) as p:
        tokens = list(tqdm(
            (
                token
                for tokens in p.imap(
                    _compute_tokens_wrapper,
                    (
                        (chunk[columns], tokenizer, dtype)
                        for chunk in pd.read_csv(
                            path,
                            iterator=True,
                            chunksize=1000,
                            use_cols=column_numbers,
                            dtype=str,
                            **read_csv_kwargs
                        )
                    )
                )
                for token in tokens
            ),
            desc="Tokenizing node list textual informations",
            disable=not verbose,
            leave=False,
            dynamic_ncols=True
        ))
        # Close the process pool.
        p.close()
        p.join()

    # Compute the TFIDF tokenization.
    return preprocessing.get_okapi_bm25_tfidf_from_documents_u32(
        documents=tokens,
        k1=k1,
        b=b,
        verbose=verbose
    )


@Cache(
    cache_path="{cache_dir}/{_hash}.npy",
    cache_dir="okapi_tfidf_weighted_textual_embedding"
)
def get_okapi_tfidf_weighted_textual_embedding(
    path: str,
    k1: float = 1.5,
    b: float = 0.75,
    columns: Optional[str] = None,
    pretrained_model_name_or_path: str = "allenai/scibert_scivocab_uncased",
    read_csv_kwargs: Optional[Dict] = None,
    bert_tokenizer_kwargs: Optional[Dict] = None,
    bert_model_kwargs: Optional[Dict] = None,
    verbose: bool = True
) -> np.ndarray:
    """Returns tokens od the textual data available at the given file.

    Parameters
    -------------------------
    path: str
        Path from where to load the CSV.
    k1: float = 1.5
        K1 parameter for the OKAPI TFIDF
    b: float = 0.75
        B parameter for the OKAPI TFIDF
    columns: Optional[str] = None
        The columns to be taken into consideration for the tokenization
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
    # Compute the TFIDF scores of the provided document.
    tfidf_scores = get_tfidf_scores(
        path,
        k1=k1,
        b=b,
        columns=columns,
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        read_csv_kwargs=read_csv_kwargs,
        bert_tokenizer_kwargs=bert_tokenizer_kwargs,
        verbose=verbose
    )
    # Retrieve the requested pretrained word embedding.
    word_embedding = get_precomputed_word_embedding(
        pretrained_model_name_or_path=pretrained_model_name_or_path,
        bert_model_kwargs=bert_model_kwargs
    )
    # Compute the weighted embedding
    return np.array([
        np.average(
            a=word_embedding[list(tfidf_score.keys())],
            weights=list(tfidf_score.values()),
            axis=0,
            returned=False
        )
        for tfidf_score in tqdm(
            tfidf_scores,
            leave=False,
            desc="Computing weighted embedding",
            disable=not verbose
        )
    ])
