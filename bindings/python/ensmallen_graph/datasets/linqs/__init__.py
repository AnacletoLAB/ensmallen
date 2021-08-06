"""This sub-module offers methods to automatically retrieve the graphs from LINQS repository."""

from .pubmeddiabetes import PubMedDiabetes
from .citeseer import CiteSeer
from .cora import Cora

__all__ = [
	"PubMedDiabetes", "CiteSeer", "Cora",
]