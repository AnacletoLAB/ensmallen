from typing import List, Tuple
from dataclasses import dataclass
from .type import Type

@dataclass
class Arguments:
    arguments: List[Tuple[str, Type]]