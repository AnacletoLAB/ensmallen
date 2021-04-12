import json
import argparse
from .utils import build_path, to_dot

def method_dependancies(args):
    parser = argparse.ArgumentParser(description='Generate a .dot graph with all the dependancies of the given method.')  
    parser.add_argument('METHOD', type=str, help='Which method to analyze')
    args = parser.parse_args(args)

    with open(build_path("results/dependancies.json"), "r") as f:
        deps = json.load(f)

    stack = [args.METHOD]
    to_print = set()

    # Do a simple visit algorithm to find all its dependancies
    while stack:
        method = stack.pop()
        neightbours = deps[method]
        stack.extend(list(set(neightbours) - to_print))
        to_print |= set(neightbours)

    with open("method.dot", "w") as f:
        f.write(to_dot(to_print, [
            (src, dst)
            for src in to_print
            for dst in deps[src]
        ]))
