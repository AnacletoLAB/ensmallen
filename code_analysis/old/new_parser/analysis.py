import os
import json
from pprint import pprint
from lark import Lark
from glob import glob
from .rust_pass import RustPass

ROOT = os.path.abspath(os.path.dirname(__file__))

def get_analysis(path):
    with open(os.path.join(ROOT, "grammars/rust.lark"), "r") as f:
        grammar = f.read()

    parser = Lark(grammar)

    modules = []
    for file in glob(path):
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

        tree = parser.parse(text)
        #pprint(tree)

        module = RustPass().transform(tree)
        module.set_file(file)
        pprint(module)
        
        modules.append(module)
        
    return modules