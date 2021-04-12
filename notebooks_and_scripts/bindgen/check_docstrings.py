import re
import sys
import json
from ensmallen_graph import EnsmallenGraph

with open("analysis.json", "r") as f:
    functions = json.load(f)

def check_doc_validity(doc, function):
    result = []
    doc = "\n".join(doc)
    if not re.findall("\s*#\s+Examples", doc):
        result.append("Missing Example header")

    if "```" not in doc:
        result.append("Missing Example")

    if not re.findall("\s*#\s+Arguments", doc):
        result.append("Missing Arguments Header")

    arguments_doc = re.findall("\s*\*\s+`?(\S+)`?\s*:\s*(.+?)\s*-\s*(.+)\s*", doc)

    arguments = [
        x[0] 
        for x in function["args"]
        if x[0] != "self"    
    ]
    
    if len(arguments_doc) < len(arguments):
        result.append("Missing the documentation of some arguments.")

    doc_args = {
        arg_name.strip("`")
        for arg_name, arg_type, arg_description in arguments_doc
    }

    for arg in arguments:
        if arg not in doc_args:
            result.append("Missing the documentation for {}".format(arg))

    return result

result = {}

for function in functions:
    if function.get("modifiers", "") != "pub":
        continue

    if function.get("struct", "") != "Graph":
        continue

    errors = check_doc_validity(function.get("doc", []), function)

    if errors:
        result[function.get("name")] = errors


print(json.dumps(result, indent=4))
if result != {}:
    with open("dockstrings_errors.json", "w") as f:
        json.dump(result, f, indent=4)

    sys.exit(1)

sys.exit(0)