import re
import sys
import json
from ensmallen_graph import EnsmallenGraph
from .utils import build_path

def check_doc(args):

    with open(build_path("results/analysis.json"), "r") as f:
        functions = json.load(f)

    with open(build_path("results/doc_analysis.json"), "r") as f:
        docs = json.load(f)

    result = {}

    for function in functions:
        if function.get("modifiers", "") != "pub":
            continue

        if function.get("struct", "") != "Graph":
            continue

        fn_name = function.get("name", "")
        errors = docs[fn_name]["errors"]

        if errors:
            result[fn_name] = [
                "[{:>4}] {}".format(x["doc_line"], x["msg"])
                for x in errors
            ]


    print(json.dumps(result, indent=4))
    if result != {}:
        with open(build_path("results/dockstrings_errors.json"), "w") as f:
            json.dump(result, f, indent=4)

        sys.exit(1)

    sys.exit(0)