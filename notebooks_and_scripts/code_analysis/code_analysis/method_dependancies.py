import json
import argparse
from .utils import build_path, to_dot

COLORS = {
    0:"blue",
    1:"red",
    2:"orange",
    3:"green",
    "default":"black",
}

def find(lista, value, default):
    try:
        return lista.index(value)
    except ValueError:
        return default

def method_dependancies(args):
    parser = argparse.ArgumentParser(description='Generate a .dot graph with all the dependancies of the given method.')  
    parser.add_argument('METHOD', type=str, help='Which method to analyze')
    args = parser.parse_args(args)

    with open(build_path("results/dependancies.json"), "r") as f:
        deps = json.load(f)

    structs = list({
        x.split("::")[0]
        for v in deps.values()
        for x in v
        if len(x.split("::")) > 1
    })

    # Initalize the stack of nodes to visit
    stack = [args.METHOD]
    # And the set of nodes visisted
    to_print = set()
    # Do a simple visit algorithm to find all its dependancies
    while stack:
        method = stack.pop()
        neightbours = deps[method]
        stack.extend(list(set(neightbours) - to_print))
        to_print |= set(neightbours)

    nodes = {
        (dst, COLORS[find(structs, dst.split("::")[0], "default")])
        for node in to_print
        for dst in deps[node]
    }

    # Dump the dot file
    with open(build_path("results/method.dot"), "w") as f:
        f.write(to_dot(nodes, [
            (src, dst)
            for src in to_print
            for dst in deps[src]
        ]))
