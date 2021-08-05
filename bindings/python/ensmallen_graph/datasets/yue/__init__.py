"""This sub-module offers methods to automatically retrieve the graphs from Yue repository."""

from .mashupppi import MashupPPI
from .ctddda import CTDDDA
from .node2vecppi import node2vecPPI
from .clintermcooc import ClinTermCOOC
from .ndfrtdda import NDFRTDDA
from .drugbankddi import DrugBankDDI
from .stringppi import StringPPI

__all__ = [
	"MashupPPI", "CTDDDA", "node2vecPPI", "ClinTermCOOC", "NDFRTDDA", "DrugBankDDI",
	"StringPPI",
]