from time import time
from humanize import naturaldelta
from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
import compress_json
import json
import numpy as np


start = time()
graph = EnsmallenGraph(
    edge_path="../graph/cooccurrence/edges.tsv",
    sources_column="subject",
    destinations_column="object",
    directed=False,
    validate_input_data=True
)
completed_graph = time() - start
start_walk = time()

walks = graph.walk(
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

mean_walks_length = np.mean([
    len(walk) for walk in walks
])

median_walks_length = np.median([
    len(walk) for walk in walks
])

response = {
    "required_time": delta,
    "human_time": naturaldelta(delta),
    "building_graph_required_time": completed_graph,
    "building_graph_required_human_time": naturaldelta(completed_graph),
    "random_walk_time": total_walk_time,
    "random_walk_human_time": naturaldelta(total_walk_time),
    "mean_walks_length": mean_walks_length,
    "median_walks_length": median_walks_length,
}

print(json.dumps(response, indent=4))

compress_json.dump(response, "time_required.json", json_kwargs={"indent": 4})
