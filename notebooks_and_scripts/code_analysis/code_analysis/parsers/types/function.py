from dataclasses import dataclass

class Function:
    def __init__(self, name):
        self.name = name
        self.args = []
        

    def dump(self) -> str:
        return {
            "name":self.name,
            
        }
        