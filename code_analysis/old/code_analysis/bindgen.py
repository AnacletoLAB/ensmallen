import os
import re
import json
from ensmallen import EnsmallenGraph
from .utils import build_path

# TODO! aggiungere invalidazione del report su tutti i metodi con inplace

RETURN_TYPE_BLACKLIST = [
    "Counter",
]

ARGS_TYPE_BLACKLIST = [
    "Counter",
    # "Graph",
    # "&Graph",
    # "&mut Graph",
]

BLACKLIST = [
    "compute_hash",
]


def bindgen(args):
    with open(build_path("results/analysis.json"), "r") as f:
        analysis = json.load(f)

    filtered_functions = {}
    for file, vals in analysis.items():
        for function in vals["functions"]:
            if function["name"] in dir(EnsmallenGraph):
                continue

            if function["name"] in BLACKLIST:
                continue

            if "unchecked" in function["name"]:
                continue

            if function.get("modifiers", "") != "pub":
                continue

            if function.get("struct", "") != "Graph":
                continue
            
            if "name" not in function.keys():
                print("WTF", function)
                continue

            if "iter" in function["name"]:
                continue

            if any(
                "Iterator" in arg[1]
                for arg in function.get("args", [])
            ):
                continue

            if "Iterator" in function.get("return_type", ""):
                continue

            filtered_functions.setdefault(function.get("file"), [])
            filtered_functions[function["file"]].append(function)

        print(filtered_functions)
        for file, functions in filtered_functions.items():
            bindgen = ""
            bindgen += "use super::*;\n"
            bindgen += "impl Graph {\n"

            methods = []
            for function in functions:
                result = ""

                if len(function.get("args", [])) > 1:
                    signature = ", " + ", ".join(
                        x[0]
                        for x in function.get("args", [])[1:]
                    )
                else:
                    signature = ""

                result += "#[text_signature = \"($self{signature})\"]\n".format(
                    signature=signature
                )

                if "doc" in function:
                    doc = "\n".join(function.get("doc", []))
                    # Remove examples
                    doc = re.sub("```.+```", "", doc, flags=re.DOTALL)
                    # Remove example header
                    doc = re.sub("# Example[^#]+", "", doc, flags=re.DOTALL)
                    # Convert the arguments header
                    doc = re.sub("#\s+Arguments", "Parameters\n--------------", doc, flags=re.DOTALL)
                    doc = re.sub("Parameters\n--------------\n\n", "Parameters\n--------------\n", doc, flags=re.DOTALL)
                    doc = re.sub(" Parameters", "Parameters", doc)
                    # Convert the arguments in python format
                    doc = re.sub(r"[ \t]*\* `(.+?)`\s*:\s*(.+?)\s*-\s*(.+)", r"\1: \2,\n\t\3", doc)
                    # Type conversions
                    doc = re.sub(r"Vec<(.+?)>", r"List[\1]", doc)
                    doc = re.sub(r"EdgeTypeT", r"int", doc)
                    doc = re.sub(r"NodeTypeT", r"int", doc)
                    doc = re.sub(r"String", r"str", doc)
                    doc = re.sub(r"NodeT", r"int", doc)
                    doc = re.sub(r"EdgeT", r"int", doc)
                    doc = re.sub(r"u64", r"int", doc)
                    doc = re.sub(r"usize", r"int", doc)
                    doc = re.sub(r"f64", r"float", doc)
                    doc = re.sub(r"&", r"", doc)
                    doc = re.sub(r"Option<(.+?)>", r"Union[\1, None]", doc)
                    doc = re.sub(r"HashSet<(.+?)>", r"Dict[\1]", doc)
                    # Remove white space at the edges
                    doc = doc.strip()
                    result += "\n".join("/// " + x for x in doc.split("\n"))
                    result += "\n"

                result += "///\n/// [Automatically generated binding]\n"
                result += "/// [Automatically generated documentation]\n"

                if len(function.get("args", [])) > 1:
                    print(function["args"])
                    args = function["args"][0][1] + ", " + ", ".join([
                        "%s : %s"%tuple(x) 
                        for x in function["args"][1:]
                    ])
                else:
                    args = function["args"][0][1]

                if any(
                    e in a
                    for a in args
                    for e in ARGS_TYPE_BLACKLIST
                ):
                    continue

                result += "fn {name}({args})".format(
                    name=function["name"],
                    args=args
                )

                if "return_type" in function:
                    result_type = function["return_type"]
                    if any(
                        e in result_type
                        for e in RETURN_TYPE_BLACKLIST
                        ):
                        continue

                    if result_type.startswith("Result"):
                        result_type = "Py" + result_type
                        result_type, _, _ = result_type.rpartition(",")
                        result_type += ">"

                    result_type = result_type.replace("Graph", "EnsmallenGraph")

                    result += " -> %s "%result_type

                body = "\tself.graph." + function["name"]
                body += "(" 
                body += ", ".join(x[0] for x in function["args"][1:])
                body += ")"

                if "return_type" in function:
                    result_type = function["return_type"]
                    
                    if result_type.startswith("Result"):
                        if "Graph" in result_type:
                            body = "\tOk(EnsmallenGraph{graph:pe!(%s)})"%body[1:]
                        else:
                            body = "\tpe!(%s)"%body[1:]
                    else:
                        if "Graph" in result_type:
                            body = "\tEnsmallenGraph{graph:%s}"%body[1:]
                        else:
                            body = "\t%s"%body[1:]


                result += "{\n"
                result += body
                result += "\n}\n"

                methods.append((function["name"], result))

            methods.sort(key=lambda x: x[0])

            for name, method in methods:
                bindgen += "\n\t" + "\n\t".join(method.split("\n"))
            bindgen += "\n}\n"

            with open(build_path(os.path.join("bindgen", file)), "w") as f:
                f.write(bindgen)