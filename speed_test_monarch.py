from time import time
from humanize import naturaldelta
from ensmallen_graph import EnsmallenGraph # pylint: disable=no-name-in-module
import compress_json
import json


start = time()
graph = EnsmallenGraph(
    edge_path="../graph/new_monarch_edges.tsv",
    sources_column="subject",
    destinations_column="object",
    directed=True,
    edge_types_column="edge_label",
    default_edge_type='biolink:interacts_with',
    validate_input_data=False
)
completed_graph = time() - start
start_walk = time()
graph.walk(
    iterations=1,
    length=80,
    min_length=0,
    return_weight=1,
    explore_weight=1,
    change_node_type_weight=1,
    change_edge_type_weight=1
)
delta = time() - start
total_walk_time = time() - start_walk

response = {
    "required_time": delta,
    "human_time": naturaldelta(delta),
    "building_graph_required_time": completed_graph,
    "building_graph_required_human_time": naturaldelta(completed_graph),
    "random_walk_time": total_walk_time,
    "random_walk_human_time": naturaldelta(total_walk_time)
}

print(json.dumps(response, indent=4))

compress_json.dump(response, "time_required.json", json_kwargs={"indent": 4})
