from typing import List, Union
from dataclasses import dataclass

from .function import Function
from .implementation import Implementation

class Module:
    file_name: str
    uses: List[str]
    impls: List[Implementation]
    functions: List[Function]
    # types
    # const
    # static

    def __init__(self, uses, impls, functions):
        self.uses = uses
        self.impls = impls
        self.functions = functions

    def set_file(self, file):
        self.file_name = file

    def __str__(self):
        return f"Module({self.file_name =}, {self.uses =}. {self.impls =}, {self.functions =})"

    def __repr__(self):
        return self.__str__()