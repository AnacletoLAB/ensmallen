"""This sub-module offers methods to automatically retrieve the graphs from LINQS repository."""

from .pubmeddiabetes import PubMedDiabetes
from .cora import Cora
from .citeseer import CiteSeer

__all__ = [
	"PubMedDiabetes", "Cora", "CiteSeer",
]