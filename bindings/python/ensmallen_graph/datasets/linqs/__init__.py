"""This sub-module offers methods to automatically retrieve the graphs from LINQS repository."""

from .citeseer import CiteSeer
from .cora import Cora
from .pubmeddiabetes import PubMedDiabetes

__all__ = [
	"CiteSeer", "Cora", "PubMedDiabetes",
]