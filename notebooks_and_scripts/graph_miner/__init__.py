"""Submodule to automatically generated graph methods for graph retrieval."""
from .repositories import (JAXGraphRepository, KGHubGraphRepository,
                           KGOBOGraphRepository, LINQSGraphRepository,
                           MonarchInitiativeGraphRepository,
                           NetworkRepositoryGraphRepository,
                           PheKnowLatorKGGraphrepository,
                           StringGraphRepository, YueGraphRepository,
                           ZenodoGraphRepository)

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
    "KGOBOGraphRepository"
]
