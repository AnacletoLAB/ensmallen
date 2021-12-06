"""Sub-module with repositories offering graph data."""
from .string_graph_repository import StringGraphRepository
from .network_repository_graph_repository import NetworkRepositoryGraphRepository
from .kg_hub_graph_repository import KGHubGraphRepository
from .yue_graph_repository import YueGraphRepository
from .linqs_graph_repository import LINQSGraphRepository
from .monarch_initiative_graph_repository import MonarchInitiativeGraphRepository
from .zenodo_graph_repository import ZenodoGraphRepository
from .pheknowlatorkg_graph_repository import PheKnowLatorKGGraphrepository
from .jax_repository import JAXGraphRepository
from .kg_obo_graph_repository import KGOBOGraphRepository
from .wikidata_graph_repository import WikiDataGraphRepository
from .freebase_graph_repository import FreebaseGraphRepository
from .wikipedia_graph_repository import WikipediaGraphRepository
from .ctd_graph_repositories import CTDGraphRepository

__all__ = [
    "StringGraphRepository",
    "NetworkRepositoryGraphRepository",
    "KGHubGraphRepository",
    "YueGraphRepository",
    "LINQSGraphRepository",
    "MonarchInitiativeGraphRepository",
    "ZenodoGraphRepository",
    "PheKnowLatorKGGraphrepository",
    "JAXGraphRepository",
    "KGOBOGraphRepository",
    "WikiDataGraphRepository",
    "FreebaseGraphRepository",
    "WikipediaGraphRepository",
    "CTDGraphRepository"
]
