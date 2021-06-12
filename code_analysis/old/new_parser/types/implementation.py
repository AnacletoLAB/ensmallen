from typing import List, Union
from dataclasses import dataclass

from .documentation import Documentation
from .function import Function

@dataclass
class Implementation:
    doc: Union[Documentation, None]
    struct: str
    trait: Union[str, None]
    functions: List[Function]