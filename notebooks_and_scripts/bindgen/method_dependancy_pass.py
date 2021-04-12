import sys
import json

if len(sys.argv) < 2:
    print("Usage: this.py METHOD_TO_STUDY")
    sys.exit(1)

method = sys.argv[1]

with open("./dependancies.json", "r") as f:
    deps = json.load(f)

stack = [method]
to_print = set()

while stack:
    method = stack.pop()
    neightbours = deps[method]
    stack.extend(list(set(neightbours) - to_print))
    to_print |= set(neightbours)

with open("method.dot", "w") as f:
    f.write("digraph deps {\n")
    f.write("\tnode [shape=box];\n")

    f.write("\n")

    for sub in to_print:
        f.write(f"\t{sub}\n")

    f.write("\n")

    for src in to_print:
        for dst in deps[src]:
            f.write(f"\t\"{src}\" -> \"{dst}\"\n")


    f.write("}")
