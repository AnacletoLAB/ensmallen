

class File:
    def __init__(self):
        self.doc = ""
        self.types = []
        self.impls = []
        self.imports = []

    def parse(text: str) -> File:
        