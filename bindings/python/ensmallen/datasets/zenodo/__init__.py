"""This sub-module offers methods to automatically retrieve the graphs from Zenodo repository."""

from .gianttn import GiantTN
from .wikilinkde import WikiLinkDE
from .wikilinken import WikiLinkEN
from .wikilinkes import WikiLinkES
from .wikilinkfr import WikiLinkFR
from .wikilinkit import WikiLinkIT
from .wikilinknl import WikiLinkNL
from .wikilinkpl import WikiLinkPL
from .wikilinkru import WikiLinkRU
from .wikilinksv import WikiLinkSV

__all__ = [
	"GiantTN", "WikiLinkDE", "WikiLinkEN", "WikiLinkES", "WikiLinkFR", "WikiLinkIT",
	"WikiLinkNL", "WikiLinkPL", "WikiLinkRU", "WikiLinkSV",
]