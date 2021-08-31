"""This sub-module offers methods to automatically retrieve the graphs from Yue repository."""

from .node2vecppi import node2vecPPI
from .ctddda import CTDDDA
from .drugbankddi import DrugBankDDI
from .mashupppi import MashupPPI
from .ndfrtdda import NDFRTDDA
from .clintermcooc import ClinTermCOOC
from .stringppi import StringPPI

__all__ = [
	"node2vecPPI", "CTDDDA", "DrugBankDDI", "MashupPPI", "NDFRTDDA", "ClinTermCOOC",
	"StringPPI",
]