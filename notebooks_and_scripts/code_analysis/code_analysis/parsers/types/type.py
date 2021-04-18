from typing import *
from .utils import *
from .lifetime import Lifetime
from .modifiers import Modifiers

class Type:
    def __init__(self, identifier: str, modifiers: Modifiers, generics:List[Union[Type, Lifetime]] = []):
        
    
    def _parse_impl(text: str) -> Tuple[str, Type]:
        pass

    def _parse_dyn(text: str) -> Tuple[str, Type]:
        pass

    def parse(text: str) -> Tuple[str, Type]:
        # Parse the modifiers if present.
        modifiers = Modifiers()
        if text[0] == "&":
            modifiers.set_reference()
            text = text[1:]
            text = skip_whitespace(text)
            if text.startswith("'"):
                text, lifetime = Lifetime.parse(text)
                modifiers.get_reference_lifetime(lifetime)

        if text.startswith("mut"):
            text = text[3:]
            modifiers.set_mutable()

        # Check if it's an impl type
        if text.startswith("impl"):
            return Type._parse_impl(text)

        # Check if it's an impl type
        if text.startswith("dyn"):
            return Type._parse_dyn(text)
        
        text, type_ident = parse_identifier(text)

        if text[0] == "<":
            text, generics_text = skip_to_match(text)

            while generics_text:
                text, generic = Type.parse(generics_text)
        
        return text, Type()

    def dump(self):
        pass

    def load(self):
        pass

        