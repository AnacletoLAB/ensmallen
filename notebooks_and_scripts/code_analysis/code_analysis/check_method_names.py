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
    "name",
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
    "trap_nodes_number",
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
    "edge_types_number",
    "node_type_name",
    "node_type_names",
    "node_types_number",
    "node_name",
    "node_names",
    "edge_node_names",
    "edge_node_ids",
    "unique_edge_node_ids",
    "unique_source_node_ids",
    "edge_weight",
    "edge_weights",
    "min_edge_weight",
    "max_edge_weight",
    "minmax_edge_id",
    "node_ids_and_type",
    "sources",
    "source_names",
    "destination",
    "destinations",
    "destination_name",
    "destination_names",
    "destination_node_id",
    "destination_node_ids",
    "source_node_id",
    "source_node_ids",
    "node_degree",
    "node_degrees",
    "edge_degree",
    "minmax_edge_ids",
    "neighbour_node_names",
    "neighbour_node_ids",
    "singleton",
    "singletons",
    "singleton_node_ids",
    "not_singletons_node_ids",
    "singleton_with_selfloops",
    "singletons_with_selfloops",
    "singleton_with_selfloops_node_ids",
    "selfloops",
    "multilabel_node_types",
    "unknown_node_types",
    "unknown_edge_types",
    "unknown_node_types_number",
    "minimum_node_types_number",
    "unknown_edge_types_number",
    "minimum_edge_types_number",
    "directed_edges_number",
    "nodes_number",
    "unique_directed_edges_number",
    "nodes_mapping",
    "dense_nodes_mapping",
    "multigraph_edges_number",
    "unique_source_nodes_number",
    "edge_type_counter",
    "edge_type_counts_hashmap",
    "node_type_counter",
    "node_type_counts_hashmap",
    "cumulative_node_degrees",
    "node_connected_component_ids",
    "non_singleton_node_ids",
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
                groups = matchs.groups()
                # WHY PYTHON WHY
                if isinstance(groups, str):
                    groups = [groups]

                for group in groups:
                    if group.startswith("unchecked"):
                        continue

                    for sub_grup in group.split("_and_"):
                        if sub_grup not in CAPTURE_WHITELIST:
                            result.setdefault(function_name, [])
                            result[function_name].append(sub_grup)
                            print("The method {} does not match the approved capture lists, the wrong value is {}".format(
                                function.get("name", ""), sub_grup
                            ))