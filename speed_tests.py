from time import time
from humanize import naturaldelta
from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
import numpy as np
import pandas as pd
from glob import glob
from tqdm import tqdm
import os
from pympler import asizeof
from collections import Counter
from notipy_me import Notipy


def single_speed_test(directory: str):
    start = time()
    graph = EnsmallenGraph(
        edge_path=f"{directory}/edges.tsv",
        sources_column="subject",
        destinations_column="object",
        directed=True,
        edge_types_column="edge_label",
        node_path=f"{directory}/nodes.tsv",
        nodes_column="id",
        node_types_column="category",
        default_edge_type='biolink:interacts_with',
        default_node_type='biolink:NamedThing',
        validate_input_data=False
    )
    completed_graph = time() - start
    start_walk = time()
    walks = graph.walk(
        iterations=10,
        length=80,
        min_length=0,
        return_weight=1,
        explore_weight=1,
        change_node_type_weight=1,
        change_edge_type_weight=1
    )
    delta = time() - start
    total_walk_time = time() - start_walk

    walks_lengths = [
        len(walk) for walk in walks
    ]

    mean_walks_length = np.mean(walks_lengths)
    median_walks_length = np.median(walks_lengths)
    graph_memory_size = asizeof.asizeof(graph)
    walks_memory_size = asizeof.asizeof(walks)

    return {
        "total_required_time": delta,
        "building_graph_required_time": completed_graph,
        "random_walk_time": total_walk_time,
        "mean_walks_length": mean_walks_length,
        "median_walks_length": median_walks_length,
        "traps_rate": graph.traps_rate(),
        "graph_size": graph_memory_size,
        "walks_size": walks_memory_size,
        "nodes": graph.get_nodes_number(),
        "edges": graph.get_edges_number(),
        **Counter(walks_lengths)
    }


def speed_test(root: str, iterations: int = 1) -> pd.DataFrame:
    directories = [
        directory
        for directory in glob(f"{root}/*")
        if os.path.isdir(directory)
    ][:1]
    return pd.DataFrame([
        single_speed_test(directory)
        for directory in tqdm(directories, desc="Speed test")
        for _ in tqdm(iterations, desc="Iterations")
    ])


with Notipy(task_name="Ensmallen Graph speed test"):
    speed_test("../graphs").to_csv("speed_test_report.csv", index=False)
