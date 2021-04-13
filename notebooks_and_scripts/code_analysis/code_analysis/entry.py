import os
import sys
import argparse
from .analysis import analysis
from .dependancies import dependancies
from .method_dependancies import method_dependancies
from .check_docstrings import check_doc
from .bindgen import bindgen
from .utils import build_path

commands = {
    "analysis":analysis,
    "dependancies":dependancies,
    "method_dependancies":method_dependancies,
    "check_doc":check_doc,
    "bindgen":bindgen,
}

def entrypoint():
    os.makedirs(build_path("results"), exist_ok=True)
    parser = argparse.ArgumentParser(description='Code analysis passes')  
    parser.add_argument('COMMAND', type=str, choices=list(commands.keys()), help='Which pass to run on the code.')
    args = parser.parse_args(sys.argv[1:2])
    commands[args.COMMAND](sys.argv[2:])