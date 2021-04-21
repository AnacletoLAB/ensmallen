from typing import *
from .utils import *
from .lifetime import Lifetime

class Type:
    def __init__(self):
        self.ident = ""
        self.generics = None
    
    def _parse_impl(text: str) -> Tuple[str, Self]:
        pass

    def _parse_dyn(text: str) -> Tuple[str, Self]:
        pass

    def _parse_common_type(text: str) -> Tuple[str, Self]:
        text, type_ident = Identifier.parse(text)
        result.ident = type_ident
        text, generics = Generics.parse(text)
        result.generics = generics
        return text, result

    def parse(text: str) -> Tuple[str, Self]:
        result = Type()
        # Parse the modifiers if present.
        text, modifiers = Modifiers.parse(text)
        text = text.lstrip()

        if text.startswith("impl"):
            return Type._parse_impl(text)
        elif text.startswith("dyn"):
            return Type._parse_dyn(text)
        else:
            return Type._parse_common_type(text)