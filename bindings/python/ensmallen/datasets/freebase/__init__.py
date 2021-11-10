"""This sub-module offers methods to automatically retrieve the graphs from FreeBase repository."""

from .freebase import FreeBase
from .freebase2wikidata import FreeBase2WikiData

__all__ = [
	"FreeBase", "FreeBase2WikiData",
]