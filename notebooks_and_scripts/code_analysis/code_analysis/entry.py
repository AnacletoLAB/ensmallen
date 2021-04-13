import os
import sys
import argparse
from .analysis import analysis
from .dependancies import dependancies
from .method_dependancies import method_dependancies
from .check_docstrings import check_doc
from .bindgen import bindgen
from .doc_analysis import doc_analysis
from .utils import build_path

commands = {
    "analysis":analysis,
    "dependancies":dependancies,
    "method_dependancies":method_dependancies,
    "check_doc":check_doc,
    "bindgen":bindgen,
    "doc_analysis":doc_analysis,
}

# each 
deps = {
    "analysis":[],

    "dependancies":["analysis"],
    "method_dependancies":["dependancies"],

    "doc_analysis":["analysis"],
    "check_doc":["doc_analysis"],
    
    "bindgen":["analysis", "doc_analysis"],
}

def solve_deps(start):
    dep = deps[start]
    if dep == []:
        return

    for sub_dep in dep:
        solve_deps(sub_dep)
        commands[sub_dep]([])

def entrypoint():
    os.makedirs(build_path("results"), exist_ok=True)
    parser = argparse.ArgumentParser(description='Code analysis passes')  
    parser.add_argument('COMMAND', type=str, choices=list(commands.keys()), help='Which pass to run on the code.')
    args = parser.parse_args(sys.argv[1:2])
    
    command = args.COMMAND
    solve_deps(command)
    commands[command](sys.argv[2:])