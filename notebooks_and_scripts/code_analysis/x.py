import json
from pprint import pprint
from lark import Lark, Transformer

with open("./new_parser/grammars/rust.lark", "r") as f:
    grammar = f.read()
    
rust_parser = Lark(grammar, parser="lalr")



class MyTransformer(Transformer):
    def __init__(self):
        self.function_names = []

    def function_name(self, values):
        self.function_names.append(values[0].value)
        result = {
            "function_name": values[0],
        }
        if len(values) > 1:
            result["generics"] = values[1]
        return result

from glob import glob


def check_dual(functions, A, B):
    for function in functions:
        if "unchecked" in function:
            continue
        
        if "node_name" in function:
            continue

        if function.endswith(A) and B not in function:
            inverse = function[:-len(A)] + B
            if inverse not in functions:
                print(function, inverse, inverse in functions)

        if function.endswith(B) and A not in function:
            inverse = function[:-len(B)] + A
            if inverse not in functions:
                print(function, inverse, inverse in functions)


result = {}
for file in glob("../../graph/src/queries.rs"):
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
    parser = MyTransformer()
    parsed = parser.transform(tree)
    pprint(parser.function_names)

    print("#############")


    check_dual(parser.function_names, "node_type_id", "node_type_name")
    check_dual(parser.function_names, "edge_type_id", "edge_type_name")

    result[file] = parsed