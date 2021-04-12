import json

with open("./analysis.json", "r") as f:
    functions = json.load(f)


functions_names = {
    x["name"] 
    for x in functions
    if "name" in x
}


result = {}
for function in functions:
    if not "name" in function:
        continue

    result[function["name"]] = [
        sub
        for sub in functions_names
        if sub + "(" in function.get("body", "")
    ]

with open("dependancies.json", "w") as f:
    json.dump(result, f)

with open("dependancies.dot", "w") as f:
    f.write("digraph deps {\n")
    f.write("\tnode [shape=box];\n")

    f.write("\n")

    for sub in functions_names:
        f.write(f"\t{sub}\n")

    f.write("\n")

    for src, dsts in result.items():
        for dst in dsts:
            f.write(f"\t\"{src}\" -> \"{dst}\"\n")


    f.write("}")
