import re
import sys
import json
from ensmallen_graph import EnsmallenGraph
from .utils import build_path

def check_doc(args):

    with open(build_path("results/analysis.json"), "r") as f:
        analysis = json.load(f)

    with open(build_path("results/doc_analysis.json"), "r") as f:
        docs = json.load(f)

    result = {}

    for file, values in analysis.items():
        for function in values["functions"]:
            if function.get("modifiers", "") != "pub":
                continue

            if function.get("struct", "") != "Graph":
                continue

            fn_name = function.get("name", "")
            errors = docs[fn_name]["errors"]

            if errors:
                file = function.get("file", "")
                result.setdefault(file, {})
                errs = [
                    "[{:>4}] {}".format(x["doc_line"], x["msg"])
                    for x in errors
                    if x["msg"] not in [
                        "Missing Example",
                        "The example section is missing or in the wrong order.",
                    ]
                ]
                if errs:
                    result[file][fn_name] = errs


    print(json.dumps(result, indent=4))
    if result != {}:
        with open(build_path("results/dockstrings_errors.json"), "w") as f:
            json.dump(result, f, indent=4)

        sys.exit(1)

    sys.exit(0)