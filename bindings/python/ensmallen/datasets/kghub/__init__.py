"""This sub-module offers methods to automatically retrieve the graphs from KGHub repository."""

from .kgmicrobe import KGMicrobe
from .kgidg import KGIDG
from .kgcovid19 import KGCOVID19
from .ecokg import EcoKG

__all__ = [
	"KGMicrobe", "KGIDG", "KGCOVID19", "EcoKG",
]