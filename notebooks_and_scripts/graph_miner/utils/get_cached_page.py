"""Submodule providing a simple downloader of pages with caching layer."""
from cache_decorator import Cache
import requests


@Cache(
    cache_path="cached_pages/{_hash}.txt",
    validity_duration=60*60*24*7
)
def get_cached_page(url: str) -> str:
    """Returns text from the given page url."""
    return requests.get(url).text
