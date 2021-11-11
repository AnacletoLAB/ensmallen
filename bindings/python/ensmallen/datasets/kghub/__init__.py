"""This sub-module offers methods to automatically retrieve the graphs from KGHub repository."""

from .kgcovid19 import KGCOVID19
from .kgmicrobe import KGMicrobe
from .ecokg import EcoKG

__all__ = [
	"KGCOVID19", "KGMicrobe", "EcoKG",
]