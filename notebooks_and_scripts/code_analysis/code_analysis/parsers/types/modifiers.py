from .utils import *

class Modifiers:
    def __init__(self):
        self.mutable = False
        self.reference = False
        self.reference_lifetime = None

    def parse(text: str) -> Tuple[str, Self]:
        modifiers = Modifiers()
        text = text.lstrip()
        if text[0] == "&":
            modifiers.reference = True
            _, *text = text
            text = text.lstrip()

            if text[0] == "'":
                _, *text = text
                text, lifetime_ident = Identifier.parse(text)
                text = text.lstrip()
                modifiers.reference_lifetime = lifetime_ident
        
        if text.startswith("mut"):
            modifiers.mutable = True
            text = text[3:].lstrip()
        return text, modifiers