import os

ROOT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")

def build_path(path):
    return os.path.join(ROOT_DIR, path)

def get_canonical_name(x):
    name = x["name"]
    if "struct" in x:
        name = x["struct"] + "::" + name
    return name

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

def to_dot(nodes, edges):
    result = ""
    result += "digraph deps {\n"
    result += "\tnode [shape=box];\n"

    result += "\n"

    for sub in nodes:
        if isinstance(sub, str):
            result += f"\t\"{sub}\"\n"
        else:
            result += f"\t\"{sub[0]}\" [color={sub[1]}];\n"

    result += "\n"

    for src, dst in edges:
        result += f"\t\"{src}\" -> \"{dst}\"\n"

    result += "}"
    return result
