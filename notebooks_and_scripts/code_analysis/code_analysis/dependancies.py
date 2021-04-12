import json
from .utils import build_path, get_canonical_name, filter_function, to_dot

def dependancies(args):
    with open(build_path("results/analysis.json"), "r") as f:
        functions = json.load(f)

    functions = [
        x
        for x in functions
        if filter_function(x)
    ]


    result = {}
    for function in functions:
        if not "name" in function:
            continue

        result[get_canonical_name(function)] = [
            value
            for value in functions
            if value["name"] + "(" in function.get("body", "")
        ]

    with open(build_path("results/dependancies.json"), "w") as f:
        json.dump({
            name: [
                get_canonical_name(x)
                for x in deps
            ]
            for name, deps in result.items()
        }, f)

    with open(build_path("results/dependancies.dot"), "w") as f:
        f.write(to_dot(result.keys(), [
            (src, get_canonical_name(dst_vals))
            for src, dsts in result.items()
            for dst_vals in dsts
        ]))
