from dataclasses import dataclass

class Function:
    def __init__(self, name):
        self.name = name
        self.args = []
        