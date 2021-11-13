"""This sub-module offers methods to automatically retrieve the graphs from Zenodo repository."""

from .wikilinkit import WikiLinkIT
from .wikilinkfr import WikiLinkFR
from .gianttn import GiantTN
from .wikilinkde import WikiLinkDE
from .wikilinknl import WikiLinkNL
from .wikilinksv import WikiLinkSV
from .wikilinkpl import WikiLinkPL
from .wikilinkes import WikiLinkES
from .wikilinken import WikiLinkEN
from .wikilinkru import WikiLinkRU

__all__ = [
	"WikiLinkIT", "WikiLinkFR", "GiantTN", "WikiLinkDE", "WikiLinkNL", "WikiLinkSV",
	"WikiLinkPL", "WikiLinkES", "WikiLinkEN", "WikiLinkRU",
]