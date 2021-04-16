import re
import sys
import json
from ensmallen_graph import EnsmallenGraph
from .utils import build_path

# 
FUNCTIONS_BLACKLIST = [
    "get_top_k_central_node_ids",
    "get_top_k_central_node_names",
    "get_unchecked_unique_source_node_id",
]

CAPTURE_WHITELIST = [
    "multigraph",
    "directed",
    "node",
    "nodes",
    "edge",
    "edges",
    "edge_count",
    "node_count",
    "trap_node",
    "trap_nodes",
    "edge_id",
    "edge_ids",
    "node_id",
    "node_ids",
    "edge_types",
    "edge_type_id",
    "edge_type_ids",
    "node_types",
    "node_type_id",
    "node_type_ids",
    "edge_type_name",
    "edge_type_names",
    "node_type_name",
    "node_type_names",
    "node_name",
    "node_names",
    "edge_weight",
    "edge_weights",
    "minmax_edge_id",
    "node_ids_and_type",
    "destination_node_id",
    "destination_node_ids",
    "source_node_id",
    "node_degree",
    "edge_degree",
    "minmax_edge_ids",
    "node_neighbour_names",
    "node_neighbour_ids",
    "singleton",
    "singletons",
    "singleton_with_selfloops",
    "singletons_with_selfloops",
    "selfloops",
    "multilabel_node_types",
    "unknown_node_types",
    "unknown_edge_types",
]

def extract_regex(doc):
    regexes = []
    for line in doc:
        matches = re.findall("\*\s+`([^`]+)`", line)
        if matches:
            regexes.append(matches[0])
    return regexes

def check_method_names(args):

    with open(build_path("results/analysis.json"), "r") as f:
        functions = json.load(f)

    result = {}

    for function in functions:
        function_name = function.get("name", "")
        if function_name in FUNCTIONS_BLACKLIST:
            continue

        regexes = extract_regex(function.get("impl_doc", []))

        if regexes and not any(
            re.match(regex, function_name) is not None
            for regex in regexes
        ):
            print("The method {} does not follow any of its regexes: {}".format(
                function_name, regexes
            ))
            continue

        for regex in regexes:
            matchs = re.match(regex, function_name)
            if matchs is not None:
                for group  in matchs.groups():

                    if group.startswith("unchecked"):
                        continue

                    for sub_grup in group.split("_and_"):
                        if sub_grup not in CAPTURE_WHITELIST:
                            result.setdefault(function_name, [])
                            result[function_name].append(sub_grup)
                            print("The method {} does not match the approved capture lists, the wrong value is {}".format(
                                function.get("name", ""), group
                            ))