from typing import *
from utils import *

class Lifetime:
    def __init__(self, identifier:str):
        self.identifier = identifier

    def parse(text: str) -> Tuple[str, Lifetime]:
        if not text.startswith("'"):
            raise ValueError("Cannot parse lifetime that does not start with ', The text is %s"%text)

        text = text[1:] # Skip the '
        text, lifetime_identifier = parse_identifier(text)
        return 