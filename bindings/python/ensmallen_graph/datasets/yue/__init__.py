"""This sub-module offers methods to automatically retrieve the graphs from Yue repository."""

from .stringppi import StringPPI
from .ctddda import CTDDDA
from .drugbankddi import DrugBankDDI
from .ndfrtdda import NDFRTDDA
from .mashupppi import MashupPPI
from .node2vecppi import node2vecPPI
from .clintermcooc import ClinTermCOOC

__all__ = [
	"StringPPI", "CTDDDA", "DrugBankDDI", "NDFRTDDA", "MashupPPI", "node2vecPPI",
	"ClinTermCOOC",
]