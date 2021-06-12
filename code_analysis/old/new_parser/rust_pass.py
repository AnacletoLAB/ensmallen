from typing import *
from lark import Transformer, Token, Tree
from .types import *

class RustPass(Transformer):
    def doc_line(self, values):
        return " ".join(values)

    def doc(self, values) -> Documentation:
        attributes = []
        doc = []
        for val in values:
            if isinstance(val, Token):
                if val.type == "DOC_LINE":
                    doc.append(val.value)
                elif val.type == "ATTRIBUTE":
                    doc.append(val.value)
                elif val.type == "COMMENT":
                    pass
                else:
                    raise ValueError("Cannot parse this token '{}' for the doc.".format(val))
            else:
                raise ValueError("The doc cannot parse the object: '{}'".format(val))
            
        return Attributes(attributes), Documentation(doc)

    def function_statement(self, values) -> Function:
        attributes = None 
        documentation = None 
        visibility = None 
        function_name = "" 
        generics = None
        arguments = None
        return_type = None
        body = ""
        for value in values:
            if isinstance(value, tuple):
                if isinstance(value[0], Attributes):
                    attributes = value[0]
                    documentation = value[1]
                elif isinstance(value[0], str):
                    name = value[0]
                    generics = value[1]
                else:
                    raise ValueError("Cannot parse tuple '{}' for function_statement".format(value))
            elif isinstance(value, Visibility):
                visibility = value
            elif isinstance(value, Arguments):
                arguments = value
            elif isinstance(value, Type):
                return_type = value
            elif isinstance(value, str):
                body = value
            else:
                raise ValueError("Cannot parse object '{}' for function_statement".format(value))

        return Function(attributes, documentation, visibility, function_name, generics, arguments, return_type, body)

    def visibility(self, values):
        return Visibility(str(values))


    def lifetime(self, values) -> Lifetime:
        # TODO! improve
        return str(values)

    def modifier(self, values) -> str:
        # TODO! improve
        return str(values)

    def args(self, values) -> Arguments:
        return Arguments(values)

    def arg(self, values) -> Tuple[str, Type]:
        if len(values) > 2:
            return values[1], values[2]

        return values[0], values[1]

    def self(self, values):
        return "self", Type(values[-1])

    def function_body(self, values):
        # TODO! improve
        return "".join(values)

    def NOT_BRACE(self, values):
        return "".join(values)

    def function_name(self, values):
        return values[0], None

    def generic_value(self, values):
        return values

    def simple_type(self, values):
        print("Simpletv ", values)
        result = ""
        for value in values:
            if isinstance(value, Token):
                if value.type == "IDENTIFIER":
                    result += value.value
            elif isinstance(value, Tree):
                pass

        return Type(result)

    def type(self, values) -> Type:
        # TODO! improve
        print("TYPE values", values)
        modifiers = None
        result = ""
        for value in values:
            if isinstance(value, Type):
                result += value.value

        print("Result ", result)
        return Type(result)

    def struct_type(self, values) -> StructType:
        return StructType(values[0].value)

    def trait_type(self, values) -> TraitType:
        return TraitType(values[0].value)

    def impl_content(self, values) -> List[Function]:
        # TODO! improve
        return [
            x
            for x in values
            if isinstance(x, Function)
        ]

    def impl_statement(self, values) -> Implementation:
        doc = None
        struct = ""
        trait = None
        functions = []

        for value in values:
            if isinstance(value, tuple):
                doc = value[1]
            elif isinstance(value, TraitType):
                trait = value
            elif isinstance(value, StructType):
                struct = value
            elif isinstance(value, list):
                functions = value
            else:
                raise ValueError("Cannot parse object {} in impl.".format(value))
        return Implementation(doc, struct, trait, functions)

    def use_statement(self, values):
        # TODO! improve
        return None

    def start(self, values):
        uses = []
        impls = []
        functions = []
        for value in values:
            if isinstance(value, Function):
                functions.append(value)
            elif isinstance(value, Implementation):
                impls.append(value)
            elif value is None:
                pass
            else:
                raise ValueError("Cannot parse object {} in start.".format(value))
        return Module(uses, impls, functions)