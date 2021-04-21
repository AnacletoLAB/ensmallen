import json
from pprint import pprint
from lark import Lark, Transformer

with open("./grammar.lark", "r") as f:
    grammar = f.read()
    
rust_parser = Lark(grammar, debug=True)

with open("../../graph/src/bitmaps.rs", "r") as f:
    text = f.read()

tree = rust_parser.parse(text)

class MyTransformer(Transformer):
    def doc(self, values):
        return {"doc":"".join([x.value for x in values])}

    def use_statement(self, values):
        pass

    def function_body(self, values): 
        return "".join([x for x in values])

    def function_name(self, values):
        result = {
            "name": values[0],
        }
        if len(values) > 1:
            result["generics"] = values[1]
        return result


    def function_statement(self, values):
        pprint(values)
        return {k: v for d in values for k, v in d.items()}

    def VISIBILITY(self, values):
        return {
            "visibility":values
        }

    def standard_type(self, values):
        result = {
            "type":"standard_type",
            "identifier":values[0],
        }
        if len(values) > 1:
            result["generics"] = values[1]
        return result

    def impl_type(self, values):
        return {
            "type":"impl_type",
            "inner":values[0],
            "traits": values[1:] or None
        }

    def tuple_type(self, values):
        return {
            "type":"tuple_type",
            "values":values,
        }

    def impl_statement(self, values):
        return {
            "doc": values[0],
            "struct": values[2],
            "trait": values[1],
            "functions": values[3],
        }

    def type(self, values):
        return values[0]

    def IDENTIFIER(self, values):
        return values.value

    def generics(self, values):
        return values[0]

    def lifetime(self, values):
        return {
            "type":"lifetime",
            "identifier":values[0],
        }

print(MyTransformer().transform(tree))
print(json.dumps(MyTransformer().transform(tree), indent=4))