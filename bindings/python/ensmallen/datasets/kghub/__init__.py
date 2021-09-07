"""This sub-module offers methods to automatically retrieve the graphs from KGHub repository."""

from .kgmicrobe import KGMicrobe
from .kgcovid19 import KGCOVID19

__all__ = [
	"KGMicrobe", "KGCOVID19",
]