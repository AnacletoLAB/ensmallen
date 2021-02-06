"""Sub-module with repositories offering graph data."""
from .string_graph_repository import StringGraphRepository
from .network_repository_graph_repository import NetworkRepositoryGraphRepository

__all__ = [
    "StringGraphRepository",
    "NetworkRepositoryGraphRepository"
]
