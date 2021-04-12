import json

with open("./analysis.json", "r") as f:
    functions = json.load(f)

def filter_function(x):
    if "name" not in x:
        return False 
    
    if x.get("struct", "") in [
        "EdgeTypeVocabulary",
        "NodeTypeVocabulary",
        "NodeFileWriter",
        "NodeFileReader",
        "EdgeFileWriter",
        "EdgeFileReader",
        "CSVFileReader",
    ]:
        return False
    
    if x["name"] in [
        "len",
        "clone",
        "get",
        "insert",
        "not_one",
        "contains_key",
    ]:
        return False
    
    if x["name"] == "new" and x.get("struct", "") != "Graph":
        return False
    
    return True

functions = [
    x
    for x in functions
    if filter_function(x)
]

def get_canonical_name(x):
    name = x["name"]
    if "struct" in x:
        name = x["struct"] + "::" + name
    return name

result = {}
for function in functions:
    if not "name" in function:
        continue

    result[get_canonical_name(function)] = [
        value
        for value in functions
        if value["name"] + "(" in function.get("body", "")
    ]

with open("dependancies.json", "w") as f:
    json.dump({
        name: [
            get_canonical_name(x)
            for x in deps
        ]
        for name, deps in result.items()
    }, f)

with open("dependancies.dot", "w") as f:
    f.write("digraph deps {\n")
    f.write("\tnode [shape=box];\n")

    f.write("\n")

    for sub in result.keys():
        f.write(f"\t\"{sub}\"\n")

    f.write("\n")

    for src, dsts in result.items():
        for dst_vals in dsts:
            dst = get_canonical_name(dst_vals)
            f.write(f"\t\"{src}\" -> \"{dst}\"\n")


    f.write("}")
