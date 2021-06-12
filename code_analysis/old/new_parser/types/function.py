from typing import List, Union
from dataclasses import dataclass

from .documentation import Documentation
from .visibility import Visibility
from .type import Type
from .lifetime import Lifetime
from .arguments import Arguments

@dataclass
class Function:
    attributes: List[str]
    documentation: Union[Documentation, None]
    visibility: Visibility
    function_name: str
    generics: Union[List[Union[Type, Lifetime]], None]
    arguments: Arguments
    return_type: Union[Type, None]
    body: str

    def has_return_type(self):
        return self.return_type is None

    def has_generics(self):
        return self.generics is None

    def add_attribute(self, attribute):
        self.attributes.append(attribute)