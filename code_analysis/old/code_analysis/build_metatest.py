import re
import glob
import json
from .utils import get_file, build_path

DEBUG = True

HARNESS_TEMPLATE = get_file("templates/harness.txt")
STRUCT_TEMPLATE = get_file("templates/struct.txt")
META_STRUCT_TEMPLATE = get_file("templates/meta_struct.txt")

RESET_HOOK = """
let g_copy = graph.clone();
let trace2 = trace.clone();
std::panic::set_hook(Box::new(move |info| {
\thandle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
}));"""

BLACKLISTED_FUNCS = [
    "new",
]

SELFS = [
    "self",
    "&self",
    "&mut self",
    "mut self",
]

BLACKLISTED_TYPES = [
    "impl", 
    "Fn", 
    "Iter", 
    "NodeFileReader", 
    "EdgeFileReader", 
    "Graph", 
    "Compute_hash_Params",
    "&str",
    "&[u32]",
    "&String",
    "SingleWalkParameters",
    "WalksParameters",
    "WalkWeights",
    "Self",
    "&[String]",
]

def filter_function(function):
    if function.get("modifiers", "") != "pub":
        return False

    if function.get("name", "") in BLACKLISTED_FUNCS:
        return False

    if "unchecked" in function.get("name", ""):
        return False

    if function.get("struct", "") != "Graph":
        return False

    args = function.get("args", [])
    if len(args) == 0:
        return False

    if any(
            e in arg[1]
            for arg in args
            for e in BLACKLISTED_TYPES
        ):
        return False
    return True   

def build_struct_and_call(function):
    args = function.get("args")
    function_name = function.get("name")

    struct_type = "".join(x.capitalize() for x in function_name.split("_")) + "Params"
    
    fields = "\n".join([
            "\tpub {field_name} : {field_type},".format(
                field_name=arg_name,
                field_type=(arg_type if arg_type != "S" else "String").lstrip("&"),
            )
        for arg_name, arg_type in args
        if arg_type not in SELFS
    ])

    call_args = []
    for arg in args:
        if arg[0] in SELFS or arg[1] in SELFS:
            continue

        res = f"data_for_current_test.{function_name}.{arg[0]}"

        if arg[1][0] == "&":
            res = "&" + res

        call_args.append(res)

    args = ", ".join(call_args)
    
    call = f"graph.{function_name}({args})"

    return_type = function.get("return_type", "")
    if  return_type.startswith("Result") and "&" not in function.get("args")[0][1]:
        call = "graph = " + call + "?"
    elif return_type.startswith("Result"):
        call = "let _ = " + call 
    elif "Iterator" in return_type:
        call = "let _ = " + call + ".collect::<Vec<_>>()"
    elif "&" not in function.get("args")[0][1]:
        call = "graph = " + call
    

    result = {
        "function_name":function.get("name"),
        "struct_name":function_name,
        "struct_type":struct_type,
        "call":call + ";",
        "call_args":call_args,
        "args":function.get("args"),
    }

    if len(args) > 1 :
        struct = STRUCT_TEMPLATE.format(
            struct_name=struct_type,
            fields=fields,
        )
        result["struct"] = struct

    return result

def build_metatest(args):
    with open(build_path("results/analysis.json"), "r") as f:
        analysis = json.load(f)

    result = [
        build_struct_and_call(function)
        for results in analysis.values()
        for function in results["functions"]
        if filter_function(function)
    ]

    structs = [
        res["struct"]
        for res in result
        if "struct" in res
    ]
    calls = []
    for i, res in enumerate(result):
        output = "\t\t\t{i} => {{\n".format(i=i)
        if DEBUG:
            output += "\t\t\t\ttrace.push(format!(\"{function_name}({kwargs_format})\", {call_args}));\n".format(
                function_name=res["function_name"],
                call_args=", ".join(res["call_args"]),    
                kwargs_format=", ".join("{arg_name} = {{:?}}".format(arg_name=arg[0]) for arg in res["args"][1:]),
            )
            output += "\n".join("\t\t\t\t" + x for x in RESET_HOOK.split("\n"))

        output += "\n\t\t\t\t" + res["call"]
        output += "\n\t\t\t},"
        calls.append(output)

    calls.append(
        "\t\t{i} => {{let _ = graph::test_utilities::default_test_suite(&mut graph, false);}}".format(i=len(calls))
    )
    

    params = "\n".join(
        "\tpub {struct_name}: {struct_type},".format(
            struct_name=res["struct_name"],
            struct_type=res["struct_type"],
        )
        for res in result
        if "struct" in res
    )

    meta_struct = META_STRUCT_TEMPLATE.format(
        params=params
    )

    output = HARNESS_TEMPLATE.format(
        calls="\n".join(calls),
        n_of_calls=len(calls),
        structs="\n".join(structs),
        meta_struct=meta_struct,
    )

    print("Generated a test with {} methods".format(len(calls)))

    with open(build_path("../../../fuzzing/graph_harness/src/meta_test.rs"), "w") as f:
        f.write(output)