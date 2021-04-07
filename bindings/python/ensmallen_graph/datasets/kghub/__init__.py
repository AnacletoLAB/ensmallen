"""This sub-module offers methods to automatically retrieve the graphs from KGHub repository."""

from .gocams import GOCAMs
from .string import STRING
from .drugcentral import DrugCentral
from .intact import IntAct
from .pharmgkb import PharmGKB
from .sarscov2geneannot import SARSCOV2GeneAnnot
from .zhouhostproteins import ZhouHostProteins
from .kgcovid19 import KGCOVID19

__all__ = [
	"GOCAMs", "STRING", "DrugCentral", "IntAct", "PharmGKB", "SARSCOV2GeneAnnot",
	"ZhouHostProteins", "KGCOVID19",
]