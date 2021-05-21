from dataclasses import dataclass

@dataclass
class Type:
    value:str

    def __str__(self):
        return self.value

class StructType(Type):
    pass

class TraitType(Type):
    pass