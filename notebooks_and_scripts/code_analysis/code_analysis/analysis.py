import json
import glob
import argparse
from .utils import build_path
from .parsers import RustParser

def read_file(path):
    with open(path, "r") as f:
        return f.read()
    
def read_files(path):
    return [
        (file, read_file(file))
        for file in glob.glob(path)
    ]

def analysis(args):
    parser = argparse.ArgumentParser(description='Generate a json with the informations about all the methods in the selected files.')  
    parser.add_argument('-p', "--path", type=str, default="../../graph/src/*.rs", help='A glob path of which files to parse. Default: %(default)s')
    args = parser.parse_args(args)

    path = build_path(args.path)   
    files = read_files(path)

    p = RustParser()
    p.parse_files(files)

    with open(build_path("results/analysis.json"), "w") as f:
        f.write(json.dumps(p.functions, indent=4))
