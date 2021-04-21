import json
from pprint import pprint
from lark import Lark, Transformer

with open("./grammar.lark", "r") as f:
    grammar = f.read()
    
rust_parser = Lark(grammar, parser="lalr")

class Type:

class MyTransformer(Transformer):
    def doc(self, values):
        return {"doc":"".join([x.value for x in values])}

    def use_statement(self, values):
        return {
            "use":values,
        }

    def function_body(self, values): 
        return {
            "function_body":values[0]
        }

    def function_name(self, values):
        result = {
            "function_name": values[0],
        }
        if len(values) > 1:
            result["generics"] = values[1]
        return result

    def function_statements(self, values):
        return {
            "functions":values
        }

    def function_statement(self, values):
        return merge_dicts(values)

    def VISIBILITY(self, values):
        return {
            "visibility":values.value
        }

    def simple_type(self, values):
        result = {
            "identifier":values[0],
        }
        if len(values) > 1:
            result["generics"] = values[1]
        return "simple_type", result

    def impl_type(self, values):
        return "impl_type", {
            "inner":values[0],
            "traits": values[1:] or None
        }

    def tuple_type(self, values):
        return "tuple_type", values

    def struct_type(self, values):
        return {
            "struct_type":values
        }

    def trait_type(self, values):
        return {
            "trait_type":values
        }

    def return_type(self, values):
        return {
            "return_type":values
        }

    def impl_statement(self, values):
        return merge_dicts(values)

    def type(self, values):
        return {
            "variant":values[0][0],
            "value":values[0][1]   
        }

    def args(self, args):
        return {
            "args":list(args)
        }

    def SELF(self, values):
        return {
            "type":"self",
            "modifiers":values[0]
        }

    def arg(self, arg):
        return {
            "identifier":arg[0],
            "type":arg[1],
        }

    def MODIFIER(self, values):
        print("MOD:", values)

    def IDENTIFIER(self, values):
        return values.value

    def generics(self, values):
        return values

    def lifetime(self, values):
        return {
            "type":"lifetime",
            "identifier":values[0],
        }

    def start(self, values):
        return merge_dicts(values)

from glob import glob

result = {}
for file in glob("../../graph/src/bitmaps.rs"):
    print(file)
    if "test_utilities" in file:
        continue
    if "types" in file:
        continue
    if "lib.rs" in file:
        continue
    if "utils.rs" in file:
        continue
    with open(file, "r") as f:
        text = f.read()

    tree = rust_parser.parse(text)
    print(tree.pretty())
    parsed = MyTransformer().transform(tree)
    pprint(parsed)
    result[file] = parsed

pprint(result)
with open("new_analysis.json", "w") as f:
    json.dump(result, f, indent=4)